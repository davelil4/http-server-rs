use std::{io, net::TcpStream};
/// Read bytes from the stream and append them to the buffer.
///
/// Returns the number of bytes read. A return value of `0`
/// indicates EOF.
///
/// # Phase 1
/// Used to accumulate request bytes.
pub fn read_into(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>
) -> io::Result<usize>;

/// Read bytes until the delimiter is encountered or the maximum
/// byte limit is reached.
///
/// # Phase 1
/// Optional helper for line-based parsing.
pub fn read_until(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>,
    delim: u8,
    max: usize
) -> io::Result<usize>;

/// Write all bytes to the stream.
///
/// This function does not return until all bytes have been written
/// or an error occurs.
///
/// # Phase 1
/// Required for sending responses.
pub fn write_all(
    stream: &mut TcpStream,
    bytes: &[u8]
) -> io::Result<()>;