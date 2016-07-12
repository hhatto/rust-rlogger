extern crate rlogger;
use rlogger::rlogger::RLogger;

fn main() {
    let socket_path = "/path/to/rloggerd.sock";
    let mut logger = RLogger::new(socket_path);
    let tag = "this.is.tag";
    let msg = "this is application log";
    logger.write(tag, msg);
}
