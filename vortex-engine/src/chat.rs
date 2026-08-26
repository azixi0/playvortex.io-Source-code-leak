// -- leaked by @azixi0 on github
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChatEntry { Player { from_id: u64, username: String, body: String }, Warning(String), System(String) }

#[derive(Debug, Default)]
pub struct ChatLog { entries: Vec<ChatEntry> }

impl ChatLog {
    pub fn push(&mut self, entry: ChatEntry) { self.entries.push(entry); }
    pub fn entries(&self) -> &[ChatEntry] { &self.entries }
}

