use std::{fmt::Display, io::Write, os::unix::net::UnixStream};

use clap::{Parser, ValueEnum};

use saygex::SOCKET_PATH;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(value_enum)]
    mode: Mode,
    text: String,
}

#[derive(Debug, ValueEnum, Clone)]
enum Mode {
    Out,
    Error,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Out => f.write_str("o"),
            Mode::Error => f.write_str("e"),
        }
    }
}

fn main() {
    let mut stream = UnixStream::connect(SOCKET_PATH).expect("cannot connect to the socket");
    let args = Args::parse();

    let s = format!("{} {}\n", args.mode, args.text);

    stream
        .write_all(s.as_ref())
        .expect("cannot write to the socket");
    dbg!("lol");
}
