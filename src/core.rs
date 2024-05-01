use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use crate::config::{Cfg, ConfigDyn, ConfigPer};
use crate::database::Database;
use crate::handler;
use crate::utils::consts::VERSION;
use crate::utils::{console, logs};

#[cfg(feature = "tls")]
use crate::tls::*;

/// Warmup function is called once at the start of the program.
/// It's used to allocate all global resources.
async fn warmup() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    logs::init();
    ConfigPer::init().await?;
    ConfigDyn::init().await?;
    Database::get().await?;
    console::Engine::init().await;

    Ok(())
}

/// The "real" entry point.
pub(super) async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    warmup().await?;

    let cfg = ConfigPer::get().await.unwrap();

    // on the debug mode it might be more performant to skip initializing the Resources as they are not used every time.
    #[cfg(not(debug_assertions))] {
        handler::reload_resource_map().await?;
    }

    let listener = TcpListener::bind(&cfg.addr_server).await?;
    info!("Running the HTTP server [{}] on: {}", VERSION, cfg.addr_server);

    #[cfg(feature = "cloudflare")]
    let acceptor_cf = cloudflare::TlsAcceptorCF::init()
        .expect("Failed to initialize the Cloudflare TLS!");

    #[cfg(not(feature = "tls"))]
    warn!("TLS is disabled!");

    // Main program loop.
    // todo: add graceful shutdown
    loop {
        let (stream, addr) = listener.accept().await?;
        debug!("[{}] new connection", addr);

        #[cfg(not(feature = "cloudflare"))] {
            use hyper::server::conn::http1;

            let io = TokioIo::new(stream);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(|req| async move {
                        handler::service(req, addr).await
                    }))
                    .await
                {
                    error!("Error serving connection: {:?}", err);
                }
            });
        }

        #[cfg(feature = "cloudflare")] {
            use hyper_util::server::conn::auto::Builder;
            use hyper_util::rt::TokioExecutor;

            let acceptor = acceptor_cf.clone();

            tokio::spawn(async move {
                let tls_stream = match acceptor.accept(stream).await {
                    Ok(tls_stream) => tls_stream,
                    Err(err) => {
                        #[cfg(debug_assertions)]
                        error!("Failed to perform a TLS handshake: {:#?}", err);

                        // to disable warning on release build
                        drop(err);

                        return;
                    }
                };

                if let Err(err) = Builder::new(TokioExecutor::new())
                    .serve_connection(TokioIo::new(tls_stream), service_fn(|req| async move {
                        handler::service(req, addr).await
                    }))
                    .await
                {
                    error!("Error serving connection: {:#?}", err);
                }
            });
        }
    }

    Ok(())
}
