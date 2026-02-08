use std::net::TcpListener;
use std::io::Read;

enum STATUS_CODE {
    OK = 200,
    NOT_FOUND = 404,
    INTERNAL_SERVER_ERROR = 500,
    BAD_REQUEST = 400,
    GATEWAY_TIMEOUT = 504,
}

struct Router {
    // This struct can be expanded to include routing logic
}


fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        
        let mut buffer = [0; 512];
        let bytes_read = stream.read(&mut buffer)?;
        println!("Received request: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
    }
    Ok(())
}
