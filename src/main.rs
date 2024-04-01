mod requests;
mod util;
mod pages;
mod config;

use std::collections::HashMap;
use tokio::net::*;
use tokio::*;
use tokio::io::ErrorKind;
use requests::Request::{Get, Head, Options, Post};
use requests::*;
use crate::util::*;

async fn handle_connection(mut stream: TcpStream) -> Result<(), ErrorKind> {
    let mut request_string = String::new();

    receive_request(&mut stream, &mut request_string).await;

    match Request::parse_from_string(&request_string) {
        Ok(request) => {
            match request {
                Get {resource, params, headers} => handle_get(stream, &resource, &params, &headers).await,
                Head {resource, headers} => handle_head(stream, &resource, &headers).await,
                Post {resource, headers, data} => Ok(handle_post(stream, &resource, &headers, &data).await),
                Options {resource, headers} => handle_options(stream, &resource, &headers).await,
                _ => {
                    let response_headers = HashMap::from([
                        ("Accept", "GET, HEAD, POST, OPTIONS"),
                        ("Connection", "keep-alive"),
                        ("Keep-Alive", "timeout=5, max=100")]);

                    send_response(&mut stream, 405, Some(response_headers), None).await?;
                    Ok(())
                }
            }
        },
        Err(e) => {
            let response_headers = HashMap::from([
                ("Connection", "keep-alive"),
                ("Keep-Alive", "timeout=5, max=100")]);

            send_response(&mut stream, 400, Some(response_headers), None).await?;
            Err(e)
        }
    }
}

#[tokio::main]

async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (stream, _) = listener.accept().await?;
        spawn(async move {
            handle_connection(stream).await
        });
    }
}


