#![feature(proc_macro_hygiene)]
mod handler;
mod http;
mod logger;
mod router;

use dope::executor::{self, reactor, Executor};
use dope::net::{TcpListener, TcpStream};

use router::Router;

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

async fn process<'a>(
    _reactor: reactor::Handle,
    mut stream: TcpStream,
) -> Result<(), failure::Error> {
    use futures::{AsyncReadExt, AsyncWriteExt};

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
    stream.close().await?;
    Ok(())
}

async fn main_async<'a>(executor: &executor::Handle) -> Result<(), failure::Error> {
    let reactor = executor.reactor()?;
    use futures::StreamExt;

    let addr = "127.0.0.1:8080";
    let mut incoming = TcpListener::bind(reactor.clone(), addr)?.incoming();
    log::info!("Listening on: {}", addr);

    loop {
        let stream = incoming.next().await.unwrap()?;
        executor.spawn(process(reactor.clone(), stream))?;
    }
}

fn main() -> Result<(), failure::Error> {
    color_backtrace::install();
    log::set_logger(&logger::Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let executor = Executor::new()?;
    let handle = executor.handle();
    executor.block_on(main_async(&handle)).unwrap()?;

    Ok(())
}
