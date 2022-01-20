use std::io::{BufRead, BufReader, BufWriter, Write};
use std::net::TcpListener;

fn main() {
    // listen port 10000
    let listener = TcpListener::bind("127.0.0.1:10000").unwrap();

    // Accept connection
    while let Ok((stream, _)) = listener.accept() {
        let stream0 = stream.try_clone().unwrap(); // clone
        let mut reader = BufReader::new(stream0); // Reader
        let mut writer = BufWriter::new(stream); // Writer

        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap(); // Read line
        writer.write(buf.as_bytes()).unwrap(); // Write same data
        writer.flush().unwrap(); // Send all data
    }
}
