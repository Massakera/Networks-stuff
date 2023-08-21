use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    println!("Enter the URL (e.g., http://example.com/index.html): ");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).expect("Failed to read URL");

    let url = url.trim();
    if url.starts_with("http://") {
        let without_http = &url[7..];
        webget(without_http).expect("Failed to fetch web page");
    } else {
        webget(url).expect("Failed to fetch web page");
    }
}

fn webget(url: &str) -> std::io::Result<()> {
    let parts: Vec<&str> = url.splitn(2, '/').collect();
    let domain = parts[0];
    let path = if parts.len() > 1 { &parts[1] } else { "/" };

    let mut stream = TcpStream::connect(format!("{}:80", domain))?;
    let request = format!("GET /{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n", path, domain);
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{}", response);

    Ok(())
}
