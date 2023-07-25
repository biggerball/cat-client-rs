use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub fn get_local_hostname() -> Option<String> {
    if let Ok(hostname_os_str) = hostname::get() {
        if let Ok(hostname_str) = hostname_os_str.into_string() {
            return Some(hostname_str);
        }
    }
    return None;
}

pub fn get_local_ip() -> Option<String> {
    if let Some(ip_addr) = local_ip::get() {
        return Some(ip_addr.to_string());
    }
    return None;
}

pub fn get_timestamp() -> i64 {
    return SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64;
}

pub fn get_ip_hex(ip: String) -> String {
    let items: Vec<&str> = ip.split(".").collect();
    let mut hex = String::new();
    for item in items {
        let int: u32 = item.parse().unwrap();
        let byte: i8 = int as i8;

        hex.push_str(format!("{:x}{:x}", ((byte as u8) >> 4 & 15), ((byte as u8) & 15)).as_str());
    }
    return hex;
}