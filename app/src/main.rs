#![feature(proc_macro_hygiene)]
use core::{
    http::{Request, Response},
    router::{self, Router},
    AssetStore, Logger,
};

mod handlers;
mod models;

use tokio::net::{TcpListener, TcpStream};

lazy_static::lazy_static! {
    static ref ROUTER: Router = {
        use router::Handler;

        let mut asset_store = AssetStore::new();
        let asset0 = asset_store.add("assets/raleway-light.woff", "font/woff").unwrap();

        let mut router  = Router::new(asset_store);
        router.add_get("/assets/raleway-light.woff", Handler::Resource(asset0));
        router.add_get("/articles/:article_id", Handler::Arg1(handlers::article_show));
        router.add_get("/articles/list", Handler::Arg0(handlers::article_list));
        router.add_get("/session/new", Handler::Arg0(handlers::session_new));
        router.add_post("/session/create", Handler::Arg0(handlers::session_create));
        router.add_get("/", Handler::Arg0(handlers::index));
        router.add_get("/main.css", Handler::Arg0(handlers::stylesheet));
        router
    };
}

const HEADER_SIZE: usize = 2048;

fn handle_request(request: Request, payload: &[u8]) -> Result<Response, failure::Error> {
    log::info!(
        "REQUEST: {:?} ({})",
        request.request_line(),
        std::str::from_utf8(payload)?
    );
    let uri = request.uri()?;
    let res = if let Some((handler, args)) = ROUTER.route(request.method(), uri) {
        use router::{Args, Handler};

        match (handler, args) {
            (Handler::Resource(id), Args::Arg0) => ROUTER.asset_store.serve(id),
            (Handler::Arg0(func), Args::Arg0) => func(payload),
            (Handler::Arg1(func), Args::Arg1(arg0)) => func(payload, arg0),
            (Handler::Arg2(func), Args::Arg2(arg0, arg1)) => func(payload, arg0, arg1),
            _ => panic!(),
        }
    } else {
        handlers::not_found(uri)
    };
    Ok(res)
}

async fn process(mut stream: TcpStream) -> Result<(), failure::Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut buf = [0u8; HEADER_SIZE];
    let len = stream.read(&mut buf).await?;
    if let Some(request) = core::http::Request::parse(&buf[..len]) {
        let payload = if let Some(content_length) = request.content_length() {
            let mut payload = Vec::with_capacity(content_length);
            payload.extend_from_slice(request.body);
            let offset = payload.len();
            log::info!("Reading content: Content-Length: {}", content_length);
            let len = (&mut stream)
                .take((content_length - offset) as u64)
                .read_to_end(&mut payload)
                .await?;
            assert!(len == content_length);
            payload
        } else {
            vec![]
        };

        let response = handle_request(request, &payload)?;
        stream.write(response.header.to_string().as_bytes()).await?;
        stream.write(&response.payload).await?;
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
    log::info!("\n{}", (*ROUTER).to_debug());
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
