#![allow(unused)] // For beginning only.

use chrono::{DateTime, Utc};

use crate::discord_api::messages::{MessageGetter, MessageService};
use crate::models::discord::Message;
use crate::prelude::*;
use crate::utils::config;
use crate::utils::config::Config;
use crate::utils::last_month_date::last_month_date;

mod discord_api;
mod error;
mod models;
mod prelude;
mod tests;
mod utils;

fn main() -> Result<()> {
    let last_month = last_month_date().expect("???");
    println!("{:#?}", &last_month);

    let message_service = MessageService::new(MessageGetter::new());

    let msgs: Vec<Message> = message_service.get_memes_for_month(&last_month);

    let timestamps: Vec<DateTime<Utc>> = (&msgs).into_iter().map(|msg| msg.timestamp).collect();
    timestamps.iter().enumerate().for_each(|(idx, ts)| {
        println!("{}, {:#?}", idx, ts);
    });

    let msgs_as_ref = msgs.iter().collect();

    let msgs_with_upvote = sort_messages_by_upvote(msgs_as_ref, Config::get().upvote_emoji.as_ref());

    println!("{:#?}", &msgs_with_upvote.first());
    Ok(())
}

pub fn sort_messages_by_upvote(msgs: Vec<&Message>, upvote_emoji: &str) -> Vec<Message> {
    /// Sorts a vector of messages by the number of upvotes.
    ///
    /// This function takes a vector of `Message` references and filters out the messages that have attachments and at least one reaction with the "👍" emoji. It then sorts the remaining messages based on the count of the "👍" reactions in ascending order.
    ///
    /// # Arguments
    ///
    /// * `msgs` - A vector of `Message` references to be sorted.
    /// * `upvote_emoji` - A &str containing the emoji that counts als the upvote or the id of the custom reaction 
    ///
    /// # Returns
    ///
    /// A new vector containing the sorted `Messages`
    ///

    let mut msgs_with_upvote: Vec<&Message> = msgs
        .into_iter()
        .filter(|msg| !msg.attachments.is_empty())
        .filter(|msg| {
            msg.reactions.as_ref().map_or(false, |reactions| {
                reactions
                    .iter()
                    .any(|rec| rec.count > 0 && rec.emoji.name == upvote_emoji)
            })
        })
        .collect();

    msgs_with_upvote.sort_by(|msg1, msg2| {
        let msg1_count = &(msg1
            .reactions
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|rec| rec.emoji.name == upvote_emoji)
            .unwrap()
            .count);
        let msg2_count = &(msg2
            .reactions
            .as_ref()
            .unwrap()
            .into_iter()
            .find(|rec| rec.emoji.name == upvote_emoji)
            .unwrap()
            .count);

        msg1_count.cmp(&msg2_count)
    });
    
    msgs_with_upvote.into_iter().map(|msg| msg.clone()).collect()
}
