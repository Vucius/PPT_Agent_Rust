pub fn save_settings_cmd(provider: &str, api_key: &str) {
    let _ = pdf_agent_storage::KeyringStore::new()
        .set_api_key(provider, api_key);
}
