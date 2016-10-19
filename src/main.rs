#![feature(proc_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate hyper;
extern crate clap;
extern crate url;

use clap::{Arg, App, ArgMatches};
use hyper::Client;
use hyper::client::Response;
use hyper::header::{Headers, Authorization, Basic};
use std::io::{Read, BufReader, BufRead};
use url::percent_encoding::utf8_percent_encode;
use url::percent_encoding::PATH_SEGMENT_ENCODE_SET;


#[derive(Deserialize)]
pub struct Change {
    seq: String,
    id: String
}

#[derive(Deserialize)]
pub struct DbUpdateResult {
    #[serde(rename="type")]
    update_type: String,
    seq: String,
    db_name: String
}

fn make_request(matches: &ArgMatches, client: &Client, path: String) -> Response {
    let server_url = matches.value_of("URL").unwrap();
    let mut headers = Headers::new();

    let url = format!("{}/{}", server_url, path);

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
    // println!("response: {}:{}", url, response.status);

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
    
    let path = format!("_db_updates?feed=continuous&timeout=100");
    let response = make_request(&matches, &client, path);
    assert_eq!(response.status, hyper::Ok);
    
    let mut response = BufReader::new(response);

    for line in response.lines() {
        // println!("{:?}", line.unwrap());
    
        match serde_json::from_str::<DbUpdateResult>(&line.unwrap()) {
            Ok(result) => {
                match result.update_type.as_ref() {
                    "updated" => {
                        if !result.db_name.starts_with("_") {
                            // println!("{}", result.db_name);
                         
                            let path = format!("{}/_changes?feed=continuous&timeout=1", utf8_percent_encode(&result.db_name, PATH_SEGMENT_ENCODE_SET));
                            let response = make_request(&matches, &client, path);
                           
                            match response.status {
                                hyper::Ok => {
                                    let mut response = BufReader::new(response);
                                   
                                    for line in response.lines() {
                                        match serde_json::from_str::<Change>(&line.unwrap()) {
                                            Ok(change) => {
                                                println!("{}/{}", result.db_name, change.id);
                                            }
                                            _ => {}
                                        }
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
