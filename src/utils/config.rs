use std::env;
use std::fmt::format;
use dotenv::dotenv;
use serde::Deserialize;
use lazy_static::lazy_static;
use crate::error::Error::{Generic, Static};
use crate::prelude::*;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub client_id: String,
    pub base_url: String,
    pub channel_id: String,
    pub upvote_emoji: String,
}

impl Config {
    fn new() -> Self {
        dotenv().expect("Could not find .env file!");

        Self {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            base_url: env::var("BASE_URL").expect("BASE_URL must be set"),
            channel_id: env::var("CHANNEL_ID").expect("CHANNEL_ID must be set"),
            upvote_emoji: env::var("UPVOTE_EMOJI").expect("UPVOTE_EMOJI must be set"),
        }
    }

    pub fn get() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: Config = Config::new();
        }
        &INSTANCE
    }
}

