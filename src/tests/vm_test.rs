use iflet::if_chain;
use reqwest::{Client, Method};

#[tokio::test]
async fn whisky_transform() {
    let client = Client::new();

    if_chain!([let Ok(req) => client.request(Method::GET, "").build(),
        let Ok(res) => client.execute(req).await] {

    });
}
