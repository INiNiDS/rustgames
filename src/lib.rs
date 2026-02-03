mod prelude;
pub mod core;
pub mod graphics;
pub mod text;
pub mod window;
mod controllers;

#[cfg(test)]
mod tests {
    use crate::text::{RichTextParser, TextWrapper};
    
    #[test]
    fn test_text_wrapping() {
        let text = "Hello world this is a test";
        let lines = TextWrapper::wrap_text(text, 50.0, 10.0);
        assert!(!lines.is_empty());
    }
    
    #[test]
    fn test_rich_text_parsing() {
        let segments = RichTextParser::parse("Hello [b]world[/b]!");
        assert_eq!(segments.len(), 3);
    }
}
