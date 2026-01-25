use crate::graphics::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone)]
pub struct TextStyle {
    pub size: f32,
    pub color: Color,
    pub alignment: TextAlignment,
    pub vertical_alignment: VerticalAlignment,
    pub line_spacing: f32,
}

impl Default for TextStyle {
    fn default() -> Self {
        Self {
            size: 16.0,
            color: Color::WHITE,
            alignment: TextAlignment::Left,
            vertical_alignment: VerticalAlignment::Top,
            line_spacing: 1.2,
        }
    }
}

impl TextStyle {
    pub fn new(size: f32) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }
    
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
    
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Self {
        self.alignment = alignment;
        self
    }
}

pub struct TextWrapper;

impl TextWrapper {
    pub fn wrap_text(text: &str, max_width: f32, char_width: f32) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();
        let mut current_width = 0.0;
        
        for word in text.split_whitespace() {
            let word_width = word.len() as f32 * char_width;
            
            if current_width + word_width <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_width += char_width;
                }
                current_line.push_str(word);
                current_width += word_width;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
                current_width = word_width;
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        lines
    }

    pub fn measure_text(text: &str, font_size: f32) -> (f32, f32) {
        let lines: Vec<&str> = text.lines().collect();
        let max_width = lines.iter()
            .map(|line| line.len() as f32 * font_size * 0.6)
            .fold(0.0_f32, f32::max);
        let height = lines.len() as f32 * font_size * 1.2;
        
        (max_width, height)
    }
}

#[derive(Debug, Clone)]
pub enum TextSegment {
    Normal(String),
    Bold(String),
    Italic(String),
    Colored { text: String, color: Color },
}

pub struct RichTextParser;

impl RichTextParser {
    pub fn parse(text: &str) -> Vec<TextSegment> {
        let mut segments = Vec::new();
        let mut current_text = String::new();
        let mut chars = text.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '[' {
                let mut tag_content = String::new();
                let mut closed_bracket = false;
                let mut temp_chars = Vec::new();

                while let Some(&next_char) = chars.peek() {
                    temp_chars.push(chars.next().unwrap());
                    if next_char == ']' {
                        closed_bracket = true;
                        break;
                    }
                    tag_content.push(next_char);
                }

                let is_valid_tag = tag_content == "b" || 
                                   tag_content == "i" || 
                                   tag_content.starts_with("color=");

                if closed_bracket && is_valid_tag {
                    if !current_text.is_empty() {
                        segments.push(TextSegment::Normal(current_text.clone()));
                        current_text.clear();
                    }

                    let tag_name = tag_content.split('=').next().unwrap_or(&tag_content);
                    let closing_tag = format!("[/{}]", tag_name);
                    let mut content = String::new();
                    let mut found_closing = false;

                    while let Some(inner_c) = chars.next() {
                        if inner_c == '[' {
                            let mut potential_close = String::from("[");
                            while let Some(&p) = chars.peek() {
                                potential_close.push(chars.next().unwrap());
                                if p == ']' { break; }
                            }

                            if potential_close == closing_tag {
                                found_closing = true;
                                break;
                            } else {
                                content.push_str(&potential_close);
                            }
                        } else {
                            content.push(inner_c);
                        }
                    }

                    if found_closing {
                        match tag_name {
                            "b" => segments.push(TextSegment::Bold(content)),
                            "i" => segments.push(TextSegment::Italic(content)),
                            _ if tag_content.starts_with("color=") => {
                                let hex = &tag_content[6..];
                                if let Some(color) = Color::from_hex(hex) {
                                    segments.push(TextSegment::Colored { text: content, color });
                                } else {
                                    segments.push(TextSegment::Normal(content));
                                }
                            }
                            _ => segments.push(TextSegment::Normal(content)),
                        }
                    }
                } else {
                    current_text.push('[');
                    for char_from_temp in temp_chars {
                        current_text.push(char_from_temp);
                    }
                }
            } else {
                current_text.push(c);
            }
        }

        if !current_text.is_empty() {
            segments.push(TextSegment::Normal(current_text));
        }
        segments
    }
}
