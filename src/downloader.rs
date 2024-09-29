use downloader::{Download, Downloader};
use std::path::Path;

pub fn download(url: &str, path: &Path) -> Result<(), ()> {
    let mut downloader = Downloader::builder().build().unwrap();

    let mut dl = Download::new(url);
    dl = dl.file_name(path);

    if !path.exists() {
        let result = downloader.download(&[dl]).unwrap();

        for r in result {
            return match r {
                Err(e) => {
                    println!("Error: {}", e.to_string());
                    Err(())
                }
                Ok(s) => Ok(()),
            };
        }
    }

    Ok(())
}
