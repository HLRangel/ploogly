// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::build::*;

use tiny_http::{Server, Response, Method};
use std::sync::{Arc, Mutex};
use std::{path::PathBuf, str::FromStr};
use std::fs::{canonicalize, exists, File};
use std::string::*;
use std::thread::spawn;
use std::io::stdin;
use std::io::ErrorKind;

fn listen_dir(port: &str, dir: &str, comm: Arc<Mutex<u8>>) -> Result<(), std::io::Error> {
    let path: PathBuf = canonicalize(PathBuf::from_str(dir).unwrap())?;
    let server: Server = Server::http(format!("127.0.0.1:{port}")).unwrap();

    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Get => {
                let mut req: String = request.url().to_string();
                req = match req.as_str() {
                    "/" => {
                        "/index.html".to_string()
                    },

                    _ => {
                        req
                    }
                };
                
                let path_to: String = String::from_str(path.join(
                                    req.strip_prefix("/").unwrap()
                                    ).to_str().unwrap()).unwrap();
                
                if exists(&path_to)? {
                    let response = Response::from_file(File::open(path_to)?);
                    request.respond(response)?;
                } else {
                    let response = Response::from_string("404!");
                    request.respond(response)?;
                }

                if *comm.lock().unwrap() == 1 {
                    return Ok(());
                }
            }, 

            // TODO: Implement some useful functionality for POSTs! ;)
            Method::Post => {
                let mut sttopr: String = String::new();
                request.as_reader().read_to_string(&mut sttopr)?;
                
                println!("{sttopr}");
            } 

            _ => {
                eprintln!("Came across an unsupported request type. Ignored.");
            }
        };
    }

    return Ok(());
}

pub fn serve_control(port: String) -> Result<(), std::io::Error>{
    if exists("./out/site")? {
        let sp: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
        let rp: Arc<Mutex<u8>> = sp.clone();
        
        let mut ended: bool = false;

        println!("Serving on 127.0.0.1:{}, access http://127.0.0.1:{} to view! \
        \nPress R, then Enter to rebuild the project \
        \nPress L, then Enter to properly quit after serving the next request. \
        \nPress Q, then Enter to force quit.\n\n", &port, &port);

        let handle = spawn(move || listen_dir(&port, "./out/site", rp));

        while !ended {
            let mut input: String = String::new();
            stdin().read_line(&mut input)?;

            match input.bytes().nth(0).unwrap() {
                b'R' | b'r' => {
                    build()?;
                    println!("Project rebuilt!");
                }

                b'Q' | b'q' => {
                    *sp.lock().unwrap() = 1;

                    return Ok(());
                },

                b'L' | b'l' => {
                    *sp.lock().unwrap() = 1;

                    ended = true;
                },

                _ => {
                    println!("Unrecognized input!");
                }
            }
        }

        handle.join().unwrap()?;
    } else {
        eprintln!("No build output exists. Use \"ploogly build\" first.");
        return Err(ErrorKind::InvalidInput.into());
    }

    return Ok(());
}
