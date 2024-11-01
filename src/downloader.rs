use reqwest::Url;

pub async fn download(url: &Url) -> Result<Vec<u8>, reqwest::Error> {
    let client = reqwest::Client::builder()
        .no_proxy() // This fixes localhost connections... for some reason (https://github.com/seanmonstar/reqwest/issues/913)
        .build()?;
    
    let body = client.get(url.to_string())
        .send()
        .await;

    Ok(body?.bytes().await?.to_vec())
}
