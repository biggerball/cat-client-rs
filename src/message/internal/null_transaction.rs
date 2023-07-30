use crate::message::transaction::Transaction;

pub const TYPE_ID: &'static str = "NullTransaction";
pub struct NullTransaction {

}

impl Transaction for NullTransaction {
    fn type_name(&self) -> &'static str {
        TYPE_ID
    }

    fn add_children(&self, message: crate::message::message::Message) {
    }
}