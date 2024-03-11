use chrono::{Datelike, DateTime, Utc};
use log::{debug, log};
use reqwest::blocking::Client;

use crate::discord_api::request::*;
use crate::error::Error::{Generic, Static};
use crate::models::discord::Message;
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

    fn get_messages(&self, after: Option<&str>, limit: Option<i32>) -> Result<Vec<Message>>;
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
    fn get_messages(&self, after: Option<&str>, _limit: Option<i32>) -> Result<Vec<Message>> {
        let limit = _limit.unwrap_or(1);
        let config: &Config = Config::get();
        let mut url: String = format!("{}/channels/{}/messages?limit={limit}", config.base_url, config.channel_id);

        if after.is_some() {
            let msg_id = after.unwrap();
            url.push_str(format!("?after={msg_id}").as_str())
        }

        let body = request(&self.client, config.token.as_str(), url.as_str(), false)?;

        if body.contains("rate limited") {
            return Err(Static("Rate limited"));
        }

        let msgs: Vec<Message> = serde_json::from_str(body.as_str())
            .expect(format!("Could not parse Message: {}", body).as_str());

        return Ok(msgs);
    }
}

pub struct MessageService<C: GetMsgs> {
    pub client: C,
}

impl<C: GetMsgs> MessageService<C> {
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

        while run {
            let mut page: Vec<Message> = self.client.get_messages(None, Option::from(15)).unwrap();
            page.sort_by(|msg1, msg2| msg1.timestamp.cmp(&msg2.timestamp));
            let timestamps: Vec<DateTime<Utc>> =
                (&page).into_iter().map(|msg| msg.timestamp).collect();
            timestamps.iter().enumerate().for_each(|(idx, ts)| {
                println!("{}, {:#?}", idx, ts);
            });

            let timestamp = &page.first().unwrap().timestamp;
            debug!("1. {:#?} 2. {:#?}", timestamp, wanted_month);
            debug!("{}", wanted_month.month());
            if timestamp.year() < wanted_month.year()
                || (timestamp.year() == wanted_month.year()
                    && timestamp.month() > wanted_month.month())
            {
                println!("The timestamp is older than the wanted month");
                run = false;
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
}
