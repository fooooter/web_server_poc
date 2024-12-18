## Progress done so far (and TODO in the future)
[✔]   	GET<br>
[✔]   	OPTIONS<br>
[✔]   	HEAD<br>
[✔]   	POST<br>
[✔]	Auto-detect MIME types<br>
[✖]		Cookies<br>
[✖]		Sessions<br>
[✔]	Config (now JSON)<br>
[✔]     Compression (GZIP and Brotli for now)<br>
[✔]     Decompression (GZIP and Brotli for now)<br>
[✔]     TLS<br>
[✔]	Redirections<br>
[✖]     HTTP/2<br>
[✖]     HTTP/3<br>


## This project aims to be similar to PHP/React.js, mainly in terms of dynamically generated web pages.

Dynamic pages are generated inside a dynamic library, so that it's easy to create them without modifying
the core and recompiling the server only to change one thing on a page.

## Build

To build Drain, run `cargo build` in the root of a source.

## Configuration

Drain can be configured using config.json file. In order to use a config.json file, you have to specify it in `DRAIN_CONFIG` environment variable. 
Currently available fields are:

- `global_response_headers` - it's a list of key-value pairs, which stand for default response headers appended to every
`response_headers` HashMap.
- `access_control`:
  * `list` - here you can control, which resources will be returned to the client and which won't through a list of key-value pairs. 
  In order to deny access to a resource, type "deny" (default action is "allow"). It uses Glob UNIX shell-like path syntax.
  * `deny_action` - it's an unsigned integer corresponding to either 404 or 403 HTTP status codes, which will be returned by the server alongside the 
  page corresponding to each status if access to the resource is denied. For safety reasons, the default is 404, so that a client won't
  know if the resource is unavailable or access to it is denied.
- `bind_host` - bind host to the server.
- `bind_port` - bind port to the server (HTTP). If you want to use 80, be sure to start the server as root or other another privileged user.
- `dynamic_pages` - holds a list of every dynamic page available, so if you create one, be sure to specify it here!
- `dynamic_pages_library` - a path to the dynamic library for dynamic pages, which must be relative to the `server_root`.
- `encoding`:
  * `enabled` - enable response body encoding.
  * `supported_encodings` - a list of all compression algorithms supported by the server. It can currently contain only "gzip" and "br".
  * `use_encoding` - a name of encoding which will be used to compress the response body. It should be present in `supported_encodings`, otherwise the server will return uncompressed data.
  * `encoding_applicable_mime_types` - a list of media types to which encoding should be applied. It's best to leave this setting as is.
- `document_root` - a directory in which documents/files returned to the client are stored. Makes for the root of a URL.
- `server_root` - a directory in which server data are kept, like, for example, key-pairs.
- `https`:
  * `enabled` - enable HTTPS.
  * `bind_port` - bind port to the server (HTTPS). If you want to use 443, be sure to start the server as root or another privileged user.
  * `min_protocol_version` - a minimum version of TLS/DTLS/SSL the server accepts. Must be one of the following: 
    + SSL3
    + TLS1.3
    + TLS1
    + DTLS1
    + DTLS1.2
    + TLS1.1
    + TLS1.2
    
    Instead, it will be set to accept every protocol.
  * `cipher_list` - a colon-separated list of ciphers the server will use. Must be one of the following:
    + TLS_AES_128_GCM_SHA256
    + TLS_AES_256_GCM_SHA384
    + TLS_CHACHA20_POLY1305_SHA256
    + TLS_AES_128_CCM_SHA256
    + TLS_AES_128_CCM_8_SHA256
    + TLS_SHA384_SHA384 - integrity-only
    + TLS_SHA256_SHA256 - integrity-only
  
    Instead, the default configuration will be used: `TLS_AES_256_GCM_SHA384:TLS_CHACHA20_POLY1305_SHA256:TLS_AES_128_GCM_SHA256`
  * `ssl_private_key_file` - a path to the private key file in PEM format (a necessary field once HTTPS is enabled). 
  A path to it must be relative to the `server_root`.
  * `ssl_certificate_file` - a path to the certificate file in PEM format (a necessary field once HTTPS is enabled). 
  The certificate must match the private key and a path to it must be relative to the `server_root`.

Drain must be restarted in order for changes to take effect.

## Usage

### Template

It's strongly advised to use a template - https://github.com/fooooter/drain_page_template
(mainly because the default 404 and 403 pages are defined inside this template)

### Macros

Stick to the macro library if possible - https://github.com/fooooter/drain_macros

### Structure

Each page should be a Rust module defined in a separate file, declared in lib.rs and have the following structure:

```rust
use std::collections::HashMap;
use drain_common::RequestData::{self, *};
use drain_macros::*;

#[export_name = "index"]
#[drain_page]
pub fn index() -> Option<Vec<u8>> {
    let content: Vec<u8> = Vec::from(format!(r#"
    <!DOCTYPE html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Index</title>
        </head>
        <body>
            Hello, world! {} request was sent.
        </body>
    </html>"#, match request_data {
        Get {..} => "GET",
        Post {..} => "POST",
        Head {..} => "HEAD"
    }));

    header!("Content-Type", "text/html; charset=utf-8");

    Some(content)
}
```

### Naming convention

You can name these pages however you like, but keep in mind there's a naming convention.
`::` is a separator, which simulates a directory structure (it's equivalent to `/`, but for the sake of being accepted by the linker
unlike `/`, it was used instead of that). Don't worry, in config.json and in URL bar, it's still `/`, but it's converted to `::` automatically
by the server. At first, it can be easily confused with module paths, but take `app::login` as an example. In this case, 
`app` corresponds to a module, in which a submodule `login` containing a function (page) `login` resides, but as the page is named identically
to the module it resides in, it's just `app::login` instead of `app::login::login`. It's equivalent to the following resource specifier: `app/login`.

### RequestData

`RequestData` is a struct-like Enum, which has variants, that tell, what kind of HTTP request method was used and stores
request headers and data specific to each variant.

```rust
pub enum RequestData<'a> {
    Get {params: &'a Option<HashMap<String, String>>, headers: &'a HashMap<String, String>},
    Post {headers: &'a HashMap<String, String>, data: &'a Option<HashMap<String, String>>},
    Head {headers: &'a HashMap<String, String>}
}
```

POST `data` is an application/x-www-form-urlencoded string parsed to a HashMap and GET
`params` are regular key-value pairs sent in the URL. `headers`, however is a HashMap containing every request header field.

### `response_headers`

`response_headers` is a HashMap containing every header, that will be sent in response. It's a mutable reference,
so that you can simply append a header to existing ones. Its best use cases are redirections using `Location` header and
changing content type to JSON, for example. `Content-Type` header must be set explicitly, otherwise an empty page will be returned.
You should use the `header!` macro whenever possible.

### Redirections

Redirections are done once you append the `Location` or `location` header to `response_headers` in a dynamic page. 
It's up to you, whether a page should return content in redirection response or not, but it's preferred to 
return `None` after specifying `Location` (or `location`). The status code is set by default to 302, but this will be changeable very soon.