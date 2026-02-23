//! Text rendering and wrapping utilities

pub struct TextRenderer;

impl TextRenderer {
    /// Break text into lines that fit within a given width
    pub fn wrap_text(text: &str, max_width: u32, font_size: u32) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        
        // Approximate character width (this is a rough estimate)
        let approx_char_width = (font_size as f32 * 0.6) as u32;
        let max_chars = (max_width / approx_char_width).max(1);
        
        for word in words {
            let test_line = if current_line.is_empty() {
                word.to_string()
            } else {
                format!("{} {}", current_line, word)
            };
            
            if test_line.len() as u32 <= max_chars {
                current_line = test_line;
            } else {
                if !current_line.is_empty() {
                    lines.push(current_line);
                }
                current_line = word.to_string();
            }
        }
        
        if !current_line.is_empty() {
            lines.push(current_line);
        }
        
        if lines.is_empty() {
            lines.push(String::new());
        }
        
        lines
    }

    /// Calculate the height needed for wrapped text
    pub fn calculate_text_height(lines: usize, font_size: u32, line_spacing: u32) -> u32 {
        if lines == 0 {
            return 0;
        }
        (font_size * lines as u32) + (line_spacing * (lines.saturating_sub(1)) as u32)
    }
}
