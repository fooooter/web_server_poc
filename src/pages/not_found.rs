use std::collections::HashMap;
use tokio::net::*;
use tokio::io::ErrorKind;
use crate::util::send_response;

pub async fn not_found(mut stream: &mut TcpStream, headers: &HashMap<String, String>) -> Result<(), ErrorKind> {
    let content = String::from(
r#"<!DOCTYPE html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <link rel="stylesheet" href="main.css">
            <title>404</title>
        </head>
        <body>
            Requested content isn't found on the server.
        </body>"#
    );

    let response_headers = HashMap::from([
        ("Connection", "keep-alive"),
        ("Keep-Alive", "timeout=5, max=100"),
        ("Content-Type", "text/html; charset=utf-8")]);

    if let Err(e) = send_response(&mut stream, 404, Some(response_headers), Some(content)).await {
        return Err(e)
    }
    Ok(())
}