use reqwest::Url;

pub async fn download(url: &Url) -> Result<Vec<u8>, ()> {
    let client = reqwest::Client::builder()
        .build()
        .unwrap();
    
    let body = client.get(url.to_string())
        .send()
        .await;

    Ok(body.unwrap().bytes().await.unwrap().to_vec())
}
