extern crate hyper;
extern crate rustc_serialize;
extern crate clap;
extern crate url;

use clap::{Arg, App, ArgMatches};
use hyper::Client;
use hyper::client::Response;
use hyper::header::{Headers, Authorization, Basic};
use std::io::{Read, BufReader, BufRead};
use rustc_serialize::json;
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::PATH_SEGMENT_ENCODE_SET;


#[derive(RustcDecodable, RustcEncodable)]
pub struct ChangesResult {
    seq: String,
    id: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct Changes {
    last_seq: String,
    results: Vec<ChangesResult>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DbUpdateResult {
    seq: String,
    db_name: String
}

fn make_request(matches: &ArgMatches, client: &Client, path: String) -> Response {
    let server_url = matches.value_of("URL").unwrap();
    let mut headers = Headers::new();

    let url = format!("{}/{}", server_url, path);

    // Gets a value for username if supplied by user, or defaults to "default.conf"
    if let Some(username) = matches.value_of("username") {
        let password = matches.value_of("password").unwrap();
        // println!("auth: {}:{}", username, password);

        headers.set(Authorization(
           Basic {
               username: username.to_owned(),
               password: Some(password.to_owned())
           }
       ));
    }

    let response = client.get(&url).headers(headers).send().unwrap();
    assert_eq!(response.status, hyper::Ok);

    response
}

fn main() {
    let matches = App::new("Global Changes Feed")
                          .version("1.0")
                          .author("Johannes J. Schmidt <schmidt@netzmerk.com>")
                          .about("Listens to changes on all databases")
                          .arg(Arg::with_name("username")
                               .short("u")
                               .long("username")
                               .value_name("NAME")
                               .help("Sets the username for authentication")
                               .requires("password")
                               .takes_value(true))
                          .arg(Arg::with_name("password")
                               .short("p")
                               .long("password")
                               .value_name("PASSWORD")
                               .help("Sets the password for authentication")
			       .requires("username")
                               .takes_value(true))
                          .arg(Arg::with_name("URL")
                               .help("Sets the url to the CouchDB server")
                               .required(true)
                               .index(1))
                          .get_matches();

    

    let client = Client::new();
    
    let path = format!("_db_updates?feed=continuous");
    let response = make_request(&matches, &client, path);
    let mut response = BufReader::new(response);
    let mut buffer = String::new();

    while response.read_line(&mut buffer).unwrap() > 0 {
        // println!("{:?}", buffer);
    
        let result: DbUpdateResult = json::decode(&buffer).unwrap();
           if !result.db_name.starts_with("_") {
             // println!("{}", result.db_name);

             let path = format!("{}/_changes", utf8_percent_encode(&result.db_name, PATH_SEGMENT_ENCODE_SET));
             let mut response = make_request(&matches, &client, path);

             let mut body = String::new();
             response.read_to_string(&mut body).unwrap();
             // println!("{}", body);

             let changes: Changes = json::decode(&body).unwrap();
             for r in changes.results {
               println!("{}/{}", result.db_name, r.id);
             }
           }
        
        buffer.clear();
    }
}
