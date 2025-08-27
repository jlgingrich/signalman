use chrono::{Local, TimeZone};
use futures::{Stream, TryStreamExt};
use std::{env, fs::File, io::Write, process, time::Duration};

use eventsource_client as es;

#[tokio::main]
async fn main() -> Result<(), es::Error> {
    env_logger::init();

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please pass the SSE url as an argument");
        process::exit(1);
    }

    let url = &args[1];

    let client = es::ClientBuilder::for_url(url)?
        .reconnect(
            es::ReconnectOptions::reconnect(true)
                .retry_initial(false)
                .delay(Duration::from_secs(1))
                .backoff_factor(2)
                .delay_max(Duration::from_secs(60))
                .build(),
        )
        .build();

    let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}

    Ok(())
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            es::SSE::Connected(connection) => {
                println!(
                    "Connected with status code {}",
                    connection.response().status()
                )
            }
            es::SSE::Event(ev) => {
                let data_parsed = json::parse(&ev.data).expect("Unable to parse message JSON");
                let unix_timestamp = &data_parsed["envelope"]["timestamp"];
                let timestamp = Local
                    .timestamp_millis_opt(
                        unix_timestamp
                            .as_i64()
                            .expect("Timestamp should be an integer"),
                    )
                    .earliest()
                    .expect("Timestamp should be a valid local time");

                let formatted_time = timestamp.format("%F_%I-%M-%S");

                println!(
                    "Got '{0}' event at {1}",
                    ev.event_type,
                    timestamp.format("%c")
                );

                let mut file = File::create(format!("{0}_{1}.json", formatted_time, ev.event_type))
                    .expect("Unable to create log file");

                write!(file, "{:#}", data_parsed).expect("Not able to write to file");
            }
            _ => (),
        })
        .map_err(|err| eprintln!("Streaming error: {:?}", err))
}
