use sxurl::v2::encode;

fn main() {
    let url = "https://example.com/page";
    match encode(url) {
        Ok(sxurl) => println!("SXURL for {}: {}", url, sxurl),
        Err(e) => eprintln!("Error encoding URL: {}", e),
    }
}
