# Markup Engine
`markup_engine` is a minimal, pluggable Rust trait for rendering structured content into various output formats like Markdown, HTML, or custom markup languages. It was originally developed to power LeadSheetML, but is fully general and can be used in any context that requires formatting content for multiple output formats.
# Features 
Define a common interface for rendering:
  - Titles 
  - Headers (various levels)
  - Paragraphs 
  - Italic, bold, and other inline formatting 
  - Line breaks

Write one rendering logic, and plug in multiple output styles
Easily extended for any other output format

### Usage
Add to your Cargo.toml (Local Dev)
If you’re developing locally:
```
markup_engine = { path = "../markup_engine" }
```

Or from GitHub:
```
markup_engine = { git = "https://github.com/yourname/markup_engine", branch = "main" }
```

Eventually:
```
markup_engine = "0.1"
```

### Example

```rust
use markup_engine::{MarkupEngine, MarkdownEngine, HtmlEngine};

fn render_example<T: MarkupEngine>(engine: &T) -> String {
    [
        engine.header1("Welcome to MarkupEngine"),
        engine.italic("Pluggable rendering in Rust"),
        engine.line_break(),
        engine.header2("Features"),
        engine.paragraph("• Clean trait-based design\n• Output to multiple formats\n• Easy to extend"),
    ]
    .join("\n")
}


fn main() {
    let md = MarkdownEngine;
    println!("{}", render_example(&md));

    let html = HtmlEngine;
    println!("{}", render_example(&html));
}
```

### Implementations
- MarkdownEngine — renders markdown-formatted output
- HtmlEngine — renders HTML-formatted output


### Philosophy

The goal of markup_engine is to separate rendering logic from content structure, making it easier to write one logic for rendering and plug in any output format you need.

### Contributing
Open to contributions, suggestions, and new format implementations! PRs welcome.