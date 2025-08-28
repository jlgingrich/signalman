use std::{path::PathBuf, time::Duration};

use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use jrpc_types::JsonRpcNotification;
use tokio::{
    io::{self, AsyncBufReadExt, AsyncWriteExt},
    runtime::Runtime,
};

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let handle = move |req| handle_notification(req, cli.output.clone());

    let rt = Runtime::new().unwrap();
    let _ = rt.spawn(listen_on_socket(cli.socket, handle));

    std::thread::sleep(Duration::MAX);
    Ok(())
}

async fn listen_on_socket<F>(socket_path: PathBuf, handle_ok: F) -> io::Result<()>
where
    F: AsyncFn(JsonRpcNotification) -> io::Result<()>,
{
    let stream = tokio::net::UnixStream::connect(socket_path).await?;
    let reader = io::BufReader::new(stream);
    let mut lines = reader.lines();
    while let Some(line) = lines.next_line().await? {
        match TryInto::<JsonRpcNotification>::try_into(line.as_str()) {
            Ok(req) => {
                handle_ok(req).await?;
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    Ok(())
}

async fn handle_notification(
    req: JsonRpcNotification,
    output_dir: Option<PathBuf>,
) -> io::Result<()> {
    let value = &req.params.expect("Couldn't parse JSON body");
    let unix_timestamp = value["envelope"]["timestamp"]
        .as_i64()
        .expect("Timestamp was in an invalid format");
    let datetime_local = get_local_time_from_unix_seconds(unix_timestamp);

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

    write_to_file(full_path, serde_json::to_string_pretty(value)?).await
}

fn get_local_time_from_unix_seconds(milliseconds: i64) -> DateTime<Local> {
    Local
        .timestamp_millis_opt(milliseconds)
        .earliest()
        .expect("Timestamp should be a valid local time")
}

async fn write_to_file(path: PathBuf, contents: String) -> io::Result<()> {
    let mut file = tokio::fs::File::create(path).await?;
    file.write(contents.as_bytes()).await.map(|_| ())
}
