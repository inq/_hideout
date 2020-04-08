use hideout::Logger;
use tokio::net::{TcpListener, TcpStream};

const HEADER_SIZE: usize = 2048;

async fn process(mut stream: TcpStream) -> Result<(), failure::Error> {
    use bytes::BytesMut;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let mut bytes = BytesMut::with_capacity(HEADER_SIZE);
    unsafe { bytes.set_len(HEADER_SIZE) };
    let len = stream.read(bytes.as_mut()).await?;
    unsafe { bytes.set_len(len) };

    let request = hideout::http::Request::parse(bytes.freeze())?;
    let payload = if let Some(content_length) = request.content_length() {
        let mut payload = Vec::with_capacity(content_length);
        payload.extend_from_slice(&request.body);
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

    let response = app::controllers::Root::serve(request, &payload).await;
    stream.write(response.header.to_string().as_bytes()).await?;
    stream.write(&response.payload).await?;
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
    let config = hideout::Config::from_file("config/config.yaml")?;

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
