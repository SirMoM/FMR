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
    author: User,
    pub attachments: Vec<Attachment>,
    // pub embeds: Vec<String>,
    mentions: Vec<String>,
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
    id: String,
    username: String,
    avatar: String,
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
    url: String,
    proxy_url: String,
    width: i32,
    height: i32,
    content_type: String,
    placeholder: String,
    placeholder_version: i32,
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
