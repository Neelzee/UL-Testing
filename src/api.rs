use eyre::{Context, Result};
use reqwest::{Client, Response};

use crate::consts;

pub fn create_client() -> Client {
    Client::default()
}

pub async fn get_product(client: &Client, id: u32) -> Result<Response> {
    client
        .get(format!("{}{}", consts::_URL, id))
        .send()
        .await
        .wrap_err("Failed to get product")
}

pub async fn get_image(client: &Client, url: &str) -> Result<Response> {
    client
        .get(url)
        .send()
        .await
        .wrap_err("Failed to get product")
}
