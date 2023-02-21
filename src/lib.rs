mod handlers;

use std::net::TcpListener;
use axum::{routing::get, Router};
use hyper::server::conn::AddrIncoming;

use handlers::health_check;

// Create a new server with a given listener and pass it back to the caller
//
// In the Zero2Prod example, the server is relatively self-contained, so the return type is just
// `Result<Server, std::io::Error>`. Here, we run into axum being a thin wrapper around hyper; in
// fact, `axum::Server` is just a re-export of `hyper::Server`. Hyper's `server` has three
// trait-bound generics, and for two we need to specify types. The first is a type from hyper, so
// we must add hyper as a dependency as well.
//
// Note also that we are returning a server, not a `Result`.
pub fn run(
    listener: TcpListener,
) -> axum::Server<AddrIncoming, axum::routing::IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(health_check));

    axum::Server::from_tcp(listener).unwrap().serve(app.into_make_service())
}
