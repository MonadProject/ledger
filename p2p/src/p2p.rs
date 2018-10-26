use futures::stream::Stream;
use std::net::SocketAddr;
use tokio::io;
use tokio::prelude::*;
use tokio_core::net::TcpListener;
use tokio_core::reactor::{Handle, Interval, Remote, Timeout};
use tokio_io::io::{read_exact, ReadExact};

fn start_server(address: SocketAddr, handle: &Handle) {
    let socket = TcpListener::bind(&address, handle).unwrap();
    println!("Listening on: {}", address);
    //答应我，这是一个Future
    let server = socket.incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            Ok(())
        });
    //我查了，spawn 的中文意思是"卵" :)
    handle.spawn(server);
}


#[cfg(test)]
mod tests {
    use reactor;
    use std::net::SocketAddr;
    use super::start_server;

    #[test]
    fn test_start_serer() {
        let address = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
        let mut el = reactor::reactor();
        start_server(address, &el.handle());
        el.run(reactor::forever()).unwrap();
    }
}