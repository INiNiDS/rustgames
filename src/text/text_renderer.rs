use crate::graphics::color::Color;
use crate::text::{FontWeight, TextAttributes};

#[derive(Debug, Clone)]
pub struct StyledSegment {
    pub text: String,
    pub attrs: TextAttributes,
}

pub struct RichTextParser;

struct ParseState {
    segments: Vec<StyledSegment>,
    current_text: String,
    weight_stack: Vec<FontWeight>,
    italic_stack: Vec<bool>,
    color_stack: Vec<Option<Color>>,
}

impl ParseState {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
            current_text: String::new(),
            weight_stack: vec![FontWeight::Normal],
            italic_stack: vec![false],
            color_stack: vec![None],
        }
    }

    fn flush_segment(&mut self) {
        if self.current_text.is_empty() {
            return;
        }
        self.segments.push(StyledSegment {
            text: std::mem::take(&mut self.current_text),
            attrs: TextAttributes {
                weight: *self.weight_stack.last().unwrap(),
                italic: *self.italic_stack.last().unwrap(),
                underline: false,
                strikethrough: false,
                color: *self.color_stack.last().unwrap(),
            },
        });
    }

    fn apply_open_tag(&mut self, tag: &str) {
        match tag {
            "b" => self.weight_stack.push(FontWeight::Bold),
            "m" => self.weight_stack.push(FontWeight::Medium),
            "sb" => self.weight_stack.push(FontWeight::SemiBold),
            "i" => self.italic_stack.push(true),
            _ if tag.starts_with("color=") => {
                self.color_stack.push(Color::from_hex(&tag[6..]));
            }
            _ => {}
        }
    }

    fn apply_close_tag(&mut self, tag: &str) {
        match tag {
            "b" | "m" | "sb" => {
                if self.weight_stack.len() > 1 {
                    self.weight_stack.pop();
                }
            }
            "i" => {
                if self.italic_stack.len() > 1 {
                    self.italic_stack.pop();
                }
            }
            "color" => {
                if self.color_stack.len() > 1 {
                    self.color_stack.pop();
                }
            }
            _ => {}
        }
    }
}

impl RichTextParser {
    #[must_use]
    pub fn parse(text: &str) -> Vec<StyledSegment> {
        let mut state = ParseState::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c != '[' {
                state.current_text.push(c);
                continue;
            }
            Self::process_tag(&mut state, &mut chars);
        }

        state.flush_segment();
        state.segments
    }

    fn process_tag(
        state: &mut ParseState,
        chars: &mut std::iter::Peekable<std::str::Chars<'_>>,
    ) {
        let is_closing = chars.peek() == Some(&'/');
        if is_closing {
            chars.next();
        }

        let tag_content = Self::read_tag_content(chars);
        state.flush_segment();

        if is_closing {
            state.apply_close_tag(&tag_content);
        } else {
            state.apply_open_tag(&tag_content);
        }
    }

    fn read_tag_content(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
        let mut tag_content = String::new();
        while let Some(&next_char) = chars.peek() {
            if next_char == ']' {
                chars.next();
                break;
            }
            tag_content.push(chars.next().unwrap());
        }
        tag_content
    }
}
