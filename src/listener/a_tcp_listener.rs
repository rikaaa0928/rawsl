use crate::connection::Connection;
use crate::listener::Listener;
use std::io::Result;
use std::net::{SocketAddr, TcpListener};
pub struct ATCPListener {
    l: TcpListener,
}
impl ATCPListener {
    pub fn new(l: TcpListener) -> ATCPListener {
        ATCPListener { l }
    }
}
impl Listener for ATCPListener {
    fn accept(&self) -> Result<(Box<dyn Connection>, SocketAddr)> {
        match self.l.accept() {
            Ok((socket, addr)) => Result::Ok((Box::new(socket), addr)),
            Err(e) => Result::Err(e),
        }
    }
}
