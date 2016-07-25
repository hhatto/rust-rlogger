#![feature(test)]

extern crate test;
extern crate rlogger;
extern crate unix_socket;
extern crate tempdir;

use std::thread;
use std::path::Path;
use std::io::prelude::*;
use rlogger::rlogger::RLogger;
use test::Bencher;
use unix_socket::UnixListener;

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

#[bench]
fn smoke_bench(b: &mut Bencher) {
    let tag = "this is tag";
    let msg = "this is log";
    let dir = tempdir::TempDir::new("rust-rlogger").unwrap();
    let socket_path = dir.path().join(Path::new("rloggerd.dummy.sock"));
    let listener = UnixListener::bind(socket_path.to_str().unwrap()).unwrap();
    thread::spawn(|| handle_client(listener));

    let mut l = RLogger::new(socket_path.to_str().unwrap());
    b.iter(|| l.write(tag, msg).unwrap());
}
