use std::error::Error;

#[cfg(feature = "daemon")]
type ReqError = Box<dyn Error + Send + Sync>;

#[cfg(not(feature = "daemon"))]
type ReqError = Box<dyn Error>;

//This is realisation of mini ureq
pub fn parse_status_code(status: &str) -> Result<u16, ReqError> {
    let mut parts = status.split_whitespace();
    let _http_ver = parts.next().ok_or("Missing HTTP version")?;
    let status_code = parts.next().ok_or("Missing status code")?;
    Ok(status_code.parse()?)
}

#[cfg(not(feature = "daemon"))]
pub fn send_http_request(addr: &str, request: &str) -> Result<String, ReqError> {
    use std::io::{Read, Write};
    use std::net::TcpStream;

    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    Ok(response)
}

#[cfg(not(feature = "daemon"))]
pub fn get_ip(proxy: Option<&str>) -> Result<String, ReqError> {
    let (addr, request) = build_ip_request(proxy);

    let response = send_http_request(&addr, &request)?;

    parse_ip_response(&response)
}

#[cfg(feature = "daemon")]
pub async fn send_http_request(addr: &str, request: &str) -> Result<String, ReqError> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpStream;

    let mut stream = TcpStream::connect(addr).await?;
    stream.write_all(request.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response = String::new();
    stream.read_to_string(&mut response).await?;

    Ok(response)
}

#[cfg(feature = "daemon")]
pub async fn get_ip(proxy: Option<&str>) -> Result<String, ReqError> {
    let (addr, request) = build_ip_request(proxy);

    let response = send_http_request(&addr, &request).await?;

    parse_ip_response(&response)
}

fn build_ip_request(proxy: Option<&str>) -> (String, String) {
    match proxy {
        None => {
            let request = "GET / HTTP/1.1\r\n\
                           Host: api.ipify.org\r\n\
                           Connection: close\r\n\
                           \r\n"
                .to_string();

            ("api.ipify.org:80".to_string(), request)
        }

        Some(proxy_addr) => {
            let request = "GET http://api.ipify.org/ HTTP/1.1\r\n\
                           Host: api.ipify.org\r\n\
                           Connection: close\r\n\
                           \r\n"
                .to_string();

            (proxy_addr.to_string(), request)
        }
    }
}

fn parse_ip_response(response: &str) -> Result<String, ReqError> {
    let (status, rest) = response
        .split_once("\r\n")
        .ok_or("Can't read status line")?;

    let status_code = parse_status_code(status)?;

    if !(200..300).contains(&status_code) {
        return Err(format!("Server response unsuccessful: {status_code}").into());
    }

    let (_, body) = rest.split_once("\r\n\r\n").ok_or("Can't get body")?;

    Ok(body.trim().to_string())
}
