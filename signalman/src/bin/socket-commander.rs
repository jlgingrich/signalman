use anyhow::Context;
use std::path::PathBuf;
use tokio::net::UnixStream;

use clap::Parser;

pub type JsonRpcResult = jrpc_types::response::Status;

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Connect to UNIX socket
    let unix_stream = UnixStream::connect(&cli.socket)
        .await
        .context("Could not create stream from socket")?;

    // Print results
    match json_rpc::send_request(unix_stream, "listDevices").await? {
        JsonRpcResult::Success(response) => {
            println!("{}", serde_json::to_string_pretty(&response)?);
        }
        JsonRpcResult::Error {
            code,
            message,
            data: _,
        } => {
            println!("Error code {}: {}", code, message);
        }
    }

    Ok(())
}

pub mod util {
    pub fn generate_new_id() -> String {
        uuid::Uuid::new_v4().into()
    }
}

pub mod json_rpc {
    use crate::{JsonRpcResult, util::generate_new_id};
    use anyhow::Context;
    use jrpc_types::{JsonRpcRequest, JsonRpcResponse};
    use tokio::{io::AsyncReadExt, io::AsyncWriteExt, net::UnixStream};

    pub async fn send_request(
        socket: UnixStream,
        method: &str,
    ) -> Result<JsonRpcResult, anyhow::Error> {
        let next_id = generate_new_id();

        let request = JsonRpcRequest::builder()
            .id(next_id.as_str())
            .method(method)
            .build();

        send_request_internal(socket, request).await
    }

    pub async fn send_request_with<T>(
        socket: UnixStream,
        method: &str,
        params: T,
    ) -> Result<JsonRpcResult, anyhow::Error>
    where
        T: serde::ser::Serialize,
    {
        let next_id = generate_new_id();

        let request = JsonRpcRequest::builder()
            .id(next_id.as_str())
            .method(method)
            .params_serialize(params)?
            .build();

        send_request_internal(socket, request).await
    }

    async fn send_request_internal(
        mut socket: UnixStream,
        request: JsonRpcRequest,
    ) -> Result<JsonRpcResult, anyhow::Error> {
        let serialized_json_request = serde_json::to_string(&request)?;

        socket
            .write(serialized_json_request.as_bytes())
            .await
            .context("Unable to write to socket")?;

        socket
            .shutdown()
            .await
            .context("Unable to shutdown writing to socket")?;

        let mut response = String::new();

        socket
            .read_to_string(&mut response)
            .await
            .context("Unable to read from socket")?;

        match TryInto::<JsonRpcResponse>::try_into(response.as_str()) {
            Ok(r) => {
                assert!(
                    r.id == request.id,
                    "Received response ID that did not match request ID"
                );
                Ok(r.status)
            }
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}
