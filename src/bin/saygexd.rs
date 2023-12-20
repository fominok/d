use std::{
    fs::File,
    io::{BufRead, BufReader, Cursor, Read},
    os::unix::net::{UnixListener, UnixStream},
};

use saygex::SOCKET_PATH;

fn main() {
    if std::fs::metadata(SOCKET_PATH).is_ok() {
        std::fs::remove_file(SOCKET_PATH);
    }

    let listener = UnixListener::bind(SOCKET_PATH).expect("cannot connect to the socket");

    let mut line = String::new();
    loop {
        if let Some(Ok(incoming)) = listener.incoming().next() {
            let mut reader = BufReader::new(incoming);
            loop {
                match reader.read_line(&mut line) {
                    Ok(n) if n > 0 => {
                        if &line[0..2] == "e " {
                            eprintln!("error: {}", &line[2..]);
                        } else {
                            println!("ok: {}", &line[2..]);
                        }
                    }
                    _ => break,
                }
            }
        }
    }
}
