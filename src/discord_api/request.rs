use std::env;
use reqwest::{
    blocking::Client,
    header::{ACCEPT, CONTENT_TYPE},
};

use crate::prelude::*;

pub fn request(client: &Client, token: &str, url: &str, debug: bool) -> Result<String> {
    let req = client
        .get(url)
        .header("Authorization", token)
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .build()?;
    //.map_err(|error: reqwest::Error| Error::Generic(format!("Reqwest err:\n {:#?}", error)))?;

    if debug {
        println!("{:#?}", req);
    }

    Ok(client.execute(req)?.text()?)
}
