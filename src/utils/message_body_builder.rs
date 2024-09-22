use crate::models::discord::{Embed, EmbedImage, EmbedThumbnail, MessageBody};

pub struct MessageBodyBuilder {
    content: String,
    embeds: Option<Vec<Embed>>,
}

impl MessageBodyBuilder {
    pub fn new(content: &str) -> Self {
        MessageBodyBuilder {
            content: content.to_string(),
            embeds: None,
        }
    }

    pub fn embeds(mut self, embeds: Vec<Embed>) -> Self {
        self.embeds = Some(embeds);
        self
    }

    pub fn add_embed(mut self, embed: Embed) -> Self {
        match self.embeds {
            Some(ref mut embeds) => embeds.push(embed),
            None => self.embeds = Some(vec![embed]),
        }
        self
    }

    pub fn build(self) -> MessageBody {
        MessageBody {
            content: self.content,
            tts: Option::from(false),
            embeds: self.embeds,
            sticker_ids: None,
            payload_json: None,
            flags: None,
        }
    }
}

impl Embed {
    pub fn new(title: &str, description: &str, url: &str) -> Embed {
        Embed {
            title: title.to_string(),
            description: description.to_string(),
            fields: None,
            author: None,
            footer: None,
            color: None,
            image: Some(EmbedImage {
                url: url.to_string(),
            }),
            thumbnail: Some(EmbedThumbnail {
                String: url.to_string(),
            }),
        }
    }
}
