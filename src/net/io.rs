use std::{io::{self, Read, Write}, net::TcpStream};
use super::framing;

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
) -> io::Result<usize> {
    let mut temp = [0; 1024];
    let bytes_read = stream.read(&mut temp)?;
    buf.extend_from_slice(&temp[..bytes_read]);
    Ok(bytes_read)
}

/// Read bytes until the delimiter is encountered or the maximum
/// byte limit is reached.
///
/// # Phase 1
/// Optional helper for line-based parsing.
pub fn read_until(
    stream: &mut TcpStream,
    buf: &mut Vec<u8>,
    delim: &[u8],
    max: usize
) -> io::Result<usize> {

    let mut bytes_read = 0;

    loop {
        let mut temp = [0; 1024];
        let temp_bytes = stream.read(&mut temp)?;
        if temp_bytes == 0 {
            break;
        }
        match framing::find_subslice(&temp, &delim) {
            Some(idx) => { 
                buf.extend_from_slice(&temp[..idx]);
                bytes_read += idx;
                break;
            },
            None => {
                buf.extend_from_slice(&temp[..temp_bytes]);
                bytes_read += temp_bytes;
            }
        }
        
    }
    
    Ok(bytes_read)
}

// fn find_delim(buf: &[u8], delim: &[u8]) -> Option<usize> {
//     if delim.is_empty() {
//         return Some(0);
//     }
//     buf.windows(delim.len()).position(|window| window == delim)
// }

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
) -> io::Result<()> {
    stream.write_all(bytes)
}