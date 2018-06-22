extern crate tokio;

use tokio::net::TcpListener;

fn main() {
    let addr = "127.0.0.1:1337".parse().unwrap();
    let listener = TcpListener::bind(&addr).expect("Unable to bind to address.");

    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            let (reader, writer) = sock.split();
            // TODO: implement ServerListPing
        });

    Tokio::run(server);
}