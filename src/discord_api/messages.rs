use std::ops::DerefMut;
use std::slice::Iter;
use std::thread;
use std::time::Duration;
use std::vec::IntoIter;

use chrono::{DateTime, Datelike, Utc};
use log::{debug, log, warn};
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE};

use crate::discord_api::request::*;
use crate::error::Error::{Generic, Static};
use crate::models::discord::{Message, MessageBody};
use crate::prelude::*;
use crate::utils::config::Config;

pub trait GetMsgs {
    /// Trait for retrieving messages.
    ///
    /// This trait provides a method `get_messages` that can be implemented by types
    /// to retrieve a list of messages. The method takes two optional parameters:
    /// `after` and `_limit`. The `after` parameter is used to specify a message ID
    /// to start retrieving messages after, and the `_limit` parameter is used to
    /// limit the number of messages to retrieve.
    ///
    /// # Arguments
    ///
    /// - `after` - An optional string representing the message ID to start retrieving
    ///             messages after.
    /// - `limit` - An optional integer representing the maximum number of messages to
    ///              retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Message` objects if successful, or an `Error`
    /// if an error occurred during the retrieval process.
    ///
    /// # Examples
    ///
    /// Implement the GetMsgs trait for a custom type
    /// struct MyType;
    /// impl GetMsgs for MyType {
    /// fn get_messages(&self, after: Option<&str>, _limit: Option<i32>) -> Result<Vec<Message>> {
    /// //Implementation goes here
    ///     }
    /// }

    fn get_messages(&self, after: Option<String>, limit: Option<i32>) -> Result<Vec<Message>>;
}
pub trait SendMsgs {
    fn send_messages(&self, message_body: MessageBody) -> Result<()>;
}
pub struct MessageGetter {
    client: Client,
}

impl MessageGetter {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }
}

impl GetMsgs for MessageGetter {
    /// Implements the `GetMsgs` trait for the `MessageGetter` struct.
    ///
    /// This implementation provides a method `get_messages` that retrieves messages from a Discord channel.
    ///
    /// # Arguments
    ///
    /// - `after`: An optional string representing the ID of the message to start retrieving from.
    /// - `_limit`: An optional integer representing the maximum number of messages to retrieve.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Message` structs if successful, or an `Error` if an error occurs.
    ///
    fn get_messages(&self, after: Option<String>, _limit: Option<i32>) -> Result<Vec<Message>> {
        let limit = _limit.unwrap_or(1);
        let config: &Config = Config::get();
        let mut url: String = format!(
            "{}/channels/{}/messages?",
            config.base_url, config.in_channel_id
        );

        if after.is_some() {
            let msg_id = after.as_ref().unwrap().clone();
            url.push_str(format!("after={msg_id}&").as_str())
        }
        url.push_str(format!("limit={}", limit.to_string()).as_str());
        println!("{:#?}", url);

        let body = request(&self.client, config.token.as_str(), url.as_str(), false)?;

        if body.contains("rate limited") {
            warn!("Get messages was rate limited!");
            debug!("GET Messages was rate limited!");
            thread::sleep(Duration::from_secs(1));
            return self.get_messages(after, _limit);
        }

        let msgs: Vec<Message> = serde_json::from_str(body.as_str())
            .expect(format!("Could not parse Message:\n Body:\n {}", body.as_str()).as_str());

        Ok(msgs)
    }
}

impl SendMsgs for MessageGetter {
    fn send_messages(&self, message_body: MessageBody) -> Result<()> {
        let config: &Config = Config::get();
        let mut url: String = format!(
            "{}/channels/{}/messages",
            config.base_url, config.out_channel_id
        );

        let req = self
            .client
            .post(url)
            .header("Authorization", config.token.as_str())
            .header(CONTENT_TYPE, "application/json")
            .header(ACCEPT, "application/json")
            .body(serde_json::to_string(&message_body).unwrap())
            .build()?;

        let res = self.client.execute(req)?;
        println!("{:#?}", res.status());
        println!("{:#?}", res.text());
        Ok(())
    }
}

pub struct MessageService<C: GetMsgs + SendMsgs> {
    pub client: C,
}

impl<C: GetMsgs + SendMsgs> MessageService<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    pub fn get_memes_for_month(&self, wanted_month: &DateTime<Utc>) -> Vec<Message> {
        /// This is a method called `get_memes_for_month` belonging to a struct `MessageService`.
        /// It takes a reference to a `DateTime<Utc>` called `wanted_month` as input.
        /// The method retrieves messages from the `client` using the `get_messages` method.
        /// It sorts the messages by timestamp and filters them based on the `wanted_month`.
        /// The filtered messages are then returned as a `Vec<Message>`.
        /// The method continues retrieving messages until it encounters a timestamp older than the `wanted_month`.
        /// Note: The code snippet assumes the existence of a `debug!` macro for logging purposes.
        ///
        /// # Arguments
        ///
        /// * `self` - The `MessageService` instance.
        /// * `wanted_month` - The desired month to filter the messages.
        ///
        /// # Returns
        ///
        /// A `Vec<Message>` containing the filtered messages for the desired month.
        ///
        /// # Example
        ///
        /// use chrono::{DateTime, Utc};
        /// use your_crate::MessageService;
        /// let service = MessageService::new();
        /// let wanted_month = DateTime::parse_from_rfc3339("2022-01-01T00:00:00Z").unwrap();
        /// let memes = service.get_memes_for_month(&wanted_month);
        /// println!("{:#?}", memes);
        let mut run = true;
        let mut result: Vec<Message> = Vec::new();

        let mut last_msg: Option<String> = None;

        while run {
            let mut page: Vec<Message> = self
                .client
                .get_messages(last_msg.clone(), Some(10))
                .unwrap();
            if page.is_empty() {
                run = false;

                break;
            }

            let oldest_msg: Message = page.first().expect("Should never happen!").clone();
            page.sort_by(|msg1, msg2| msg1.timestamp.cmp(&msg2.timestamp));

            let page_iter = page.clone().into_iter();
            let timestamps: Vec<DateTime<Utc>> = page_iter.map(|msg| msg.timestamp).collect();

            timestamps.iter().enumerate().for_each(|(idx, ts)| {
                println!("{}, {:#?}", idx, ts);
            });

            let timestamp: DateTime<Utc> = oldest_msg.clone().timestamp;

            debug!("1. {:#?} 2. {:#?}", timestamp, wanted_month);
            debug!("{}", wanted_month.month());

            if timestamp.year() < wanted_month.year()
                || (timestamp.year() == wanted_month.year()
                    && timestamp.month() > wanted_month.month())
            {
                println!("The timestamp is older than the wanted month");
                run = false;
            } else {
                let id = oldest_msg.clone().id;
                last_msg = Some(id);
            }

            result.append(&mut page);
        }

        // Filter wanted_month.year() == timestamp.year() && wanted_month.month() == timestamp.month()
        let filtered_result: Vec<Message> = result
            .iter()
            .filter(|msg| {
                wanted_month.year() == msg.timestamp.year()
                    && wanted_month.month() == msg.timestamp.month()
            })
            .map(|msg| msg.clone())
            .collect();

        return filtered_result;
    }

    pub fn send_message(&self, message_body: MessageBody) {
        self.client.send_messages(message_body);
    }
}
