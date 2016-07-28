# rust-rlogger [![](https://travis-ci.org/hhatto/rust-rlogger.svg?branch=master)](https://travis-ci.org/hhatto/rust-rlogger)

Rust client for [rlogd](https://github.com/pandax381/rlogd)'s rloggerd.

# Installation

`Cargo.toml`

```
[dependencies]
rlogger = { git = "https://github.com/hhatto/rust-rlogger.git", branch = "master" }
```

# Usage

```rust
extern crate rlogger;
use rlogger::rlogger::RLogger;

fn main() {
    let socket_path = "/path/to/rloggerd.sock";
    let mut logger = RLogger::new(socket_path);
    let tag = "this.is.tag";
    let msg = "this is application log";
    logger.write(tag, msg);
}
```

# X-rlogger family
* [rlogger-py](https://github.com/KLab/rlogger-py) (Python)
* [php-rlogger](https://github.com/hnw/php-rlogger) (PHP)
* [go-rlogger](https://github.com/hhatto/go-rlogger) (Go)

# License

MIT
