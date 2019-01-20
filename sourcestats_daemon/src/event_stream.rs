use mio::net::TcpStream;

pub struct EventStream {
    pub stream: TcpStream,
}

impl EventStream {
    pub fn new(stream: TcpStream) -> EventStream {
        EventStream {
            stream
        }
    }
}