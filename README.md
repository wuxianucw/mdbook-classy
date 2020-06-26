# mdbook-classy

## What is this?

This is a focused markdown preprocessor for [mdbook](https://crates.io/crates/mdbook) that makes it simple css classes to your markdown paragraphs.

It uses kramdown-style class annotation, changeing this

```markdown
{:.class-name}
This is a *grand* textual paragraph. Truly **grand**!
```

to this:

```
<div class="class-name">

This is a *grand* textual paragraph. Truly **grand**!

</div>
```

## Motifivation

mdbook-classy lets you easily define new stylistic element types for your book.  
Give them a name and define the style for the element in css and you're on your way!

## Installation

To install mdbook-classy, use cargo:

```
cargo install mdbook-classy
```

Then add the following to `book.toml`:

```
[preprocessor.classy]
```
