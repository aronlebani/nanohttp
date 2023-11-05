# nanohttp

`nanohttp` is a small library to parse http requests and build valid http responses.

## Example

```rust
use std::str::from_utf8;

use async_std::io::{ReadExt, WriteExt};
use async_std::net::{TcpListener, TcpStream};
use async_std::task;

use nanohttp::{Method, Status, Request, Response};

async fn handler(req: Request) -> Response {
    match req.path.uri.as_str() {
        "/" => match req.method {
            Method::GET => {
                Response::empty().status(Status::Ok)
            },
            _ => Response::empty().status(Status::NotAllowed),
        },
        "/hello" => match req.method {
            Method::GET => {
                let html = "<html><head><title>Hello, world!</title></head><body><h1>Hello, world!</h1></body></html>";
                Response::body(html, "text/html").status(Status::Ok)
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
