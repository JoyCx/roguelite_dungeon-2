use ratatui::prelude::Color;

/// Centralized color definitions for consistent theming
pub struct Colors;

impl Colors {
    // Health colors
    pub fn health_good() -> Color {
        Color::Green
    }

    pub fn health_warning() -> Color {
        Color::Yellow
    }

    pub fn health_critical() -> Color {
        Color::Red
    }

    // Resource colors
    pub fn gold() -> Color {
        Color::Yellow
    }

    pub fn mana() -> Color {
        Color::Cyan
    }

    // Cooldown colors
    pub fn cooldown_attack() -> Color {
        Color::Red
    }

    pub fn cooldown_dash() -> Color {
        Color::Magenta
    }

    pub fn cooldown_bow() -> Color {
        Color::Cyan
    }

    pub fn cooldown_block() -> Color {
        Color::Blue
    }

    // UI colors
    pub fn text_normal() -> Color {
        Color::White
    }

    pub fn text_highlight() -> Color {
        Color::Yellow
    }

    pub fn text_error() -> Color {
        Color::Red
    }

    pub fn text_success() -> Color {
        Color::Green
    }

    pub fn background_dark() -> Color {
        Color::Black
    }

    pub fn border() -> Color {
        Color::Gray
    }

    pub fn pulse_bright() -> Color {
        Color::Indexed(196)
    }

    pub fn pulse_dim() -> Color {
        Color::Indexed(88)
    }

    // Attack colors
    pub fn attack_area() -> Color {
        Color::Red
    }

    pub fn ultimate_area() -> Color {
        Color::Yellow
    }

    // Item colors
    pub fn item_common() -> Color {
        Color::White
    }

    pub fn item_uncommon() -> Color {
        Color::Green
    }

    pub fn item_rare() -> Color {
        Color::Blue
    }

    pub fn item_epic() -> Color {
        Color::Magenta
    }

    pub fn item_legendary() -> Color {
        Color::Yellow
    }

    // Status effect colors
    pub fn effect_poison() -> Color {
        Color::Green
    }

    pub fn effect_burn() -> Color {
        Color::Red
    }

    pub fn effect_bleed() -> Color {
        Color::Magenta
    }

    pub fn effect_stun() -> Color {
        Color::Yellow
    }

    // Particle colors
    pub fn particle_impact() -> Color {
        Color::Yellow
    }

    pub fn particle_crit() -> Color {
        Color::Red
    }

    pub fn particle_heal() -> Color {
        Color::Green
    }

    pub fn particle_buff() -> Color {
        Color::Cyan
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_colors_exist() {
        // Ensure all color methods are accessible
        let _ = Colors::health_good();
        let _ = Colors::gold();
        let _ = Colors::cooldown_attack();
        let _ = Colors::text_normal();
        let _ = Colors::particle_heal();
    }
}
