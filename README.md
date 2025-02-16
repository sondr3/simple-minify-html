<h1 align="center">simple-minify-html</h1>
<p align="center">
    <a href="https://github.com/sondr3/simple-minify-html/actions"><img alt="GitHub Actions Status" src="https://github.com/sondr3/simple-minify-html/workflows/pipeline/badge.svg" /></a>
    <a href="https://crates.io/crates/simple-minify-html"><img alt="Crates" src="https://img.shields.io/crates/v/simple-minify-html.svg" /></a>
</p>

> [!CAUTION]
> This is a small fork of [wilsonlin][wilson]'s great [minify-html][fork], you
> should probably use that instead.

Changes include, but are not limited to:

- No support for other platforms than Rust
- Uses [OXC](https://oxc.rs/) for JavaScript minification over [minify-js](https://github.com/wilsonzlin/minify-js)
- Does not support templating syntax
- Less configuration options

View the [changelog](./CHANGELOG.md) to see the latest updates.

# Get

```toml
[dependencies]
simple-minify-html = "0.15.0"
```

# Use

Check out the [docs](https://docs.rs/simple-minify-html) for API and usage examples.

## Features

### JavaScript minification

To enable minification of JavaScript, enable the `js` feature and this will
automatically be handled:

```toml
[dependencies]
simple-minify-html = { version = "0.15.0", features = ["js"] }
```

### CSS minification

To enable minification of JavaScript, enable the `js` feature and this will
automatically be handled:

```toml
[dependencies]
simple-minify-html = { version = "0.15.0", features = ["css"] }
```

## Minification

### Spec compliance

WHATWG is the current HTML standard and [obsoletes all previous standards](https://www.w3.org/html/). WHATWG lists
suggested validators [here](https://whatwg.org/validator/).

To minify even further, it's possible to enable options that may output HTML that doesn't fully pass validation, but is
still interpreted and rendered correctly according to
the [WHATWG parsing specification](https://html.spec.whatwg.org/multipage/parsing.html), which major browser engines (
Firefox, Chrome, Safari) implement. Refer to these options:

- `allow_noncompliant_unquoted_attribute_values`
- `allow_optimal_entities`
- `allow_removing_spaces_between_attributes`
- `minify_doctype`

In Rust, `Cfg::enable_possibly_noncompliant` can enable all of these at once.

### Whitespace

minify-html has advanced context-aware whitespace minification that does things such as:

- Leave whitespace untouched in `pre` and `code`, which are whitespace sensitive.
- Trim and collapse whitespace in content tags, as whitespace is collapsed anyway when rendered.
- Remove whitespace in layout tags, which allows the use of inline layouts while keeping formatted code.

#### Methods

There are three whitespace minification methods. When processing text content, minify-html chooses which ones to use
depending on the containing element.

<details>
<summary><strong>Collapse whitespace</strong></summary>

> **Applies to:** any element except [whitespace sensitive](./minify-html-common/src/spec/tag/whitespace.rs) elements.

Reduce a sequence of whitespace characters in text nodes to a single space (U+0020).

<table><thead><tr><th>Before<th>After<tbody><tr><td>

```html
<p>↵
    ··The·quick·brown·fox↵
    ··jumps·over·the·lazy↵
    ··dog.↵
</p>
```

<td>

```html
<p>·The·quick·brown·fox·jumps·over·the·lazy·dog.·</p>
```

</table>
</details>

<details>
<summary><strong>Destroy whole whitespace</strong></summary>

> **Applies to:** any element
>
except [whitespace sensitive](./minify-html-common/src/spec/tag/whitespace.rs), [content](src/spec/tag/whitespace.rs), [content-first](./minify-html-common/src/spec/tag/whitespace.rs),
> and [formatting](./minify-html-common/src/spec/tag/whitespace.rs) elements.

Remove any text nodes between tags that only consist of whitespace characters.

<table><thead><tr><th>Before<th>After<tbody><tr><td>

```html

<ul>↵
    ··
    <li>A</li>
    ↵
    ··
    <li>B</li>
    ↵
    ··
    <li>C</li>
    ↵
</ul>
```

<td>

```html

<ul>↵
    ··
    <li>A</li>
    <li>B</li>
    <li>C</li>
    ↵
</ul>
```

</table>
</details>

<details>
<summary><strong>Trim whitespace</strong></summary>

> **Applies to:** any element except [whitespace sensitive](./minify-html-common/src/spec/tag/whitespace.rs)
> and [formatting](./minify-html-common/src/spec/tag/whitespace.rs) elements.

Remove any leading/trailing whitespace from any leading/trailing text nodes of a tag.

<table><thead><tr><th>Before<th>After<tbody><tr><td>

```html
<p>↵
    ··Hey,·I·<em>just</em>·found↵
    ··out·about·this·<strong>cool</strong>·website!↵
    ··<sup>[1]</sup>↵
</p>
```

<td>

```html
<p>Hey,·I·<em>just</em>·found↵
    ··out·about·this·<strong>cool</strong>·website!↵
    ··<sup>[1]</sup></p>
```

</table>
</details>

#### Element types

minify-html assumes HTML and SVG elements are used in specific ways, based on standards and best practices. By making
these assumptions, it can apply optimal whitespace minification strategies. If these assumptions do not hold, consider
adjusting the HTML source or turning off whitespace minification.

| Group         | Elements                                                                     | Expected children                                     |
|---------------|------------------------------------------------------------------------------|-------------------------------------------------------|
| Formatting    | `a`, `strong`, [and others](./minify-html-common/src/spec/tag/whitespace.rs) | Formatting elements, text.                            |
| Content       | `h1`, `p`, [and others](./minify-html-common/src/spec/tag/whitespace.rs)     | Formatting elements, text.                            |
| Layout        | `div`, `ul`, [and others](./minify-html-common/src/spec/tag/whitespace.rs)   | Layout elements, content elements.                    |
| Content-first | `label`, `li`, [and others](./minify-html-common/src/spec/tag/whitespace.rs) | Like content but could be layout with only one child. |

<details>
<summary><strong>Formatting elements</strong></summary>

> Whitespace is collapsed.

Formatting elements are usually inline elements that wrap around part of some text in a content element, so its
whitespace isn't trimmed as they're probably part of the content.

</details>

<details>
<summary><strong>Content elements</strong></summary>

> Whitespace is trimmed and collapsed.

Content elements usually represent a contiguous and complete unit of content such as a paragraph. As such, whitespace is
significant but sequences of them are most likely due to formatting.

###### Before

```html
<p>↵
    ··Hey,·I·<em>just</em>·found↵
    ··out·about·this·<strong>cool</strong>·website!↵
    ··<sup>[1]</sup>↵
</p>
```

###### After

```html
<p>Hey,·I·<em>just</em>·found·out·about·this·<strong>cool</strong>·website!·<sup>[1]</sup></p>
```

</details>

<details>
<summary><strong>Layout elements</strong></summary>

> Whitespace is trimmed and collapsed. Whole whitespace is removed.

These elements should only contain other elements and no text. This makes it possible to remove whole whitespace, which
is useful when using `display: inline-block` so that whitespace between elements (e.g. indentation) does not alter
layout and styling.

###### Before

```html

<ul>↵
    ··
    <li>A</li>
    ↵
    ··
    <li>B</li>
    ↵
    ··
    <li>C</li>
    ↵
</ul>
```

###### After

```html

<ul>
    <li>A</li>
    <li>B</li>
    <li>C</li>
</ul>
```

</details>

<details>
<summary><strong>Content-first elements</strong></summary>

> Whitespace is trimmed and collapsed.

These elements are usually like content elements but are occasionally used like a layout element with one child. Whole
whitespace is not removed as it might contain content, but this is OK for using as layout as there is only one child and
whitespace is trimmed.

###### Before

```html

<li>↵
    ··
    <article>↵
        ····
        <section></section>
        ↵
        ····
        <section></section>
        ↵
        ··
    </article>
    ↵
</li>
```

###### After

```html

<li>
    <article>
        <section></section>
        <section></section>
    </article>
</li>
```

</details>

### Tags

[Optional opening and closing tags](https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-omission) are removed.

### Attributes

Any entities in attribute values are decoded, and then the shortest representation of the value is calculated and used:

- Double quoted, with any `"` encoded.
- Single quoted, with any `'` encoded.
- Unquoted, with `"`/`'` first character (if applicable), any `>`, and any whitespace encoded.

Attributes have their whitespace (after any decoding) trimmed and collapsed when possible.

[Boolean attribute](https://github.com/wilsonzlin/html-data) values are removed.
[Some other attributes](https://github.com/wilsonzlin/html-data) are completely removed if their value is empty or the
default value after any processing.

`type` attributes on `script` tags with a value equaling
a [JavaScript MIME type](https://mimesniff.spec.whatwg.org/#javascript-mime-type) are removed.

If an attribute value is empty after any processing, everything but the name is completely removed (i.e. no `=`), as an
empty attribute is implicitly [the same](https://html.spec.whatwg.org/multipage/syntax.html#attributes-2) as an
attribute with an empty string value.

Spaces are removed between attributes when possible.

### Entities

Entities are decoded if they're valid and shorter or equal in length when decoded. UTF-8 sequences that have a shorter
entity representation are encoded.

Numeric entities that do not refer to a
valid [Unicode Scalar Value](https://www.unicode.org/glossary/#unicode_scalar_value) are replaced with
the [replacement character](https://en.wikipedia.org/wiki/Specials_(Unicode_block)#Replacement_character).

Encoding is avoided when possible; for example, `<` are only encoded in content if they are followed by a valid tag name
character.
If necessary, the shortest entity representation is chosen.

### Comments

Comments are removed.

### Ignored

Bangs, [processing instructions](https://en.wikipedia.org/wiki/Processing_Instruction), and empty elements are not
removed as it is assumed there is a special reason for their declaration.

## Parsing

minify-html can process any HTML, handling all possible syntax (including invalid ones) gracefully like browsers.
See [Parsing.md](./notes/Parsing.md) for more details.

## Issues and contributions

Pull requests and any contributions welcome!

If minify-html did something unexpected, misunderstood some syntax, or incorrectly kept/removed some
code, [raise an issue](https://github.com/sondr3/simple-minify-html/issues) with some relevant code that can be used to
reproduce and investigate the issue.

[wilson]: https://github.com/wilsonzlin

[fork]: https://github.com/wilsonzlin/minify-html