use std::path::PathBuf;

use clap::Parser;
use tokio::io::{self};
use tokio::net::*;

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let socket = UnixStream::connect(cli.socket).await?;

    let mut notifications = json_rpc::listen(socket).await;

    loop {
        let notif = notifications
            .recv()
            .await
            .expect("Failed to receive notification");

        let value = &notif.params.expect("Couldn't parse JSON body");
        let unix_timestamp = value["envelope"]["timestamp"]
            .as_i64()
            .expect("Timestamp was in an invalid format");
        let datetime_local = util::get_local_time_from_unix_seconds(unix_timestamp);

        println!(
            "Got {0} notification on {1}",
            notif.method,
            datetime_local.format("%D at %r")
        );

        let file_name_timestamp = datetime_local.format("%F_%H-%M-%S");

        let mut file_path = PathBuf::new();

        if let Some(odir) = &cli.output {
            file_path = file_path.join(odir);
        }

        let full_path = file_path.join(format!("{0}_{1}.json", file_name_timestamp, &notif.method));

        util::write_to_file(full_path, serde_json::to_string_pretty(value)?).await?;
    }
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
        sync::broadcast,
    };

    pub async fn listen(socket: UnixStream) -> broadcast::Receiver<JsonRpcNotification> {
        let (sender, receiver) = broadcast::channel(32);

        tokio::spawn(async move {
            let reader = io::BufReader::new(socket);
            let mut lines = reader.lines();
            while let Some(line) = lines.next_line().await.expect("Failed to await next line") {
                let notification = TryInto::<JsonRpcNotification>::try_into(line.as_str())
                    .expect("JSON-RPC notification could not be deserialized");

                sender
                    .send(notification)
                    .expect("Failed to send notification to channel");
            }
        });

        receiver
    }
}
