use crate::message::Message;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Color, Element, Length, Theme};
use similar::{ChangeTag, TextDiff};

/// Semi-transparent background style for deleted lines.
/// Per ui-interaction-spec §3.4.5: rgba(231, 76, 60, 0.15)
struct DeletedRowStyle;

impl container::StyleSheet for DeletedRowStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                0.91, 0.30, 0.24, 0.15,
            ))),
            ..Default::default()
        }
    }
}

/// Semi-transparent background style for inserted lines.
/// Per ui-interaction-spec §3.4.5: rgba(46, 204, 113, 0.15)
struct InsertedRowStyle;

impl container::StyleSheet for InsertedRowStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgba(
                0.18, 0.80, 0.44, 0.15,
            ))),
            ..Default::default()
        }
    }
}

pub fn view<'a>(
    original_markdown: &'a str,
    patch_preview: &'a str,
    history_controls: Element<'a, Message>,
) -> Element<'a, Message> {
    let diff = TextDiff::from_lines(original_markdown, patch_preview);

    let mut diff_lines = column![].spacing(2);

    for change in diff.iter_all_changes() {
        let line_content = change.value().trim_end_matches('\n');
        match change.tag() {
            ChangeTag::Delete => {
                let line_text = format!("- {}", line_content);
                let styled_text = text(line_text)
                    .style(Color::from_rgb(0.91, 0.30, 0.24))
                    .size(14);
                let row_container = container(styled_text)
                    .width(Length::Fill)
                    .padding([2, 4])
                    .style(iced::theme::Container::Custom(Box::new(DeletedRowStyle)));
                diff_lines = diff_lines.push(row_container);
            }
            ChangeTag::Insert => {
                let line_text = format!("+ {}", line_content);
                let styled_text = text(line_text)
                    .style(Color::from_rgb(0.18, 0.80, 0.44))
                    .size(14);
                let row_container = container(styled_text)
                    .width(Length::Fill)
                    .padding([2, 4])
                    .style(iced::theme::Container::Custom(Box::new(InsertedRowStyle)));
                diff_lines = diff_lines.push(row_container);
            }
            ChangeTag::Equal => {
                let line_text = format!("  {}", line_content);
                diff_lines = diff_lines.push(
                    text(line_text)
                        .style(Color::from_rgb(0.8, 0.8, 0.8))
                        .size(14),
                );
            }
        }
    }

    column![
        row![
            text("🔀 Proposed LLM Patch (Diff Preview)")
                .size(20)
                .font(BOLD_FONT),
            Space::with_width(20),
            history_controls,
        ]
        .align_items(iced::Alignment::Center),
        Space::with_height(15),
        row![
            button("✅ Accept Patch").on_press(Message::AcceptPatchClicked),
            Space::with_width(10),
            button("❌ Reject Patch").on_press(Message::RejectPatchClicked),
        ],
        Space::with_height(15),
        iced::widget::scrollable(diff_lines).height(Length::Fill)
    ]
    .width(Length::FillPortion(1))
    .padding(20)
    .into()
}
