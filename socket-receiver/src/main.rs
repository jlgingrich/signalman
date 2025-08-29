use std::path::PathBuf;

use clap::Parser;
use jrpc_types::JsonRpcNotification;
use tokio::io::{self};

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let socket = tokio::net::UnixStream::connect(cli.socket).await?;

    async fn dump_notification_to_file(
        req: JsonRpcNotification,
        output_dir: Option<PathBuf>,
    ) -> io::Result<()> {
        let value = &req.params.expect("Couldn't parse JSON body");
        let unix_timestamp = value["envelope"]["timestamp"]
            .as_i64()
            .expect("Timestamp was in an invalid format");
        let datetime_local = util::get_local_time_from_unix_seconds(unix_timestamp);

        println!(
            "Got {0} notification on {1}",
            req.method,
            datetime_local.format("%D at %r")
        );

        let file_name_timestamp = datetime_local.format("%F_%H-%M-%S");

        let mut file_path = PathBuf::new();

        if let Some(odir) = output_dir {
            file_path = file_path.join(odir);
        }

        let full_path = file_path.join(format!("{0}_{1}.json", file_name_timestamp, &req.method));

        util::write_to_file(full_path, serde_json::to_string_pretty(value)?).await
    }
    let handler = move |req| dump_notification_to_file(req, cli.output.clone());

    json_rpc::handle_notifications(socket, handler).await?;

    Ok(())
}

pub mod util {
    use std::path::PathBuf;

    use chrono::{DateTime, Local, TimeZone};
    use tokio::io::AsyncWriteExt;

    pub fn get_local_time_from_unix_seconds(milliseconds: i64) -> DateTime<Local> {
        Local
            .timestamp_millis_opt(milliseconds)
            .earliest()
            .expect("Timestamp should be a valid local time")
    }

    pub async fn write_to_file(path: PathBuf, contents: String) -> std::io::Result<()> {
        let mut file = tokio::fs::File::create(path).await?;
        file.write(contents.as_bytes()).await.map(|_| ())
    }
}

pub mod json_rpc {
    use jrpc_types::JsonRpcNotification;
    use tokio::{
        io::{self, AsyncBufReadExt},
        net::UnixStream,
    };

    pub async fn handle_notifications<F>(socket: UnixStream, handle_ok: F) -> tokio::io::Result<()>
    where
        F: AsyncFn(JsonRpcNotification) -> io::Result<()>,
    {
        let reader = io::BufReader::new(socket);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            let notification = TryInto::<JsonRpcNotification>::try_into(line.as_str())
                .expect("JSON-RPC notification could not be deserialized");
            handle_ok(notification).await?;
        }
        Ok(())
    }
}
