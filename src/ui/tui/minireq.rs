use std::{
    error::Error,
    io::{Read, Write},
    net::TcpStream,
};
//This is realisation of mini ureq 
pub fn parse_status_code(status: &str) -> Result<u16, Box<dyn Error>> {
    let mut parts = status.split_whitespace();
    let _http_ver = parts.next().ok_or("Missing HTTP version")?;
    let status_code = parts.next().ok_or("Missing status code")?;
    Ok(status_code.parse()?)
}

pub fn send_http_request(addr: &str, request: &str) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect(addr)?;
    stream.write_all(request.as_bytes())?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}

// Don't remember adding auto port func 
pub fn get_ip(proxy: Option<&str>) -> Result<String, Box<dyn Error>> {
    let (addr, request) = match proxy {
        None => {
            let request =
                "GET / HTTP/1.1\r\n\
                 Host: api.ipify.org\r\n\
                 Connection: close\r\n\
                 \r\n"
                    .to_string();

            ("api.ipify.org:80".to_string(), request)
        }
        Some(proxy_addr) => {
            let request =
                "GET http://api.ipify.org/ HTTP/1.1\r\n\
                 Host: api.ipify.org\r\n\
                 Connection: close\r\n\
                 \r\n"
                    .to_string();

            (proxy_addr.to_string(), request)
        }
    };

    let response = send_http_request(&addr, &request)?;

    let (status, rest) = response
        .split_once("\r\n")
        .ok_or("Can't read status line")?;

    let status_code = parse_status_code(status)?;
    if !(200..300).contains(&status_code) {
        return Err(format!("Server response unsuccessful: {status_code}").into());
    }

    let (_, body) = rest
        .split_once("\r\n\r\n")
        .ok_or("Can't get body")?;

    Ok(body.trim().to_string())
}