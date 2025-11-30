pub trait MarkupEngine {
    fn header(&self, level: u8, text: &str) -> String;
    fn paragraph(&self, text: &str) -> String;
    fn italic(&self, text: &str) -> String;
    fn bold(&self, text: &str) -> String;
    fn link(&self, text: &str, url: &str) -> String;
    fn linebreak(&self) -> String;
    fn line_segment(&self, text: &str) -> String;
    fn non_breaking_space(&self) -> String;

    fn pre_block(&self, text: &str) -> String;
}
