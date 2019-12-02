use std::io::{self, BufRead};

use native_tls::TlsConnector;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::net::TcpStream;

use log::{info, trace};
use regex::Regex;
use serde_json;


use rand::prelude::*;

fn build_doc(content: String, user: &str, identifier: &str) -> HashMap<String, String> {
    let mut doc = HashMap::new();
    let tags = format!("env:staging,user:{},session:{}", user, identifier);

    doc.insert(String::from("ddsource"), String::from("log-pipe"));
    doc.insert(String::from("ddtags"), tags);

    doc.insert(String::from("service"), String::from("cli-client"));
    doc.insert(String::from("message"), content);

    doc
}


fn conceal(line: String) -> String {
    let re = Regex::new(r"=[0-9a-fA-F]{0,24}").unwrap();

    let after = re.replace_all(line.as_str(), "=[REDACTED]");
    return after.into_owned();
}

fn main() {
    pretty_env_logger::init();
    let mut rng = rand::thread_rng();

    let site = env::var("DD_SITE").unwrap_or(String::from("datadoghq.com"));
    let dd_url = env::var("DD_URL").unwrap_or(String::from("https://app.datadoghq.com"));

    let user = env::var("USER").unwrap_or(String::from("no-user"));
    let api_key = env::var("DD_API_KEY").unwrap_or(String::from("NOKEY"));
    let remote_host = format!("intake.logs.{}", site);
    let tcp_remote = format!("{}:10516", remote_host);

    let connector = TlsConnector::new().unwrap();
    info!("Using remote: {}", remote_host);

    let stream = TcpStream::connect(tcp_remote).unwrap();
    let mut stream = connector.connect(&remote_host, stream).unwrap();

    let fsession: f64 = rng.gen();

    let session_id = (999.0 * fsession).trunc() as i32; // generates a float between 0 and 1

    let identifier = format!("sess{}", session_id);
    println!("{}/logs?cols=event&index=main&live=true&query=source%3Alog-pipe+service%3Acli-client+session%3A{}&sort=desc&stream_sort=desc", dd_url,identifier);

    let stdin = io::stdin();
    let mut c = 0;
    for line in stdin.lock().lines() {
        let doc = build_doc(conceal(line.unwrap()), &user, &identifier);
        let log_line = serde_json::to_string(&doc).unwrap();
        let apied = format!("{} {}\n", api_key, log_line);
        stream.write_all(&apied.clone().into_bytes()).unwrap();
        c = c + 1;
        trace!("{}", apied);
    }
    println!("Pushed {} lines", c);
}
