use eyre::{Context, Error, Result};
use reqwest::Client;
use serde_json::Value;
use std::time::Duration;
use tokio::time::sleep;

use crate::{
    request::{get_ids, get_whisky_data},
    utils::{consts::TEST_REQUEST_RATE_LIMIT, funcs::get_url},
};

#[tokio::test]
async fn test_pages() {
    let client = Client::new();

    for i in 0..53 {
        match client.get(get_url(i)).build() {
            Ok(req) => {
                let res = client.execute(req).await;
                //sleep(Duration::from_secs_f32(TEST_REQUEST_RATE_LIMIT)).await; // Rate limit for testing
                if res.is_err() {
                    panic!("Failed with code: {:?}", res.err());
                } else {
                    println!("Page: {}", i);
                    assert_eq!(res.unwrap().status(), reqwest::StatusCode::OK);
                }
            }
            Err(err) => panic!("Failed req: {:?}", err),
        }
    }
}

#[tokio::test]
async fn test_scraper() {
    let client = Client::new();
    for i in 0..53 {
        match client.get(get_url(i)).build() {
            Ok(req) => {
                let res = client.execute(req).await;
                sleep(Duration::from_secs_f32(TEST_REQUEST_RATE_LIMIT)).await; // Rate limit for testing
                if res.is_err() {
                    panic!("Failed with code: {:?}", res.err());
                } else {
                    println!("Page: {}", i);
                    let response = res.unwrap();
                    let status = response.status().clone();
                    assert_eq!(status, reqwest::StatusCode::OK);

                    let body: Result<_> = response
                        .text()
                        .await
                        .wrap_err("Failed retrieving body")
                        .and_then(|e| {
                            serde_json::from_str::<Value>(&e).wrap_err("Failed parsing json")
                        })
                        .and_then(|val| Ok(get_ids(&client, i)));

                    if let Ok(f) = body {
                        if let Ok(ids) = f.await {
                            assert!(ids.len() != 0);
                        }
                    }
                }
            }
            Err(err) => panic!("Failed req: {:?}", err),
        }
    }
}

#[tokio::test]
async fn test_whisky_getter() {
    let client = Client::default();

    if let Ok(mut ids) = get_ids(&client, 0).await {
        if let Some(raw_id) = ids.pop() {
            if let Ok(id) = raw_id.parse::<u32>() {
                let whisky = get_whisky_data(&client, id).await;
                assert!(whisky.is_ok());
                let res = whisky.unwrap();
                println!("{:?}", res);
            }
        }
    }
}
