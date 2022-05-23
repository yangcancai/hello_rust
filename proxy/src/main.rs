use hyper::{client::HttpConnector, client::Client, Uri, Request};
use futures::{TryFutureExt, TryStreamExt};
use hyper_proxy::{Proxy, ProxyConnector, Intercept};
use headers::Authorization;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let proxy = {
        let proxy_uri = "http://127.0.0.1:8080".parse().unwrap();
        let proxy = Proxy::new(Intercept::All, proxy_uri);
        // proxy.set_authorization(Authorization::basic("John Doe", "Agent1234"));
        let connector = HttpConnector::new();
        let proxy_connector = ProxyConnector::from_proxy(connector, proxy).unwrap();
        proxy_connector
    };

    // Connecting to http will trigger regular GETs and POSTs.
    // We need to manually append the relevant headers to the request
    let uri: Uri = "http://my-remote-website.com".parse().unwrap();
    let mut req = Request::get(uri.clone()).body(hyper::Body::empty()).unwrap();

    if let Some(headers) = proxy.http_headers(&uri) {
        req.headers_mut().extend(headers.clone().into_iter());
    }

   Ok(())
}
