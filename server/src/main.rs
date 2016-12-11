extern crate ws;
use ws::listen;

extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("ws tester server-side.")
                      .arg(Arg::with_name("port").takes_value(true))
                      .get_matches();

    let port = matches.value_of("port").unwrap_or(8000);
    println!("Value for port: {}", port);

    listen("127.0.0.1:3012", |out| move |msg| out.send(msg)).unwrap();
}
