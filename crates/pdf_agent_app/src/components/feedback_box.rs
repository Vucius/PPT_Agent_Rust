use crate::message::Message;
use iced::widget::{button, row, text_input};
use iced::{Element, Length};

pub fn view<'a>(
    feedback_input: &'a str,
    selected_block_id: Option<&'a str>,
) -> Element<'a, Message> {
    let is_block_selected = selected_block_id.is_some();
    let mut submit_btn = button("Submit");
    if is_block_selected && !feedback_input.trim().is_empty() {
        submit_btn = submit_btn.on_press(Message::SubmitFeedbackClicked);
    }

    let feedback_label = if let Some(id) = selected_block_id {
        format!("Feedback on Block (ID: {})...", &id[..id.len().min(8)])
    } else {
        "Select a block above to request LLM alignment...".to_string()
    };

    row![
        text_input(&feedback_label, feedback_input)
            .on_input(Message::FeedbackInputChanged),
        submit_btn
    ]
    .spacing(10)
    .width(Length::Fill)
    .into()
}
