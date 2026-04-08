#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Message {
    pub message_id: i32,
    pub conversation_id: i32,
    pub sender_id: i32,
    pub content: String,
    pub timestamp: String,
    pub status: Status,
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub enum Status {
    Sent,
    Delivered,
    Draft,
}

impl Message {
    pub fn new(msg: String) -> Self {
        Message {
            message_id: 1,
            conversation_id: 1,
            sender_id: 1,
            content: msg,
            timestamp: String::from("0102020"),
            status: Status::Sent,
        }
    }
}
