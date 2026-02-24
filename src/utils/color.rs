//! Color utilities

pub struct Color;

impl Color {
    /// Validate if a string is a hex color code
    pub fn is_hex_color(s: &str) -> bool {
        if !s.starts_with('#') {
            return false;
        }
        
        let hex = &s[1..];
        hex.len() == 6 && hex.chars().all(|c| c.is_ascii_hexdigit())
    }

    /// Convert HTML color name to hex
    pub fn name_to_hex(name: &str) -> Option<String> {
        match name.to_lowercase().as_str() {
            "white" => Some("#ffffff".to_string()),
            "black" => Some("#000000".to_string()),
            "red" => Some("#ff0000".to_string()),
            "green" => Some("#00ff00".to_string()),
            "blue" => Some("#0000ff".to_string()),
            "yellow" => Some("#ffff00".to_string()),
            "cyan" => Some("#00ffff".to_string()),
            "magenta" => Some("#ff00ff".to_string()),
            "gray" | "grey" => Some("#808080".to_string()),
            "silver" => Some("#c0c0c0".to_string()),
            "maroon" => Some("#800000".to_string()),
            "olive" => Some("#808000".to_string()),
            "lime" => Some("#00ff00".to_string()),
            "aqua" => Some("#00ffff".to_string()),
            "teal" => Some("#008080".to_string()),
            "navy" => Some("#000080".to_string()),
            "fuchsia" => Some("#ff00ff".to_string()),
            "purple" => Some("#800080".to_string()),
            _ => None,
        }
    }
}
