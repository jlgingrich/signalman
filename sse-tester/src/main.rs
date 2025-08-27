use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use chrono::{DateTime, Local, TimeZone};
use clap::Parser;
use jrpc_types::JsonRpcNotification;

#[derive(Parser, Debug)]
struct Cli {
    socket: PathBuf,
    #[arg(short, long, value_name = "OUTPUT_DIR")]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let stream = UnixStream::connect(cli.socket)?;
    let reader = BufReader::new(stream);

    for line in reader.lines() {
        match TryInto::<JsonRpcNotification>::try_into(line?.as_str()) {
            Ok(req) => {
                handle_notification(req, &cli.output)?;
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
    Ok(())
}

fn handle_notification(
    req: JsonRpcNotification,
    output_dir: &Option<PathBuf>,
) -> Result<(), std::io::Error> {
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

    write_to_file(full_path, serde_json::to_string_pretty(value)?)?;

    Ok(())
}

fn get_local_time_from_unix_seconds(milliseconds: i64) -> DateTime<Local> {
    Local
        .timestamp_millis_opt(milliseconds)
        .earliest()
        .expect("Timestamp should be a valid local time")
}

fn write_to_file(path: PathBuf, contents: String) -> Result<(), std::io::Error> {
    let mut file = File::create(path)?;

    write!(file, "{}", contents)
}
