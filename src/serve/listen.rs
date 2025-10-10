use crate::build::*;
use crate::serve::data::*;
use crate::serve::lua_gen::*;

use std::fs::{File, canonicalize, exists};
use std::io::ErrorKind;
use std::io::stdin;
use std::string::*;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::{path::PathBuf, str::FromStr};
use tiny_http::{Method, Response, Server};

fn listen_dir(port: &str, dir: &str, comm: Arc<Mutex<u8>>) -> Result<(), std::io::Error> {
    let path: PathBuf = canonicalize(PathBuf::from_str(dir).unwrap())?;
    let server: Server = match Server::http(format!("127.0.0.1:{port}")) {
	Ok(server) => server,
	Err(_) => return Err(ErrorKind::AddrInUse.into()) 
    };

    for mut request in server.incoming_requests() {
        match request.method() {
            Method::Get | Method::Post => {
                let mut req: String = request.url().to_string();
                req = match req.as_str() {
                    "/" => "/index.html".to_string(),

                    _ => req,
                };

                let mut path_to: String =
                    String::from_str(path.join(req.strip_prefix("/").unwrap()).to_str().unwrap())
                        .unwrap();

                if path_to.ends_with("/") {
                    path_to.push_str("index.html");
                }

                let mut info: ReqInfo = getreqinfo(
                    &path_to,
                    TinyHTTPMethod(request.method().clone()).to_reqmethod(),
                );
                path_to = info.data.path.clone();

                if *request.method() == Method::Post {
                    let mut body: String = String::new();
                    request.as_reader().read_to_string(&mut body)?;

                    let mut entries: Vec<Entry> = url_query_to_entries(&body);
                    if !info.entries.is_none() {
                        let mut ne: Vec<Entry> = info.entries.clone().unwrap();
                        ne.append(&mut entries);

                        info.entries = Some(ne);
                    } else {
                        info.entries = Some(entries);
                    }
                }

                if exists(&path_to)? {
                    // lua functionality here
                    // reqinfo, etc.
                    if path_to.ends_with(".lua") {
                        let response = Response::from_data(pluacgi(&path_to, &info).unwrap());
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
            }

            _ => {
                eprintln!("Came across an unsupported request type. Ignored.");
            }
        };
    }

    return Ok(());
}

pub fn serve_control(port: String) -> Result<(), std::io::Error> {
    if exists("./out/site")? {
        let sp: Arc<Mutex<u8>> = Arc::new(Mutex::new(0));
        let rp: Arc<Mutex<u8>> = sp.clone();

        let mut ended: bool = false;

        println!(
            "Serving on 127.0.0.1:{}, access http://127.0.0.1:{} to view! \
        \nPress R, then Enter to rebuild the project \
        \nPress L, then Enter to properly quit after serving the next request. \
        \nPress Q, then Enter to force quit.\n\n",
            &port, &port
        );

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
                }

                b'L' | b'l' => {
                    *sp.lock().unwrap() = 1;

                    ended = true;
                }

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
