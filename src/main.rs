#![feature(proc_macro_hygiene)]
mod http;
mod logger;
mod router;

use dope::executor::{self, reactor, Executor};
use dope::net::{TcpListener, TcpStream};

use router::Router;

fn handler_index(uri: &str) -> http::Response {
    let payload = format!(
        "{}",
        tent::html!(
            html
                span.hello "This is from MACRO!"
                .hello {format!("given uri: {}", uri)}
        )
    );
    http::Response::new_html(200, payload)
}

lazy_static::lazy_static! {
    static ref ROUTER: Router = {
        let mut builder  = router::Builder::new();
        builder.add_path("/hello", handler_index);
        builder.build()
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
        let uri = request.uri()?;
        let response = ROUTER.test_handle(uri);
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
