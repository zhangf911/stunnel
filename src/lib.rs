#![feature(macro_rules)]

extern crate crypto;
extern crate time;

macro_rules! on_error {
    ($op: expr, $err_op: expr) => ({
        ($op).err().map(|_| $err_op)
    });
}

macro_rules! ok_or_break {
    ($op: expr) => ({
        match $op {
            Ok(v) => v,
            Err(_) => break
        }
    });
}

pub mod client;
pub mod server;
pub mod socks5;
pub mod crypto_wrapper;

mod protocol {
    pub const VERIFY_DATA: [u8, ..8] =
        [0xF0u8, 0xEF, 0xE, 0x2, 0xAE, 0xBC, 0x8C, 0x78];
    pub const HEARTBEAT_INTERVAL: i64 = 2;
    pub const ALIVE_TIMEOUT_TIME: i64 = 8;

    pub mod cs {
        pub const OPEN_PORT: u8 = 1;
        pub const CLOSE_PORT: u8 = 2;
        pub const CONNECT: u8 = 3;
        pub const CONNECT_DOMAIN_NAME: u8 = 4;
        pub const DATA: u8 = 5;
        pub const HEARTBEAT: u8 = 6;
    }

    pub mod sc {
        pub const CONNECT_OK: u8 = 1;
        pub const SHUTDOWN: u8 = 2;
        pub const DATA: u8 = 3;
        pub const HEARTBEAT_RSP: u8 = 4;
    }
}
