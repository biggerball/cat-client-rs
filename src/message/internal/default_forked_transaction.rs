use crate::message::transaction::{ForkedTransaction, Transaction};

pub const TYPE_ID: &'static str = "DefaultForkedTransaction";
pub struct DefaultForkedTransaction {

}

impl ForkedTransaction for DefaultForkedTransaction {
    
}

impl Transaction for DefaultForkedTransaction {
    fn type_name(&self) -> &'static str {
        TYPE_ID
    }

    fn add_children(&self, message: crate::message::message::Message) {
        todo!()
    }
}
