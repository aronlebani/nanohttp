# nanohttp

`nanohttp` is a small zero-dependency library to parse http requests and build valid http
responses.

It is intended purely as an implementation of the HTTP protocol, and therefore does not
handle things like routing, json serialization and deserialization, or building a HTTP server.
See the examples below for how you can use it in combination with a TCP server and a runtime
library such as [tokio](https://docs.rs/tokio/latest/tokio/) or
[async-std](https://docs.rs/async-std/latest/async_std/) to build a custom HTTP server.

This library is intended to abstract away the details of dealing with HTTP, without removing
the need to understand how HTTP works at a high level. For example there are a few helper
methods which will automatically set relevant headers. But for the most part, it is up to the
consumer of the library to ensure that the correct headers are set, and generally ensure that
the constructed HTTP response is valid. An example of this is ensuring that the `Location`
header is set when returning a `303` response code.

## Examples

Parse an incoming HTTP request.

```rust
use nanohttp::{Request, Method};

let req = "GET / HTTP/1.1\r\n";
let res = Request::from_string(req).unwrap();

assert_eq!(res.method, Method::GET);
assert_eq!(res.path.uri, "/");

```

Build a HTTP response, and convert it to a valid HTTP message.

```rust
use nanohttp::{Response, Status, Header};

let html = "<html><head></head><body><h1>Hello, world!</h1></body></html>";
let res = Response::body(html)
    .header(Header::new("Content-Type", "text/html"))
    .header(Header::new("Content-Length", &html.len().to_string()))
    .status(Status::Ok);

assert_eq!(res.to_string(), "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 61\r\n\r\n<html><head></head><body><h1>Hello, world!</h1></body></html>");

```

Use `nanohttp` to build a custom TCP server using only the
[async-std](https://docs.rs/async-std/latest/async_std/) crate as a dependency.

```rust
use std::str::from_utf8;

use async_std::io::{ReadExt, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::task;

use nanohttp::{Method, Status, Request, Response};

async fn handler(req: Request) -> Response {
    match req.path.uri.as_str() {
        "/" => match req.method {
            Method::GET => Response::empty().status(Status::Ok),
            _ => Response::empty().status(Status::NotAllowed),
        },
        "/hello" => match req.method {
            Method::GET => {
                let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
                Response::content(html, "text/html").status(Status::Ok)
            },
            _ => Response::empty().status(Status::NotAllowed),
        },
        _ => Response::empty().status(Status::NotFound),
    }
}

async fn handle_connection(mut connection: TcpStream) {
    let mut buffer = [0; 1024];

    connection.read(&mut buffer).await.unwrap();

    let req_text = from_utf8(&buffer).unwrap().trim_end_matches("\0");

    let req = Request::from_string(req_text).unwrap();
    let res = handler(req).await.to_string();

    let res_bytes = res.as_bytes();

    connection.write(res_bytes).await.unwrap();
    connection.flush().await.unwrap();
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    loop {
        let (connection, _) = listener.accept().await.unwrap();
        task::spawn(async move {
            handle_connection(connection).await;
        });
    }
}
```
