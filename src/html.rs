use super::engine::MarkupEngine;

pub struct HtmlEngine;

impl MarkupEngine for HtmlEngine {
    fn header(&self, level: u8, text: &str) -> String {
        format!("<h{lvl}>{}</h{lvl}>\n", text, lvl=level)
    }
    fn paragraph(&self, text: &str) -> String {
        format!("<p>{}</p>\n", text)
    }
    fn italic(&self, text: &str) -> String {
        format!("<i>{}</i>", text)
    }
    fn bold(&self, text: &str) -> String {
        format!("<b>{}</b>", text)
    }
    fn link(&self, text: &str, url: &str) -> String {
        format!("<a href=\"{}\">{}</a>", url, text)
    }
    fn linebreak(&self) -> String {
        "<br>\n".to_string()
    }
    fn line_segment(&self, text:&str) -> String {
        text.to_string()
    }
}