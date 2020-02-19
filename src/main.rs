#![feature(proc_macro_hygiene)]
mod handler;
mod http;
mod logger;
mod router;

use router::Router;
use tokio::net::{TcpListener, TcpStream};

lazy_static::lazy_static! {
    static ref ROUTER: Router = {
        let mut router  = Router::new();
        router.add_path("/hello/world", handler::hello_world);
        router.add_path("/hello", handler::hello);
        router.add_path("/", handler::index);
        router.add_path("/main.css", handler::stylesheet);
        router
    };
}

async fn process(mut stream: TcpStream) -> Result<(), failure::Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = [0u8; 1024];
    let len = stream.read(&mut buf).await?;
    if let Some(request) = http::Request::parse(&buf[..len]) {
        log::info!("REQUEST: {:?}", request.request_line());
        let uri = request.uri()?;
        let response = if let Some(hande) = ROUTER.route(uri) {
            log::info!("ROUTE: {}", uri);
            hande(uri)
        } else {
            handler::not_found(uri)
        };
        stream.write(response.to_string().as_bytes()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    use std::net::Ipv4Addr;
    use tokio::stream::StreamExt;

    color_backtrace::install();
    log::set_logger(&logger::Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let addr = (Ipv4Addr::new(127, 0, 0, 1), 8080);
    log::info!("Listening on: {:?}", addr);
    let mut listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    loop {
        let stream = incoming.next().await.unwrap()?;
        tokio::spawn(process(stream));
    }
}
