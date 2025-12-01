use crate::engine::MarkupEngine;

pub struct HtmlEngine;
impl MarkupEngine for HtmlEngine {
    fn header(&self, level: u8, text: &str) -> String {
        let text = escape_html(text);
        format!("<h{lvl}>{}</h{lvl}>\n", text, lvl = level)
    }

    fn paragraph(&self, text: &str) -> String {
        format!("<p>{}</p>\n", escape_html(text))
    }

    fn italic(&self, text: &str) -> String {
        format!("<i>{}</i>", escape_html(text))
    }

    fn bold(&self, text: &str) -> String {
        format!("<b>{}</b>", escape_html(text))
    }

    fn link(&self, text: &str, url: &str) -> String {
        format!(
            "<a href=\"{}\">{}</a>",
            escape_html(url),
            escape_html(text),
        )
    }

    fn line_segment(&self, text: &str) -> String {
        escape_html(text)
    }

    fn pre_block(&self, text: &str) -> String {
        format!("<pre>{}</pre>\n", escape_html(text))
    }

    fn non_breaking_space(&self) -> String {
        "&nbsp;".to_string()
    }

    fn linebreak(&self) -> String {
        "<br/>".to_string()
    }

    fn list(&self, ordered: bool, items: &[&str]) -> String {
        let mut out = String::new();

        if ordered {
            out.push_str("<ol>\n");
        } else {
            out.push_str("<ul>\n");
        }

        for item in items {
            out.push_str(&format!("  <li>{}</li>\n", item));
        }

        if ordered {
            out.push_str("</ol>\n");
        } else {
            out.push_str("</ul>\n");
        }

        out
    }
}

fn escape_html(text: &str) -> String {
    text.chars().map(|c| match c {
        '&' => "&amp;".to_string(),
        '<' => "&lt;".to_string(),
        '>' => "&gt;".to_string(),
        '"' => "&quot;".to_string(),
        '\'' => "&#39;".to_string(),
        _ => c.to_string(),
    }).collect()
}
