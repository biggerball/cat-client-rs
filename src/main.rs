use cat::producer::{MessageProducer, self};

mod cat;
mod message;




fn main() {
    let producer = MessageProducer::new();
    let transaction1 = producer.new_transaction();
    println!("tran1");
    let transaction2 = producer.new_transaction();
}