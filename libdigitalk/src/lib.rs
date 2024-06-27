use std::collections::HashMap;
use uuid::{self, Uuid};

pub struct MessageContents {
    text: String,
    // when requesting attachments, path should include some sort of contents of the
    // file, channel id, and file name.
    // e.g. /123456/PNG393939/image.png
    attachments: Vec<Uuid>,
}

pub struct Message {
    contents: MessageContents,
    id: String,
    sender_id: u128,
    replies: Vec<Message>,
    /// hashmap containing K, the reaction index; and V, the reactor.
    reactions: HashMap<i32,u128>
}

pub struct Group {
    identifier: String,
    users: Vec<u128>,
}

pub struct ChannelPermissions {
    read: bool,
    send_text: bool,
    send_attachment: bool,
    react: bool,
    allowed_mention: Vec<Group>
}

pub trait Channel {
//                              uid, perms
    const PERMISSIONS: HashMap<u128, ChannelPermissions>;

}

pub struct User {
    pub username: String,
    pub id: u128,
    pub bio: String,
    pub avatar_path: String,
}

pub enum StatusType {
    Offline,
    Online,
    Idle,
    DoNotDisturb,
}

pub struct Status {
    pub current: StatusType,
    pub message: String,
}
