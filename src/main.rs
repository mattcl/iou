use std::{num::ParseIntError, process::Command};
use clap::{App, AppSettings, Arg, crate_authors, crate_description, crate_name, crate_version};
use warp::Filter;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
struct Url {
    url: String,
}

fn handle_url(url: Url, _wsl: bool) {
    let res = Command::new("wslview")
        .args([url.url])
        .output();

    match res {
        Ok(_) => {},
        Err(e) => {log::error!("{:?}", e)}
    }
}

fn exit_with_error(e: &str) {
    clap::Error::with_description(
        &format!("{:?}", e),
        clap::ErrorKind::InvalidValue, // just a generic kind, but it could be anything
    )
    .exit()
}

#[tokio::main]
async fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("port")
                .help("What port to listen on")
                .long("port")
                .short("p")
                .default_value("3756")
                .takes_value(true))
        .arg(
            Arg::with_name("wsl")
                .help("Specifal flag for running in WSL")
                .long("wsl")
                .short("w")
                .required(false)
                .takes_value(false)
        ).get_matches();

    let port: Result<u16, ParseIntError> = matches.value_of("port").unwrap().parse();

    if let Err(ref e) = port {
        exit_with_error(&e.to_string())
    }

    let wsl = matches.is_present("wsl");

    pretty_env_logger::init();
    let log = warp::log("iou::api");
    let route = warp::post()
        .and(warp::body::content_length_limit(1024 * 32))
        .and(warp::body::json())
        .map(move |url: Url| {
            handle_url(url, wsl);
            warp::reply()
        })
        .with(log);

    warp::serve(route)
        .run(([0, 0, 0, 0], port.unwrap()))
        .await;
}
