use super::engine::MarkupEngine;

pub struct MarkdownEngine;

impl MarkupEngine for MarkdownEngine {
    fn header(&self, level: u8, text: &str) -> String {
        format!("{} {}", "#".repeat(level as usize), text)
    }

    fn paragraph(&self, text: &str) -> String {
        format!("{}\n", text)
    }

    fn italic(&self, text: &str) -> String {
        format!("*{}*", text)
    }

    fn bold(&self, text: &str) -> String {
        format!("**{}**", text)
    }

    fn link(&self, text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }

    fn linebreak(&self) -> String {
        format!("\n")
    }

    fn line_segment(&self, text: &str) -> String {
        text.to_string()
    }

    fn non_breaking_space(&self) -> String {
        " ".to_string()
    }
    
    fn pre_block(&self, text: &str) -> String {
        format!("```\n{}\n```", text)
    }
}
