use clap::{arg, command, ArgMatches, Command};
use mdbook::book::{Book, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor, PreprocessorContext};
use mdbook::utils::new_cmark_parser;
use pulldown_cmark::{CowStr, Event, Tag};
use std::io;
use std::process;

#[derive(Default)]
pub struct Classy;

impl Classy {
    pub fn new() -> Classy {
        Classy
    }
}

impl Preprocessor for Classy {
    fn name(&self) -> &str {
        "classy"
    }
    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|book| {
            if let mdbook::BookItem::Chapter(chapter) = book {
                if let Err(e) = classy(chapter) {
                    eprintln!("classy error: {:?}", e);
                }
            }
        });
        Ok(book)
    }
    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

#[derive(Debug, Clone, Copy)]
enum State<'a> {
    BeforeStart,
    Expecting,
    Accepted(&'a str),
    Inner,
}

/// This is where the markdown transformation actually happens.
/// Take paragraphs beginning with `{:.class-name}` and give them special rendering.
/// Mutation: the payload here is that it edits chapter.content.
fn classy(chapter: &mut Chapter) -> Result<(), Error> {
    let parser = new_cmark_parser(&chapter.content, false);

    let mut state = State::BeforeStart;
    let mut new_events = vec![];

    for event in parser {
        if let State::Accepted(text) = state {
            if matches!(event, Event::SoftBreak | Event::HardBreak) {
                state = State::Inner;
                new_events.push(Event::Start(Tag::Paragraph));
            } else {
                state = State::BeforeStart;
                new_events.pop().unwrap();
                new_events.push(Event::Start(Tag::Paragraph));
                new_events.push(Event::Text(text.into()));
                new_events.push(event);
            }
            continue;
        }

        match event {
            Event::Start(Tag::Paragraph) => {
                state = State::Expecting;
            }

            Event::Text(CowStr::Borrowed(text)) if matches!(state, State::Expecting) => {
                // "{:.class}", "{:.class1 class2}", ...
                // event sequence: Start(Paragraph) -> Text(Borrowed(_)) -> SoftBreak | HardBreak
                if text.len() > "{:.}".len() && text.starts_with("{:.") && text.ends_with('}') {
                    state = State::Accepted(text);
                    new_events.push(Event::Html(
                        format!("<div class=\"{}\">\n", &text[3..text.len() - 1]).into(),
                    ));
                } else {
                    state = State::BeforeStart;
                    new_events.push(Event::Start(Tag::Paragraph));
                    new_events.push(Event::Text(text.into()));
                }
            }

            Event::End(Tag::Paragraph) => {
                if matches!(state, State::Expecting) {
                    new_events.push(Event::Start(Tag::Paragraph));
                }
                new_events.push(Event::End(Tag::Paragraph));
                if matches!(state, State::Inner) {
                    new_events.push(Event::Html("</div>\n".into()));
                }
                state = State::BeforeStart;
            }

            ev => {
                if matches!(state, State::Expecting) {
                    state = State::BeforeStart;
                    new_events.push(Event::Start(Tag::Paragraph));
                }
                new_events.push(ev);
            }
        }
    }

    let mut buf = String::with_capacity(chapter.content.capacity());
    pulldown_cmark_to_cmark::cmark(new_events.into_iter(), &mut buf).expect("can re-render cmark");
    chapter.content = buf;
    Ok(())
}

/// Housekeeping:
/// 1. Check compatibility between preprocessor and mdbook
/// 2. deserialize, run the transformation, and reserialize.
fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

/// Check to see if we support the processor (classy only supports html right now)
fn handle_supports(pre: &dyn Preprocessor, sub_args: &ArgMatches) -> ! {
    let renderer = sub_args
        .get_one::<String>("renderer")
        .expect("Required argument");
    let supported = pre.supports_renderer(renderer);

    if supported {
        process::exit(0);
    }
    process::exit(1);
}

fn main() {
    // 1. Define command interface, requiring renderer to be specified.
    let matches = command!("classy")
        .about("A mdbook preprocessor that recognizes kramdown style paragraph class annotation.")
        .subcommand(
            Command::new("supports")
                .arg(arg!(<renderer>).required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
        .get_matches();

    // 2. Instantiate the preprocessor.
    let preprocessor = Classy::new();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(&preprocessor, sub_args);
    }
    if let Err(e) = handle_preprocessing(&preprocessor) {
        eprintln!("{}", e);
        process::exit(1);
    }
}
