# mdbook-classy example

<style>
    .big{font-size: 2em;}
    .blackbackground{background: black;}
    .little{font-size: .75em}
    .orange{color: orange;}
    .padded{padding: 2em;}
    .red{color: red;}
</style>

You can set a style in your markdown or in a theme or style sheet.  Here we've done it within the markdown file by adding the following:

```html
<style>
    .big{font-size: 2em;}
    .blackbackground{background: black;}
    .little{font-size: .75em}
    .orange{color: orange;}
    .padded{height: 2em;}
    .red{color: red;}
</style>
```

## red

Do this

```markdown
{:.red big}
You can write **Markdown** paragraphs with red text anytime you want.
```

to get this:

{:.red big}
You can write **Markdown** paragraphs with red text anytime you want.

## orange

Or this

```markdown
{:.orange blackbackground little padded}
Orange works too!
```

to get this:

{:.orange blackbackground little padded}
Orange ~~is broken~~ **works** too!
