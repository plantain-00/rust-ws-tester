extern crate ws;
use ws::{listen, Sender, CloseCode, Handshake, Handler};

extern crate clap;
use clap::{Arg, App};

extern crate pretty_bytes;
use pretty_bytes::converter::convert;

extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;
use timer::Guard;

fn main() {
    let matches = App::new("ws tester server-side.")
        .arg(Arg::with_name("port").takes_value(true).long("port"))
        .arg(Arg::with_name("host").takes_value(true).long("host"))
        .arg(Arg::with_name("message-count-per-second")
            .takes_value(true)
            .long("message-count-per-second"))
        .arg(Arg::with_name("message-count-increase")
            .takes_value(true)
            .long("message-count-increase"))
        .arg(Arg::with_name("message-length-increase")
            .takes_value(true)
            .long("message-length-increase"))
        .arg(Arg::with_name("increase-per-second")
            .takes_value(true)
            .long("increase-per-second"))
        .get_matches();
    let host = matches.value_of("host").unwrap_or("localhost");
    let port = matches.value_of("port").unwrap_or("8000").parse::<u16>().unwrap();
    let messageLength = matches.value_of("message-length")
        .unwrap_or("100")
        .parse::<usize>()
        .unwrap();
    let messageCountIncrease = matches.value_of("message-count-increase")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let messageLengthIncrease = matches.value_of("message-length-increase")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let increasePerSecond = matches.value_of("increase-per-second")
        .unwrap_or("0")
        .parse::<i32>()
        .unwrap();
    let messageCountPerSecond = matches.value_of("message-count-per-second")
        .unwrap_or("1")
        .parse::<i32>()
        .unwrap();

    println!("Listening {}:{}.", host, port);
    println!("Sending {} message * {} times per second.",
             convert(messageLength as f64),
             messageCountPerSecond);

    let message = std::iter::repeat("a").take(messageLength).collect::<String>();
    println!("{}.", message);

    let mut errorCount = 0;
    let mut messageCount = 0;
    let mut messageTotalLength = 0;

    struct Server {
        connection: Sender,
        guard: Guard,
    }

    impl Handler for Server {
        fn on_open(&mut self, shake: Handshake) -> std::result::Result<(), ws::Error> {
            let timer = timer::Timer::new();
            self.guard = timer.schedule_repeating(chrono::Duration::milliseconds(1000), move || {
                move |message| {
                    for i in 0..messageCountPerSecond {
                        let result = self.connection.send(message);
                        match result {
                            Result::Ok(val) => {
                                messageTotalLength += messageLength;
                                messageCount += 1;
                            }
                            Result::Err(e) => {
                                errorCount += 1;
                            }
                        }
                    }
                }
            });
        }
        fn on_close(&mut self, code: CloseCode, reason: &str) {
            drop(self.guard);
        }
    }

    listen((host, port), |connection| Server { connection: connection }).unwrap();
}
