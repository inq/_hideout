#![feature(proc_macro_hygiene)]
use core::{
    router::{self, Router},
    Logger,
};

mod handlers;

use tokio::net::{TcpListener, TcpStream};

lazy_static::lazy_static! {
    static ref ROUTER: Router = {
        use router::Handler;

        let mut router  = Router::new();
        router.add_path("/articles/:article_id", Handler::Arg1(handlers::article_show));
        router.add_path("/articles", Handler::Arg0(handlers::article_list));
        router.add_path("/", Handler::Arg0(handlers::index));
        router.add_path("/main.css", Handler::Arg0(handlers::stylesheet));
        router
    };
}

async fn process(mut stream: TcpStream) -> Result<(), failure::Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = [0u8; 1024];
    let len = stream.read(&mut buf).await?;
    if let Some(request) = core::http::Request::parse(&buf[..len]) {
        log::info!("REQUEST: {:?}", request.request_line());
        let uri = request.uri()?;
        let response = if let Some((handler, _args)) = ROUTER.route(uri) {
            use router::Handler;

            log::info!("ROUTE: {}", uri);
            match handler {
                Handler::Arg0(func) => func(),
                _ => panic!(),
            }
        } else {
            handlers::not_found(uri)
        };
        stream.write(response.to_string().as_bytes()).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), failure::Error> {
    use std::net::Ipv4Addr;
    use tokio::stream::StreamExt;

    // Log
    color_backtrace::install();
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    // Config
    let config = core::Config::from_file(".config.yaml")?;

    // Database
    let (_client, connection) =
        tokio_postgres::connect(&config.database_string(), tokio_postgres::NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let addr = (Ipv4Addr::new(127, 0, 0, 1), 8080);
    log::info!("Listening on: {:?}", addr);
    let mut listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();
    loop {
        let stream = incoming.next().await.unwrap()?;
        tokio::spawn(process(stream));
    }
}
