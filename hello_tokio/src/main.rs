use anyhow::Result;
use rustls_pemfile::certs;
use std::{
    fs::File,
    io::{self, BufReader},
    path::Path,
    sync::Arc,
};
use tokio::{
    io::{copy, sink, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};
use tokio_rustls::{
    rustls::{self, Certificate, PrivateKey},
    TlsAcceptor,
};
mod tls_client;
fn load_certs(path: &Path) -> io::Result<Vec<Certificate>> {
    certs(&mut BufReader::new(File::open(path)?))
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "invalid cert"))
        .map(|mut certs| certs.drain(..).map(Certificate).collect())
}

fn load_keys(path: &Path) -> io::Result<Vec<PrivateKey>> {
    let private_key_bytes = std::fs::read(path).expect("ca private key file path not valid!");
    let private_key = rustls_pemfile::pkcs8_private_keys(&mut private_key_bytes.as_slice())
        .expect("Failed to parse private key");
    let private_key = rustls::PrivateKey(private_key[0].clone());
    Ok(vec![private_key])
}
#[tokio::main]
async fn main() -> Result<()> {
    tls_client::req("www.google.com", 443, Some("ca/cacert.pem")).await?;
    let certs = load_certs(Path::new("ca/cacert.pem"))?;
    let mut keys = load_keys(Path::new("ca/cakey.pem"))?;

    let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, keys.remove(0))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))?;
    let acceptor = TlsAcceptor::from(Arc::new(config));
    let listener = TcpListener::bind("127.0.0.1:5666").await?;
    loop {
        let (mut stream, peer_addr) = listener.accept().await?;
        let acceptor = acceptor.clone();
        let fut = async move {
            // receive connection message, such as CONNECT method for https proxy protocol
            let mut buf = [0; 2];
            let _n = stream.read(&mut buf).await?;
            println!("recv = {:?}", buf);
            // tcp upgrade to tls
            let mut stream = acceptor.accept(stream).await?;
            let mut output = sink();
            stream
                .write_all(
                    &b"HTTP/1.0 200 ok\r\n\
                    Connection: close\r\n\
                    Content-length: 12\r\n\
                    \r\n\
                    Hello world!"[..],
                )
                .await?;
            stream.shutdown().await?;
            copy(&mut stream, &mut output).await?;
            println!("Hello: {}", peer_addr);
            Ok(()) as io::Result<()>
        };
        tokio::spawn(async move {
            if let Err(err) = fut.await {
                eprintln!("{:?}", err);
            }
        });
        // break;
    }
    Ok(())
}
