use iced::widget::{button, container, text};
use iced::{Border, Color, Shadow, Theme};

// ===== Color Palette =====
// Professional monochrome design for ML tools

// Base colors - Pure grayscale
pub const BG_DARK: Color = Color::from_rgb(0.11, 0.11, 0.11); // #1c1c1c
pub const BG_CARD: Color = Color::from_rgb(0.15, 0.15, 0.15); // #262626
pub const BG_RAISED: Color = Color::from_rgb(0.18, 0.18, 0.18); // #2e2e2e
pub const BG_INPUT: Color = Color::from_rgb(0.12, 0.12, 0.12); // #1f1f1f
pub const BG_HOVER: Color = Color::from_rgb(0.20, 0.20, 0.20); // #333333

// Accent - Single subtle gray for interactive elements
pub const ACCENT: Color = Color::from_rgb(0.35, 0.35, 0.35); // #595959
pub const ACCENT_HOVER: Color = Color::from_rgb(0.40, 0.40, 0.40); // #666666
pub const ACCENT_ACTIVE: Color = Color::from_rgb(0.30, 0.30, 0.30); // #4d4d4d

// No semantic colors - pure professional gray
pub const ERROR: Color = Color::from_rgb(0.50, 0.50, 0.50); // #808080

// Text colors - Grayscale hierarchy
pub const TEXT_PRIMARY: Color = Color::from_rgb(0.92, 0.92, 0.92); // #ebebeb
pub const TEXT_SECONDARY: Color = Color::from_rgb(0.62, 0.62, 0.62); // #9e9e9e
pub const TEXT_TERTIARY: Color = Color::from_rgb(0.45, 0.45, 0.45); // #737373
pub const TEXT_DISABLED: Color = Color::from_rgb(0.35, 0.35, 0.35); // #595959

// Border colors - Very subtle
pub const BORDER: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.06); // rgba(255,255,255,0.06)
pub const BORDER_FOCUS: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.12); // rgba(255,255,255,0.12)
pub const BORDER_ACCENT: Color = Color::from_rgba(1.0, 1.0, 1.0, 0.10); // rgba(255,255,255,0.10)

// ===== Constants =====

pub const BORDER_RADIUS: f32 = 6.0;
pub const BORDER_RADIUS_CARD: f32 = 8.0;
pub const BORDER_WIDTH: f32 = 1.0;

// ===== Container Style Functions =====

pub fn container_card() -> impl Fn(&Theme) -> container::Style {
    |_theme| container::Style {
        background: Some(BG_CARD.into()),
        border: Border {
            color: BORDER,
            width: BORDER_WIDTH,
            radius: BORDER_RADIUS_CARD.into(),
        },
        shadow: Shadow::default(),
        text_color: Some(TEXT_PRIMARY),
    }
}



// ===== Button Style Functions =====

pub fn button_primary() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_color) = match status {
            button::Status::Active => (ACCENT, ACCENT),
            button::Status::Hovered => (ACCENT_HOVER, ACCENT_HOVER),
            button::Status::Pressed => (ACCENT_ACTIVE, ACCENT_ACTIVE),
            button::Status::Disabled => (BG_INPUT, BORDER),
        };

        button::Style {
            background: Some(bg.into()),
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color: if matches!(status, button::Status::Disabled) {
                TEXT_DISABLED
            } else {
                TEXT_PRIMARY
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn button_secondary() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_color, text_color) = match status {
            button::Status::Active => (BG_RAISED, BORDER, TEXT_PRIMARY),
            button::Status::Hovered => (BG_HOVER, BORDER_FOCUS, TEXT_PRIMARY),
            button::Status::Pressed => (BG_INPUT, BORDER, TEXT_PRIMARY),
            button::Status::Disabled => (BG_INPUT, BORDER, TEXT_DISABLED),
        };

        button::Style {
            background: Some(bg.into()),
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color,
            shadow: Shadow::default(),
        }
    }
}

pub fn button_success() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_color) = match status {
            button::Status::Active => (ACCENT, BORDER_ACCENT),
            button::Status::Hovered => (ACCENT_HOVER, BORDER_FOCUS),
            button::Status::Pressed => (ACCENT_ACTIVE, BORDER_ACCENT),
            button::Status::Disabled => (BG_INPUT, BORDER),
        };

        button::Style {
            background: Some(bg.into()),
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color: if matches!(status, button::Status::Disabled) {
                TEXT_DISABLED
            } else {
                TEXT_PRIMARY
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn button_danger() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| {
        let (bg, border_color) = match status {
            button::Status::Active => (BG_RAISED, ERROR),
            button::Status::Hovered => (BG_HOVER, ERROR),
            button::Status::Pressed => (BG_INPUT, ERROR),
            button::Status::Disabled => (BG_INPUT, BORDER),
        };

        button::Style {
            background: Some(bg.into()),
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color: if matches!(status, button::Status::Disabled) {
                TEXT_DISABLED
            } else {
                TEXT_SECONDARY
            },
            shadow: Shadow::default(),
        }
    }
}

pub fn button_ghost() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| {
        let bg = match status {
            button::Status::Active => Color::TRANSPARENT,
            button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            button::Status::Pressed => Color::TRANSPARENT,
            button::Status::Disabled => Color::TRANSPARENT,
        };

        let border_color = match status {
            button::Status::Hovered => BORDER,
            _ => Color::TRANSPARENT,
        };

        button::Style {
            background: Some(bg.into()),
            border: Border {
                color: border_color,
                width: BORDER_WIDTH,
                radius: BORDER_RADIUS.into(),
            },
            text_color: if matches!(status, button::Status::Disabled) {
                TEXT_DISABLED
            } else {
                TEXT_PRIMARY
            },
            shadow: Shadow::default(),
        }
    }
}

// ===== Text Style Functions =====

pub fn text_primary() -> impl Fn(&Theme) -> text::Style {
    |_theme| text::Style {
        color: Some(TEXT_PRIMARY),
    }
}

pub fn text_secondary() -> impl Fn(&Theme) -> text::Style {
    |_theme| text::Style {
        color: Some(TEXT_SECONDARY),
    }
}

pub fn text_tertiary() -> impl Fn(&Theme) -> text::Style {
    |_theme| text::Style {
        color: Some(TEXT_TERTIARY),
    }
}


