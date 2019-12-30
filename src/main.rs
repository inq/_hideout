#![feature(proc_macro_hygiene)]
mod http;
mod logger;

use dope::executor::{self, reactor, Executor};
use dope::net::{TcpListener, TcpStream};
use dope::timer::Delay;

async fn process(reactor: reactor::Handle, mut stream: TcpStream) -> Result<(), failure::Error> {
    use futures::{AsyncReadExt, AsyncWriteExt};

    Delay::start(reactor, chrono::Duration::seconds(1))?.await?;

    log::warn!("SPAWNED");
    let mut buf = [0u8; 1024];
    let len = stream.read(&mut buf).await?;
    // log::debug!("{:?}", String::from_utf8(buf[..len].to_vec()));
    log::debug!("{:?}", http::Request::parse(&buf[..len]));

    let payload = "Hello, world!";
    let to_write = format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: Closed\r\n\r\n{}", payload.len(), payload);
    stream.write(to_write.as_bytes()).await?;

    stream.close().await?;

    Ok(())
}

async fn main_async(executor: &executor::Handle) -> Result<(), failure::Error> {
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

    println!(
        "{}",
        html_macro::html!(
            html
                span.hello
                    Hello
                .hello
        )
    );

    let executor = Executor::new()?;
    let handle = executor.handle();
    executor.block_on(main_async(&handle)).unwrap()?;

    Ok(())
}
