use crate::message::Message;
use crate::state::MainState;
use iced::widget::{container, row, text, Space};
use iced::{Color, Element, Length};

/// Status indicator dot color based on application state.
fn status_color(state: &MainState) -> Color {
    match state {
        MainState::Empty | MainState::PdfLoaded { .. } => Color::from_rgb(0.18, 0.8, 0.44),  // green — Ready
        MainState::Converting { .. } => Color::from_rgb(0.39, 0.58, 0.93),                    // blue — Converting
        MainState::Converted { .. } => Color::from_rgb(0.18, 0.8, 0.44),                      // green — Done
        MainState::Failed { .. } => Color::from_rgb(0.91, 0.3, 0.24),                         // red — Failed
    }
}

/// Status label text.
fn status_label(state: &MainState) -> &'static str {
    match state {
        MainState::Empty => "Ready",
        MainState::PdfLoaded { .. } => "PDF Loaded",
        MainState::Converting { .. } => "Converting",
        MainState::Converted { .. } => "Converted",
        MainState::Failed { .. } => "Failed",
    }
}

/// Renders the bottom status bar.
///
/// Layout per ui-interaction-spec §2.4:
/// ```text
/// ┌──────────────────────────────────────────────────────────┐
/// │ ● Ready  │ OCR: Auto  │ Output: Markdown  │ Quota: 0/50k │
/// └──────────────────────────────────────────────────────────┘
/// ```
pub fn view<'a>(
    state: &'a MainState,
    ocr_mode: &'a str,
    output_format: &'a str,
    daily_limit: i64,
) -> Element<'a, Message> {
    let dot_color = status_color(state);
    let label = status_label(state);

    let status_dot = text("●").size(12).style(dot_color);
    let status_text = text(label).size(12).style(Color::from_rgb(0.6, 0.6, 0.65));

    let separator = text("│").size(12).style(Color::from_rgb(0.2, 0.2, 0.25));

    let ocr_label = text(format!("OCR: {}", capitalize(ocr_mode)))
        .size(12)
        .style(Color::from_rgb(0.6, 0.6, 0.65));

    let format_label = text(format!("Output: {}", capitalize(output_format)))
        .size(12)
        .style(Color::from_rgb(0.6, 0.6, 0.65));

    let quota_label = text(format!("Quota: 0/{}", format_limit(daily_limit)))
        .size(12)
        .style(Color::from_rgb(0.6, 0.6, 0.65));

    let bar = row![
        Space::with_width(8),
        status_dot,
        Space::with_width(4),
        status_text,
        Space::with_width(12),
        separator.clone(),
        Space::with_width(12),
        ocr_label,
        Space::with_width(12),
        separator.clone(),
        Space::with_width(12),
        format_label,
        Space::with_width(12),
        separator,
        Space::with_width(12),
        quota_label,
        Space::with_width(Length::Fill),
    ]
    .align_items(iced::Alignment::Center)
    .height(28);

    container(bar)
        .width(Length::Fill)
        .style(iced::theme::Container::default())
        .into()
}

/// Capitalize the first letter of a string.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Format daily token limit for display (e.g. 50000 → "50k").
fn format_limit(limit: i64) -> String {
    if limit >= 1000 {
        format!("{}k", limit / 1000)
    } else {
        limit.to_string()
    }
}
