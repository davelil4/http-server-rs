use std::io;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream, ToSocketAddrs};

/// Bind a TCP listener to the given socket address.
///
/// This function is responsible only for creating and returning
/// a `TcpListener`. It does not accept connections or spawn handlers.
fn listen<A : ToSocketAddrs>(addr: A) -> io::Result<TcpListener> {
    TcpListener::bind(addr)
}

/// Run a blocking accept loop on a `TcpListener`.
///
/// For each accepted connection, `on_conn` is invoked with the
/// connected `TcpStream` and the peer address.
///
/// This function owns the lifetime of the accept loop but does not
/// define how individual connections are handled.
pub fn accept_loop(
    listener: TcpListener,
    mut on_conn: impl FnMut(TcpStream, SocketAddr) -> io::Result<()>,
) -> io::Result<()> {
    loop {
        let (stream, addr) = listener.accept()?;
        let _ = on_conn(stream, addr);
    }
}

/// Convenience function that binds a listener and immediately
/// enters the accept loop.
///
/// This is a thin wrapper around `listen` and `accept_loop`.
pub fn serve<A: ToSocketAddrs>(
    addr: A,
    // Fn because serve might go across multiple threads and this ensures handler can handle
    // this. handler must fit tighter constraints in threaded case
    mut on_conn: impl Fn(TcpStream, SocketAddr) -> io::Result<()> + Send + Sync + 'static,
) -> io::Result<()> {
    let listener = listen(addr)?;
    accept_loop(listener, on_conn)
}

