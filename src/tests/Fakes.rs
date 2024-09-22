use std::fs;

use crate::discord_api::messages::GetMsgs;
use crate::models::discord::Message;
use crate::prelude::*;

pub struct FakeMessageGetter;

impl GetMsgs for FakeMessageGetter {
    fn get_messages(&self, after: Option<String>, _limit: Option<i32>) -> Result<Vec<Message>> {
        let path = "src/tests/data/msgs.json";
        let msgs: Vec<Message> = serde_json::from_str(fs::read_to_string(path)?.as_str())
            .expect("Could not parse Message");
        return Ok(msgs);
    }
}
