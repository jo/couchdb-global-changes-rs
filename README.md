# CouchDB global changes stream
This is an _experimental_ project to implement a global changes feed in Rust.

## Usage
```sh
$ couchdb-global-changes --help
Global Changes Feed 1.0
Johannes J. Schmidt <schmidt@netzmerk.com>
Listens to changes on all databases

USAGE:
    couchdb-global-changes [OPTIONS] <URL>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --password <PASSWORD>    Sets the password for authentication
    -u, --username <NAME>        Sets the username for authentication

ARGS:
    <URL>    Sets the url to the CouchDB server
```

(c) 2016 Johannes J. Schmidt
