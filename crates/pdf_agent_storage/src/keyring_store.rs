use crate::error::Result;
use keyring::Entry;

pub struct KeyringStore {
    service_name: String,
}

impl KeyringStore {
    pub fn new() -> Self {
        Self {
            service_name: "ppt-agent-rust".to_string(),
        }
    }

    pub fn set_api_key(&self, provider: &str, api_key: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, provider)?;
        entry.set_password(api_key)?;
        Ok(())
    }

    pub fn get_api_key(&self, provider: &str) -> Result<String> {
        let entry = Entry::new(&self.service_name, provider)?;
        let password = entry.get_password()?;
        Ok(password)
    }

    pub fn delete_api_key(&self, provider: &str) -> Result<()> {
        let entry = Entry::new(&self.service_name, provider)?;
        entry.delete_password()?;
        Ok(())
    }
}

impl Default for KeyringStore {
    fn default() -> Self {
        Self::new()
    }
}
