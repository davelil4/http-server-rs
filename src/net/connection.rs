use std::{io, net::TcpStream, net::SocketAddr, u8};

/// Configure a newly accepted TCP stream.
///
/// This function is responsible for setting socket options such as
/// timeouts or TCP_NODELAY.
fn configure_stream(stream: &TcpStream) -> io::Result<()> {
    unimplemented!()
}

/// Return the peer socket address for a connected stream.
pub fn peer_addr(stream: &TcpStream) -> io::Result<SocketAddr> {
    stream.peer_addr()
}

/// Handle the full lifecycle of a single TCP connection.
///
/// This function owns:
/// - reading bytes from the stream
/// - buffering until a complete request is available
/// - invoking `on_message` to produce a response
/// - writing response bytes back to the stream
/// - closing the connection
///
/// The `on_message` callback represents protocol-level logic
/// (e.g., HTTP parsing and response generation).
///
/// # Phase 1 behavior
/// - Reads exactly one request
/// - Calls `on_message` once
/// - Writes one response
/// - Closes the connection
///
/// # Later phases
/// Can evolve to support keep-alive and multiple requests
/// without changing the external API.
pub fn handle_connection(
    stream: TcpStream,
    mut on_message: impl FnMut(&[u8]) -> io::Result<Vec<u8>>,
) -> io::Result<()> {
    unimplemented!()
}

/// Shut down a TCP stream in the specified direction.
pub fn shutdown(stream: TcpStream, how: std::net::Shutdown) -> io::Result<()> {
    unimplemented!()
}
