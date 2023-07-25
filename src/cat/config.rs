pub struct Config {
    pub domain: String,
    pub hostname: String,
    pub env: String,
    pub ip: String,
    pub ip_hex: String,

    pub http_server_port: u32,
}

impl Config {
    pub fn new() -> Self {
        Config{
            domain: "".to_string(),
            hostname: "".to_string(),
            env: "".to_string(),
            ip: "".to_string(),
            ip_hex: "".to_string(),
            http_server_port: 0,
        }
    }
    pub fn get_domain(&self) -> &String {
        &self.domain
    }

    pub fn get_hostname(&self) -> &String {
        &self.hostname
    }

    pub fn get_ip(&self) -> &String {
        &self.ip
    }

    pub fn get_env(&self) -> &String {
        &self.env
    }

    pub fn get_ip_hex(&self) -> &String {
        &self.ip_hex
    }
}