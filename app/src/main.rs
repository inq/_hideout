use bytes::Bytes;
use hideout::{
    http,
    util::{config, Logger},
};
use tokio::net::{TcpListener, TcpStream};

const HEADER_SIZE: usize = 2048;

#[derive(Debug)]
enum Error {
    SetLogger(log::SetLoggerError),
    StreamExhausted,
    Io(std::io::Error),
    Config(config::Error),
    Database(mongodb::error::Error),
    HttpRequest(http::request::Error),
}

async fn prepare_buffer(stream: &mut TcpStream) -> Result<Bytes, Error> {
    use bytes::BytesMut;
    use tokio::io::AsyncReadExt;

    let mut buffer = BytesMut::with_capacity(HEADER_SIZE);
    unsafe { buffer.set_len(HEADER_SIZE) };
    let len = stream.read(buffer.as_mut()).await.map_err(Error::Io)?;
    unsafe { buffer.set_len(len) };
    Ok(buffer.freeze())
}

fn unwrap_response(response: http::Result<http::Response>) -> http::Response {
    match response {
        Ok(res) => res,
        Err(err) => match err {
            http::Error::Unauthorized { uri } => app::handlers::unauthorized(&uri),
            http::Error::NotFound { uri } => app::handlers::not_found(&uri),
        },
    }
}

async fn process(state: app::ServerState, mut stream: TcpStream) -> Result<(), Error> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let buffer = prepare_buffer(&mut stream).await?;

    let request = hideout::http::Request::parse(buffer).map_err(Error::HttpRequest)?;
    let context = app::Context::new(state, request);

    let payload = if let Some(content_length) = context.request.content_length() {
        let mut payload = Vec::with_capacity(content_length);
        payload.extend_from_slice(&context.request.body);
        let offset = payload.len();
        log::info!("Reading content: Content-Length: {}", content_length);
        let len = (&mut stream)
            .take((content_length - offset) as u64)
            .read_to_end(&mut payload)
            .await
            .map_err(Error::Io)?;
        assert!(len == content_length, "{}, {}", content_length, len);
        payload
    } else {
        vec![]
    };

    let response = app::controllers::Root::serve(context, &payload).await;
    let response = unwrap_response(response);
    stream
        .write(response.header.to_string().as_bytes())
        .await
        .map_err(Error::Io)?;
    stream.write(&response.payload).await.map_err(Error::Io)?;
    Ok(())
}

async fn _main() -> Result<(), Error> {
    use tokio::stream::StreamExt;

    color_backtrace::install();
    log::set_logger(&Logger).map_err(Error::SetLogger)?;
    log::set_max_level(log::LevelFilter::Debug);

    let config = hideout::util::Config::from_file("config/config.yaml").map_err(Error::Config)?;
    let state = app::ServerState::new(config)
        .await
        .map_err(Error::Database)?;

    let addr = (std::net::Ipv4Addr::new(127, 0, 0, 1), 8080);
    log::info!("Listening on: {:?}", addr);
    let mut listener = TcpListener::bind(addr).await.map_err(Error::Io)?;

    let mut incoming = listener.incoming();

    loop {
        let stream = incoming
            .next()
            .await
            .ok_or(Error::StreamExhausted)?
            .map_err(Error::Io)?;
        tokio::task::spawn_local(process(state.clone(), stream));
    }
}

fn main() -> Result<(), Error> {
    let mut rt = tokio::runtime::Runtime::new().map_err(Error::Io)?;
    tokio::task::LocalSet::new().block_on(&mut rt, _main())?;

    Ok(())
}
