pub mod http;

use std::sync::Arc;

use clap::{command, Parser};
use env_logger::Env;
use log::{debug, error, warn};
use socks5_server::{
    auth::NoAuth,
    connection::state::NeedAuthenticate,
    proto::{Address, Error, Reply},
    Command, IncomingConnection,
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[derive(Parser, Debug)]
#[command(version, long_about = "")]
struct Args {
    #[arg(short, long, default_value = "0.0.0.0")]
    bind: String,

    #[arg(short, long, default_value = "1080")]
    port: String,

    #[arg(
        short('f'),
        long("user-agent"),
        default_value = "Mozilla/5.0 (Window NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/555.66"
    )]
    user_agent: String,

    #[arg(short('l'), long("log-level"), default_value = "info")]
    log_level: String,
}

fn main() {
    let args = Args::parse();
    start_server(args);
}

static mut USERAGENT: Option<String> = None;

#[tokio::main]
async fn start_server(args: Args) {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    unsafe {
        USERAGENT = Some(args.user_agent);
    }

    let listener = TcpListener::bind(format!("{}:{}", args.bind, args.port)).await;
    let listener = match listener {
        Ok(listener) => listener,
        Err(err) => {
            error!("bind error: {}", err);
            return;
        }
    };
    let auth: Arc<_> = Arc::new(NoAuth);

    let server = socks5_server::Server::new(listener, auth);

    while let Ok((conn, _)) = server.accept().await {
        tokio::spawn(async move {
            match handler(conn).await {
                Ok(()) => {}
                Err(err) => error!("handle err: {err}"),
            }
        });
    }
}

async fn handler(conn: IncomingConnection<(), NeedAuthenticate>) -> Result<(), Error> {
    let conn = match conn.authenticate().await {
        Ok((conn, _)) => conn,
        Err((err, mut conn)) => {
            let _ = conn.shutdown().await;
            return Err(err);
        }
    };

    match conn.wait().await {
        // we don't support associate and bind command
        Ok(Command::Associate(associate, _)) => {
            warn!("received associate command, reject");
            let replied = associate
                .reply(Reply::CommandNotSupported, Address::unspecified())
                .await;

            let mut conn = match replied {
                Ok(conn) => conn,
                Err((err, mut conn)) => {
                    let _ = conn.shutdown().await;
                    return Err(Error::Io(err));
                }
            };

            let _ = conn.close().await;
        }
        Ok(Command::Bind(bind, _)) => {
            warn!("received bind command, reject");
            let replied = bind
                .reply(Reply::CommandNotSupported, Address::unspecified())
                .await;

            let mut conn = match replied {
                Ok(conn) => conn,
                Err((err, mut conn)) => {
                    let _ = conn.shutdown().await;
                    return Err(Error::Io(err));
                }
            };

            let _ = conn.close().await;
        }

        Ok(Command::Connect(connect, addr)) => {
            let target = match addr {
                Address::DomainAddress(domain, port) => {
                    let domain = String::from_utf8_lossy(&domain);
                    TcpStream::connect((domain.as_ref(), port)).await
                }
                Address::SocketAddress(addr) => TcpStream::connect(addr).await,
            };

            match target {
                Ok(mut target) => {
                    let replied = connect
                        .reply(Reply::Succeeded, Address::unspecified())
                        .await;

                    let mut conn = match replied {
                        Ok(conn) => conn,
                        Err((err, mut conn)) => {
                            error!("reply failed: {}", err);
                            let _ = conn.shutdown().await;
                            return Err(Error::Io(err));
                        }
                    };

                    let mut buf: Vec<u8> = vec![0; 8];
                    let n = match conn.read(&mut buf).await {
                        Ok(n) => n,
                        Err(err) => {
                            let _ = conn.shutdown().await;
                            let _ = target.shutdown().await;

                            error!("read failed: {}", err);
                            return Err(Error::Io(err));
                        }
                    };
                    debug!("read {} bytes", n);
                    if n == 0 {
                        let _ = conn.shutdown().await;
                        let _ = target.shutdown().await;
                        return Ok(());
                    }
                    let is_http = http::is_http_request(&mut buf[..n]);
                    debug!("is_http: {}", is_http);
                    if is_http {
                        let user_agent = unsafe { USERAGENT.as_ref().unwrap() };

                        let mut buf: Vec<u8> = vec![0; 1018];
                        let n = match conn.read(&mut buf).await {
                            Ok(n) => n,
                            Err(err) => {
                                let _ = conn.shutdown().await;
                                let _ = target.shutdown().await;

                                error!("read failed: {}", err);
                                return Err(Error::Io(err));
                            }
                        };
                        if n == 0 {
                            let _ = conn.shutdown().await;
                            let _ = target.shutdown().await;
                            return Ok(());
                        }

                        http::modify_user_agent(&mut buf, user_agent);
                    }

                    debug!("buf len: {}", buf.len());

                    target.write(&buf[..buf.len()]).await?;
                    target.flush().await?;

                    let res = io::copy_bidirectional(&mut target, &mut conn).await;
                    let _ = conn.shutdown().await;
                    let _ = target.shutdown().await;

                    res?;
                }
                Err(err) => {
                    warn!("connect failed: {}", err);

                    let replied = connect
                        .reply(Reply::HostUnreachable, Address::unspecified())
                        .await;

                    let mut conn = match replied {
                        Ok(conn) => conn,
                        Err((err, mut conn)) => {
                            let _ = conn.shutdown().await;
                            return Err(Error::Io(err));
                        }
                    };

                    let _ = conn.shutdown().await;
                }
            }
        }

        Err((err, mut conn)) => {
            let _ = conn.shutdown().await;
            return Err(err);
        }
    };

    Ok(())
}
