use std::env;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub api_address: SocketAddr,
    pub postgres_url: String,
    pub postgres_pool_size: u32,
}

impl Configuration {
    pub fn dev() -> Result<Configuration, Box<dyn std::error::Error>> {
        Ok(Configuration {
            api_address: "[::1]:50051".parse()?,
            postgres_url: String::from("postgres://postgres:mysecretpassword@localhost/postgres"),
            postgres_pool_size: 5,
        })
    }

    pub fn env() -> Result<Configuration, Box<dyn std::error::Error>> {
        let api_address = env::var("API_ADDRESS")?;
        let postgres_url = env::var("POSTGRES_URL")?;
        let postgres_pool_size = env::var("POSTGRES_POOL_SIZE")?;

        Ok(Configuration {
            api_address: api_address.parse()?,
            postgres_url,
            postgres_pool_size: postgres_pool_size.parse()?,
        })
    }
}