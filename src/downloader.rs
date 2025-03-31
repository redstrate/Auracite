use reqwest::Url;

pub async fn download(url: &Url) -> Result<Vec<u8>, reqwest::Error> {
    let mut client = reqwest::Client::builder();

    #[cfg(not(target_family = "wasm"))]
    {
        client = client.no_proxy(); // This fixes localhost connections... for some reason (https://github.com/seanmonstar/reqwest/issues/913)
    }

    let client = client.build()?;

    let body = client.get(url.to_string()).send().await;

    Ok(body?.bytes().await?.to_vec())
}
