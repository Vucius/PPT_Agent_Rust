use crate::message::Message;
use crate::theme::BOLD_FONT;
use iced::widget::{button, column, row, text, text_input, Space};
use iced::{Element, Length};
use pdf_agent_core::config::PipelineConfig;

pub fn view(config: &PipelineConfig, api_key: &str) -> Element<'static, Message> {
    let title = text("Settings").size(28).font(BOLD_FONT);

    // LLM Section
    let llm_section = column![
        text("LLM Configuration").size(20).font(BOLD_FONT),
        Space::with_height(10),
        
        row![
            text("Provider (e.g. mock, openai):").width(Length::Fixed(200.0)),
            text_input("mock / openai", &config.llm.provider)
                .on_input(Message::LlmProviderChanged)
                .width(Length::Fill),
        ].spacing(10),
        
        Space::with_height(5),
        
        row![
            text("Model Name:").width(Length::Fixed(200.0)),
            text_input("gpt-4o-mini", &config.llm.model_name)
                .on_input(Message::LlmModelChanged)
                .width(Length::Fill),
        ].spacing(10),

        Space::with_height(5),

        row![
            text("API Base URL:").width(Length::Fixed(200.0)),
            text_input("https://api.openai.com/v1", &config.llm.base_url)
                .on_input(Message::LlmBaseUrlChanged)
                .width(Length::Fill),
        ].spacing(10),

        Space::with_height(5),

        row![
            text("API Key:").width(Length::Fixed(200.0)),
            text_input("api-key-obscured", api_key)
                .on_input(Message::LlmKeyChanged)
                .width(Length::Fill),
        ].spacing(10),

        Space::with_height(5),

        row![
            text("Daily Token Limit:").width(Length::Fixed(200.0)),
            text_input("50000", &config.llm.daily_limit.to_string())
                .on_input(Message::LlmLimitChanged)
                .width(Length::Fill),
        ].spacing(10),
    ].spacing(10);

    // OCR Section
    let ocr_section = column![
        text("OCR Engine Settings").size(20).font(BOLD_FONT),
        Space::with_height(10),

        row![
            text("OCR Mode (auto / always / never):").width(Length::Fixed(200.0)),
            text_input("auto", &config.ocr_mode)
                .on_input(Message::OcrModeChanged)
                .width(Length::Fill),
        ].spacing(10),
    ].spacing(10);

    // Save Button
    let save_btn = button("Save Settings to Storage")
        .on_press(Message::SaveSettingsClicked)
        .padding(10);

    column![
        title,
        Space::with_height(20),
        llm_section,
        Space::with_height(20),
        ocr_section,
        Space::with_height(30),
        save_btn,
    ]
    .padding(30)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}
