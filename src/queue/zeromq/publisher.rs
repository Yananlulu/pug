use std::sync::Mutex;

use zmq;

use super::super::super::errors::Result;

pub struct Publisher {
    socket: Mutex<zmq::Socket>,
}

impl Publisher {
    pub fn new(url: &String) -> Result<Self> {
        let c = zmq::Context::new();
        let s = c.socket(zmq::PUB)?;
        s.bind(url)?;
        Ok(Self {
            socket: Mutex::new(s),
        })
    }
    pub fn send(&self, channel: &[u8], message: &[u8]) -> Result<()> {
        if let Ok(s) = self.socket.lock() {
            s.send(channel, zmq::SNDMORE)?;
            s.send(message, 0)?;
            return Ok(());
        }
        Err("can't get publisher".into())
    }
}
