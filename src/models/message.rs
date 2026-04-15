use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Message {
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: Uuid,
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
            message_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_id: Uuid::new_v4(),
            content: msg,
            timestamp: String::from("0102020"),
            status: Status::Sent,
        }
    }
}
