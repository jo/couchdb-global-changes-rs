extern crate hyper;
extern crate rustc_serialize;
extern crate clap;

use clap::{Arg, App};
use hyper::Client;
use hyper::header::{Headers, Authorization, Basic};
use std::io::Read;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Result {
    seq: String,
    db_name: String
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct DbUpdates {
    last_seq: String,
    results: Vec<Result>,
}

fn main() {
    let matches = App::new("DbUpdates Feed Listener")
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

    
    let server_url = matches.value_of("URL").unwrap();
    // println!("Using url: {}", server_url);

    let client = Client::new();
    
    let mut headers = Headers::new();


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
    let url = server_url.to_string() + "/_db_updates";
    let mut response = client.get(&url).headers(headers).send().unwrap();

    assert_eq!(response.status, hyper::Ok);

    let mut body = String::new();
    let result = response.read_to_string(&mut body);
    // println!("body {:?}", body);

    let db_updates: DbUpdates = json::decode(&body).unwrap();
    // println!("decoded {:?}", decoded);
    for result in db_updates.results {
       println!("{}", result.db_name);
    }
}
