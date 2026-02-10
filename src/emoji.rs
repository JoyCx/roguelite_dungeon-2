/// Unicode and emoji support utilities
/// Handles proper width calculation for emoji characters which often take 2 display widths
use unicode_width::UnicodeWidthStr;

/// Get the display width of a string, accounting for emoji and wide characters
pub fn display_width(s: &str) -> usize {
    UnicodeWidthStr::width(s)
}

/// Truncate a string to fit within a maximum display width
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    let mut result = String::new();
    let mut current_width = 0;

    for c in s.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(1);
        if current_width + char_width <= max_width {
            result.push(c);
            current_width += char_width;
        } else {
            break;
        }
    }

    result
}

/// Pad a string to reach a target display width with spaces
pub fn pad_to_width(s: &str, target_width: usize) -> String {
    let current_width = display_width(s);
    if current_width >= target_width {
        s.to_string()
    } else {
        let padding = target_width - current_width;
        format!("{}{}", s, " ".repeat(padding))
    }
}

/// Center a string within a target display width
pub fn center_in_width(s: &str, target_width: usize) -> String {
    let current_width = display_width(s);
    if current_width >= target_width {
        s.to_string()
    } else {
        let total_padding = target_width - current_width;
        let left_padding = total_padding / 2;
        let right_padding = total_padding - left_padding;
        format!(
            "{}{}{}",
            " ".repeat(left_padding),
            s,
            " ".repeat(right_padding)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emoji_width() {
        // Emoji typically takes 2 widths
        assert_eq!(display_width("🗡️"), 2);
        assert_eq!(display_width("🏹"), 2);
    }

    #[test]
    fn test_ASCII_width() {
        assert_eq!(display_width("A"), 1);
        assert_eq!(display_width("Hello"), 5);
    }

    #[test]
    fn test_truncate() {
        let truncated = truncate_to_width("Hello🗡️World", 8);
        assert_eq!(display_width(&truncated), 8);
    }
}
