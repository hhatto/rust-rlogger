use std::io;
use std::io::prelude::*;
use time;
use byteorder::{BigEndian, WriteBytesExt};
use std::os::unix::net::UnixStream;

static RLOGGER_VERSION: i8 = 1;
const RLOGGER_PSH: i8 = 1;
const RLOGGER_HEADER_LEN: i32 = 12;
// const RLOGGER_MSG_HEADER_LEN: i32 = 8;

struct RLoggerHeader {
    version: i8,
    psh: i8,
    len: i16,
    offset: i32,
    msg_len: i32,
    tag: Vec<u8>,
}

struct RLoggerMessage {
    time: i32,
    msg_len: i32,
    msg: Vec<u8>,
}

struct RLoggerPacket {
    header: RLoggerHeader,
    messages: Vec<RLoggerMessage>,
}

impl RLoggerPacket {
    fn new(tag: &str, msg: &str) -> RLoggerPacket {
        let t = time::get_time();
        let tag_len = tag.as_bytes().len();

        let (messages, msg_pkt_len) = RLoggerPacket::gen_message(t, msg);

        let rlogger_header = RLoggerHeader {
            version: RLOGGER_VERSION,
            psh: RLOGGER_PSH,
            len: RLOGGER_HEADER_LEN as i16 + tag_len as i16,
            offset: 0,
            msg_len: RLOGGER_HEADER_LEN + tag_len as i32 + msg_pkt_len as i32,
            tag: tag.as_bytes().to_vec(),
        };
        RLoggerPacket {
            header: rlogger_header,
            messages: messages,
        }
    }

    fn gen_message(t: time::Timespec, msg: &str) -> (Vec<RLoggerMessage>, usize) {
        let mut all_msg_len: usize = 0;
        let mut ret = vec![];
        for line in msg.lines() {
            let msg_len = line.len();
            let rlogger_msg = RLoggerMessage {
                time: t.sec as i32,
                msg_len: msg_len as i32,
                msg: line.as_bytes().to_vec(),
            };
            ret.push(rlogger_msg);
            all_msg_len += msg_len;
        }
        (ret, all_msg_len)
    }
}

pub struct RLogger {
    stream: UnixStream,
}

impl RLogger {
    pub fn new(socket_path: &str) -> RLogger {
        let s = UnixStream::connect(socket_path).unwrap();
        RLogger { stream: s }
    }

    fn gen_packet(&self, tag: &str, msg: &str) -> Vec<u8> {
        let pkt = RLoggerPacket::new(tag, msg);
        let mut buf = vec![];
        buf.write_i8(pkt.header.version).unwrap();
        buf.write_i8(pkt.header.psh).unwrap();
        buf.write_i16::<BigEndian>(pkt.header.len).unwrap();
        buf.write_i32::<BigEndian>(pkt.header.offset).unwrap();
        buf.write_i32::<BigEndian>(pkt.header.msg_len).unwrap();
        buf.extend_from_slice(pkt.header.tag.as_slice());
        for m in pkt.messages {
            buf.write_i32::<BigEndian>(m.time).unwrap();
            buf.write_i32::<BigEndian>(m.msg_len).unwrap();
            buf.extend_from_slice(m.msg.as_slice());
        }
        buf
    }

    pub fn write(&mut self, tag: &str, msg: &str) -> Result<(), io::Error> {
        let pkt = self.gen_packet(tag, msg);
        self.stream.write_all(pkt.as_slice())
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use rlogger::RLogger;
    use std::path::Path;
    use std::io::prelude::*;
    use std::thread;
    use std::os::unix::net::UnixListener;

    fn handle_client(listener: UnixListener) {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buf = vec![];
                    let len = stream.read_to_end(&mut buf).unwrap();
                    assert!(len != 0);
                }
                Err(err) => {
                    assert!(true, "{:?}", err);
                }
            }
            break;
        }
    }

    #[test]
    fn smoke() {
        // dummy rloggerd's unix domain socket
        let dir = tempdir::TempDir::new("rust-rlogger").unwrap();
        let socket_path = dir.path().join(Path::new("rloggerd.dummy.sock"));
        let listener = UnixListener::bind(socket_path.to_str().unwrap()).unwrap();
        thread::spawn(|| handle_client(listener));

        let mut logger = RLogger::new(socket_path.to_str().unwrap());
        logger.write("this.is.tag", "hello rlogd").unwrap();
    }

    #[test]
    fn smoke_two_msg() {
        let dir = tempdir::TempDir::new("rust-rlogger").unwrap();
        let socket_path = dir.path().join(Path::new("rloggerd.dummy.sock"));
        let listener = UnixListener::bind(socket_path.to_str().unwrap()).unwrap();
        thread::spawn(|| handle_client(listener));

        let mut logger = RLogger::new(socket_path.to_str().unwrap());
        let msg = "hello rlogd2\nhello rlogd3";
        logger.write("this.is.tag", msg).unwrap();
    }
}
