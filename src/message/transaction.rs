use super::message::Message;

pub trait Transaction {
    fn type_name(&self) -> &'static str;

    fn add_children(&self, message: Message);
}

pub trait ForkedTransaction : Transaction {
    
}