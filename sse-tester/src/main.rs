use std::env::args;
use std::io::{BufReader, prelude::*};
use std::os::unix::net::UnixStream;

use jrpc_types::JsonRpcNotification;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().collect();

    let stream = UnixStream::connect(&args[1])?;
    let mut reader = BufReader::new(stream);
    let mut data = String::new();

    loop {
        data.clear();
        let _ = reader.read_line(&mut data);

        match TryInto::<JsonRpcNotification>::try_into(data.as_str()) {
            Ok(req) => {
                println!(
                    "Got {0} notification:\n{1}",
                    req.method,
                    serde_json::to_string_pretty(&req.params.unwrap()).unwrap()
                );
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}
