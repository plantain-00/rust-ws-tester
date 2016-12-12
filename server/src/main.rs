extern crate ws;
use ws::listen;

extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("ws tester server-side.")
                      .arg(Arg::with_name("port").takes_value(true).long("port"))
                      .arg(Arg::with_name("host").takes_value(true).long("host"))
                      .get_matches();
    let host = matches.value_of("host").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8000").parse::<u16>().unwrap();
    listen((host, port), |out| move |msg| out.send(msg)).unwrap();
}
