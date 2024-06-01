use error_chain::error_chain;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let tmp_dir = Builder::new().prefix("image-download").tempdir()?;
    
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let res = reqwest::get(target).await?;

    let mut dest = {
        let fname = res
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("Full file path: '{:?}'", fname);

        File::create(fname)?
    };

    let content = res.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;
    
    Ok(())
}
