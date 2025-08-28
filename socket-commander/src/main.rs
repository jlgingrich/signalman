use anyhow::Context;
use jrpc_types::JsonRpcRequest;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;
use uuid::Uuid;

use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

fn generate_new_id() -> String {
    Uuid::new_v4().into()
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Connect to UNIX socket
    let mut unix_stream =
        UnixStream::connect(&cli.socket).context("Could not create stream from socket")?;

    let mut unix_stream_notifications =
        UnixStream::connect(&cli.socket).context("Could not create stream from socket")?;

    // Write request to socket
    let next_id = generate_new_id();

    let json_request = JsonRpcRequest::builder()
        .id(next_id.as_str())
        .method("listDevices")
        .build();

    let serialized_json_request = serde_json::to_string(&json_request)?;

    println!("Sending request:\n{}", serialized_json_request);
    writeln!(unix_stream, "{}", serialized_json_request)
        .context("Unable to write to unix socket")?;

    unix_stream
        .shutdown(std::net::Shutdown::Write)
        .context("Could not shutdown writing on the stream")?;

    // Read response from socket
    let mut response = String::new();
    unix_stream
        .read_to_string(&mut response)
        .context("Unable to read from unix socket")?;

    println!("Received response: {}", response);

    /*
    stream_write.shutdown(std:)

    let mut buffer = String::new();
    reader.read_line(&mut buffer)?;

    let response = buffer.as_str();
    println!("Response: {}", response);
    stdout().flush()?;

    match TryInto::<JsonRpcResponse>::try_into(response) {
        Ok(req) => {
            print!(
                "Received response {:?} with status {:?}",
                req.id, req.status
            );
        }
        Err(e) => {
            println!("{}", e)
        }
    }
    */
    Ok(())
}
