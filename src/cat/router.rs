use std::cell::Cell;

use async_std::net::TcpStream;
use rand::Rng;

use super::common::{ServerAddress, self};

pub struct CatRouterConfig {
    server_addresses: Vec<ServerAddress>,
    current: usize,
    url_suffix: String,
    sample: Cell<f32>,
    connection: Option<TcpStream>
}

impl CatRouterConfig {
    
    pub async fn update_router_config(&mut self) {
        let url = self.get_server_config_url();
        if let None = url {
            return;
        }
        let response = reqwest::blocking::get(url.unwrap());

        if let Ok(response) = response {
            if let Ok(response_text) = response.text() {
                // let property_config: RouterConfigXml = serde_xml_rs::from_str(&response_text).unwrap();
                // for property in property_config.properties {
                //     match property.id.as_str() {
                //         PROPERTY_SAMPLE => {
                //             self.update_sample(property.value);
                //         }
                //         PROPERTY_ROUTERS => {
                //             self.update_router(property.value).await;
                //         }
                //         PROPERTY_BLOCK => {

                //         }
                //         _ => {}
                //     }
                // }
            } else {
                // self.connect_holder.change_active_index();
            }
        }
    }

    async fn update_router(&mut self, router: String) {
        let new_routers = common::resolve_server_addresses(router);
        let new_len = new_routers.len();
        let old_len = self.server_addresses.len();

        if new_len == 0 {
            return;
        } else if old_len == 0 {
            self.server_addresses = new_routers;
            println!("router change");
        } else if old_len != new_len {
            self.server_addresses = new_routers;
            println!("router change");
        } else {
            for i in 0..old_len {
                if !common::compare_server_address(&new_routers[i], &self.server_addresses[i]) {
                    self.server_addresses = new_routers;
                    println!("router change");
                    break;
                }
            }
        }
        for (index, new_server) in self.server_addresses.iter().enumerate() {
            if index == self.current {
                if let Some(current_server) = self.server_addresses.get(index) {
                    if common::compare_server_address(current_server, new_server) {
                        return;
                    }  
                }
            } else {
                if let Ok(connection) = TcpStream::connect(format!("{}:{}", new_server.get_ip(), new_server.get_port())).await {
                    let old_connection = self.connection.as_mut();
                    self.connection = Some(connection);
                    
                }
                let a = self.connection.as_mut().unwrap();
                let v = a.shutdown(std::net::Shutdown::Both);
            }
        }
    }


    fn get_server_config_url(&self) -> Option<String> {
        let mut rng = rand::thread_rng();
        let index: usize = rng.gen_range(0..=self.server_addresses.len());
        if let Some(server) = self.server_addresses.get(index) {
            let mut http_port = server.get_http_port();
            if http_port == 0 {
                http_port = 8080;
            }
            let url = format!("http://{}:{}{}", server.get_ip(), http_port, self.url_suffix);
            return Some(url);
        }
        None
    }

    fn update_sample(&mut self, sample: String) {
        match sample.parse::<f32>() {
            Ok(number) => {
                self.sample.set(number);
            }
            Err(_) => {
            }
        }
    }

    fn get_sample(&self) -> f32 {
        self.sample.get()
    }
}