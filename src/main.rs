use std::io::{self, BufRead};

use native_tls::TlsConnector;
use std::collections::HashMap;
use std::env;
use std::io::Write;
use std::net::TcpStream;

use serde_json;

use log::{info, trace};


fn build_doc(content: String, user: &str, identifier: &str) -> HashMap<String, String> {
    let mut doc = HashMap::new();
    let tags = format!("env:staging,user:{},session:{}", user, identifier);

    doc.insert(String::from("ddsource"), String::from("log-pipe"));
    doc.insert(String::from("ddtags"), tags);

    doc.insert(String::from("service"), String::from("cli-client"));
    doc.insert(String::from("message"), content);

    doc
}

fn main() {
    pretty_env_logger::init();
    let hostname = env::var("DD_HOSTNAME").unwrap_or(String::from("intake.logs.datad0g.com"));
    let user = env::var("USER").unwrap_or(String::from("no-user"));
    let api_key = env::var("DD_API_KEY").unwrap_or(String::from("NOKEY"));
    let remote_host = format!("{}:10516", hostname);

    let connector = TlsConnector::new().unwrap();
    info!("Using remote: {}", remote_host);

    let stream = TcpStream::connect(remote_host).unwrap();
    let mut stream = connector
        .connect("intake.logs.datad0g.com", stream)
        .unwrap();

    let identifier = format!("sess{}", 23);
    println!("https://dd.datad0g.com/logs?cols=event&index=main&live=true&query=source%3Alog-pipe+service%3Acli-client+session%3A{}&sort=desc&stream_sort=desc", identifier);

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let doc = build_doc(line.unwrap(), &user, &identifier);
        let log_line = serde_json::to_string(&doc).unwrap();
        let apied = format!("{} {}\n", api_key, log_line);
        stream.write_all(&apied.clone().into_bytes()).unwrap();
        trace!("{}", apied);
    }
}
