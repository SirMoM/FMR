use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    id: String,
    #[serde(rename = "type")]
    _type: i32,
    guild_id: Option<String>,
    position: Option<i32>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: bool,
    last_message_id: Option<String>,
    bitrate: Option<i32>,
    user_limit: Option<i32>,
    rate_limit_per_user: Option<i32>,
    icon: Option<String>,
    owner_id: Option<String>,
    managed: Option<bool>,
    parent_id: Option<String>,
    last_pin_timestamp: Option<String>,
    rtc_region: Option<String>,
    video_quality_mode: Option<i32>,
    message_count: Option<i32>,
    member_count: Option<i32>,
    default_auto_archive_duration: Option<i32>,
    permissions: Option<String>,
    flags: Option<i32>,
    total_message_sent: Option<i32>,
    applied_tags: Option<Vec<String>>,
    default_thread_rate_limit_per_user: Option<i32>,
    default_sort_order: Option<i32>,
    default_forum_layout: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub(crate) id: String,
    #[serde(rename = "type")]
    _type: i32,
    content: String,
    channel_id: String,
    pub(crate) author: User,
    pub attachments: Vec<Attachment>,
    // pub embeds: Vec<String>,
    mentions: Vec<User>,
    mention_roles: Vec<String>,
    pinned: bool,
    mention_everyone: bool,
    tts: bool,
    pub timestamp: DateTime<Utc>,
    edited_timestamp: Option<String>,
    flags: i32,
    components: Vec<String>,
    interaction: Option<Interaction>,
    webhook_id: Option<String>,
    pub reactions: Option<Vec<Reaction>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Interaction {
    id: String,
    #[serde(rename = "type")]
    _type: i32,
    name: String,
    user: User,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub(crate) id: String,
    pub(crate) username: String,
    avatar: Option<String>,
    discriminator: String,
    public_flags: i32,
    flags: i32,
    banner: Option<String>,
    accent_color: Option<String>,
    global_name: Option<String>,
    avatar_decoration_data: Option<String>,
    banner_color: Option<String>,
    bot: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    id: String,
    filename: String,
    size: i32,
    pub(crate) url: String,
    proxy_url: String,
    width: Option<i32>,
    height: Option<i32>,
    content_type: String,
    placeholder: Option<String>,
    placeholder_version: Option<i32>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reaction {
    pub emoji: Emoji,
    pub count: i32,
    count_details: CountDetails,
    burst_colors: Vec<String>,
    me_burst: bool,
    burst_me: bool,
    me: bool,
    burst_count: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emoji {
    id: Option<String>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CountDetails {
    burst: i32,
    normal: i32,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageBody {
    pub content: String,
    pub tts: Option<bool>,
    pub embeds: Option<Vec<Embed>>,
    //pub components: Option<Vec<MessageComponent>>,
    pub sticker_ids: Option<Vec<String>>, // Assuming snowflakes are represented as Strings
    pub payload_json: Option<String>,
    pub flags: Option<i32>,
}
/// Describes a field that can be used inside a message embed
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedField {
    /// Field title
    #[serde(rename = "name")]
    pub title: String,
    /// Field value
    pub value: String,
    /// If true, the field will be displayed on the same line as the last
    pub inline: bool,
}

/// Describes an embed author
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedAuthor {
    /// Author name
    pub name: String,
    /// String of the author
    pub String: Option<String>,
    /// Avatar String for the author
    pub icon_String: Option<String>,
}

/// Describes an embed thumbnail
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedThumbnail {
    /// Thumbnail String
    pub String: String,
}

/// Describes an embed image
#[derive(Debug, Serialize, Deserialize)]
pub struct EmbedImage {
    /// Image String
    pub url: String,
}

/// Describes an embed footer
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmbedFooter {
    /// Footer text
    pub text: String,
    /// Footer icon String
    pub icon_String: Option<String>,
}

/// Describes an embed
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Embed {
    /// The title of the embed
    pub title: String,
    /// The description of the embed
    pub description: String,
    /// The color of the embed
    pub color: Option<u32>,
    /// The embed author
    pub author: Option<EmbedAuthor>,
    /// Possible fields
    pub fields: Option<Vec<EmbedField>>,
    /// The thumbnail of the embed
    pub thumbnail: Option<EmbedThumbnail>,
    /// The image of the embed
    pub image: Option<EmbedImage>,
    /// The footer of the embed
    pub footer: Option<EmbedFooter>,
}
