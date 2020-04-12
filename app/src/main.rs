use bytes::Bytes;
use hideout::util::Logger;
use tokio::net::{TcpListener, TcpStream};

const HEADER_SIZE: usize = 2048;

async fn prepare_buffer(stream: &mut TcpStream) -> Result<Bytes, failure::Error> {
    use bytes::BytesMut;
    use tokio::io::AsyncReadExt;

    let mut buffer = BytesMut::with_capacity(HEADER_SIZE);
    unsafe { buffer.set_len(HEADER_SIZE) };
    let len = stream.read(buffer.as_mut()).await?;
    unsafe { buffer.set_len(len) };
    Ok(buffer.freeze())
}

async fn process(
    context: hideout::model::Context,
    mut stream: TcpStream,
) -> Result<(), failure::Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let buffer = prepare_buffer(&mut stream).await?;

    let request = hideout::http::Request::parse(buffer)?;
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

    let response = app::controllers::Root::serve(request, context, &payload).await;
    stream.write(response.header.to_string().as_bytes()).await?;
    stream.write(&response.payload).await?;
    Ok(())
}

async fn _main() -> Result<(), failure::Error> {
    use tokio::stream::StreamExt;

    color_backtrace::install();
    log::set_logger(&Logger).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    let config = hideout::util::Config::from_file("config/config.yaml")?;
    let context = hideout::model::Context::new(config).await?;

    let addr = (std::net::Ipv4Addr::new(127, 0, 0, 1), 8080);
    log::info!("Listening on: {:?}", addr);
    let mut listener = TcpListener::bind(addr).await?;

    let mut incoming = listener.incoming();

    loop {
        let stream = incoming.next().await.unwrap()?;
        tokio::task::spawn_local(process(context.clone(), stream));
    }
}

fn main() -> Result<(), failure::Error> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    tokio::task::LocalSet::new().block_on(&mut rt, _main())?;

    Ok(())
}
