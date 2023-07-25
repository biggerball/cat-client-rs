use crate::message::consts;

#[derive(Default)]
pub struct Header<'h> {
    domain: &'h str,
    hostname: &'h str,
    ip: &'h str,

    message_id: String,
    parent_message_id: &'static str,
    root_message_id: &'static str,
}

impl <'h> Header<'h> {

    pub fn new(domain: &'h str, hostname: &'h str, ip: &'h str, message_id: String) -> Self {
        Header{
            domain,
            hostname,
            ip,
            message_id,
            parent_message_id: consts::EMPTY_DATA,
            root_message_id: consts::EMPTY_DATA,
        }
    }

    pub fn get_domain(&'h self) -> &'h str {
        self.domain
    }

    pub fn get_hostname(&'h self) -> &'h str {
        self.hostname
    }

    pub fn get_ip(&'h self) -> &'h str {
        self.ip
    }

    pub fn get_message_id(&self) -> &str {
        self.message_id.as_str()
    }

    pub fn get_parent_message_id(&self) -> &'static str {
        self.parent_message_id
    }

    pub fn get_root_message_id(&self) -> &'static str {
        self.root_message_id
    }
}