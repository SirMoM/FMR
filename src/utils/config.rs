use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub client_id: String,
    pub base_url: String,
    pub in_channel_id: String,
    pub out_channel_id: String,
    pub upvote_emoji: String,
}

impl Config {
    fn new() -> Self {
        dotenv().expect("Could not find .env file!");

        Self {
            token: env::var("TOKEN").expect("TOKEN must be set"),
            client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
            base_url: env::var("BASE_URL").expect("BASE_URL must be set"),
            in_channel_id: env::var("IN_CHANNEL_ID").expect("IN_CHANNEL_ID must be set"),
            out_channel_id: env::var("OUT_CHANNEL_ID").expect("OUT_CHANNEL_ID must be set"),
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
