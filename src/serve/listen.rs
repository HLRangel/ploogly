// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::build::*;
use crate::serve::lua_gen::*;

use tiny_http::{Server, Response, Method};
use std::sync::{Arc, Mutex};
use std::{path::PathBuf, str::FromStr};
use std::fs::{canonicalize, exists, File};
use std::string::*;
use std::thread::spawn;
use std::io::stdin;
use std::io::ErrorKind;

struct ReqData {
    path: String,
    query: Option<String>
}

struct Entry {
    key: String,
    value: String
}

struct ReqInfo {
    data: ReqData,
    entries: Option<Vec<Entry>>
}

fn url_query(url: &str) -> ReqData {    
    let vurl_raw: String = url.to_string();
    let vurl: Vec<u8> = vurl_raw.as_bytes().to_vec(); 
    
    let mut path: String = String::new();
    let mut query: Option<String> = None;

    for i in (0..=vurl.len() - 1).rev() {
        match vurl[i] {
            b'/' => {
                path = vurl_raw.clone();
                query = None;

                break;
            },

            b'?' => {
                path = String::from_utf8(vurl[0..i].to_vec()).unwrap();
                query = Some(String::from_utf8(vurl[i..vurl.len()].to_vec()).unwrap());
            
                break;
            },

            _ => {}
        }
    }

    return ReqData { path: path, query: query };
}

fn url_query_to_entries(query: &str) -> Vec<Entry> {
    let mut result: Vec<Entry> = Vec::new();
    let mut iskey: bool = true;

    let mut key: String = String::new();

    for entry in query.to_string().split("=") {
        if iskey {
            key = entry.to_string();
        } else {
            result.push(Entry { 
                key: key.clone(), value: entry.to_string() 
            });
        }
        iskey = !iskey;
    }

    return result;
}

fn getreqinfo(path: &str) -> ReqInfo {
    let data: ReqData = url_query(path);
    
    let info: ReqInfo;
    if data.query != None {
        info = ReqInfo {
            entries: Some(url_query_to_entries(&data.query.clone().unwrap()[1..])),
            data: data,
        };
    } else {
        info = ReqInfo {
            entries: None,
            data: data
        };
    }

    return info;
}

fn listen_dir(port: &str, dir: &str, comm: Arc<Mutex<u8>>) -> Result<(), std::io::Error> {
    let path: PathBuf = canonicalize(PathBuf::from_str(dir).unwrap())?;
    let server: Server = Server::http(format!("127.0.0.1:{port}")).unwrap();

    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Get | Method::Post => {
                let mut req: String = request.url().to_string();
                req = match req.as_str() {
                    "/" => {
                        "/index.html".to_string()
                    },

                    _ => {
                        req
                    }
                };
                
                let mut path_to: String = String::from_str(path.join(
                                    req.strip_prefix("/").unwrap()
                                    ).to_str().unwrap()).unwrap();
                
                
                let mut info: ReqInfo = getreqinfo(&path_to);
                path_to = info.data.path;

                if *request.method() == Method::Post {
                    let mut body: String = String::new();
                    request.as_reader().read_to_string(&mut body)?;


                    let mut entries: Vec<Entry> = url_query_to_entries(&body);
                    if !info.entries.is_none() {
                        info.entries.unwrap().append(&mut entries);
                    } else {
                        info.entries = Some(entries);
                    }
                }

                if exists(&path_to)? {
                    // lua functionality here
                    // reqinfo, etc.
                    if path_to.ends_with(".lua") {
                        let response = Response::from_data(pluacgi(&path_to).unwrap());
                        request.respond(response)?;
                    } else {
                        let response = Response::from_file(File::open(path_to)?);
                        request.respond(response)?;
                    }
                } else {
                    let response = Response::from_string("404!");
                    request.respond(response)?;
                }

                if *comm.lock().unwrap() == 1 {
                    return Ok(());
                }
            }, 

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
