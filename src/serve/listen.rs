use crate::build::*;
use actix_web::{web, App, HttpServer};
use actix_files::Files;
use actix_web::rt;
use std::fs::{canonicalize, exists};
use std::io::{ErrorKind, stdin};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread::spawn;
use std::time::Duration;

fn listen_dir(port: &str, dir: &str, comm: Arc<Mutex<u8>>) -> Result<(), std::io::Error> {
    let path: PathBuf = canonicalize(PathBuf::from_str(dir).unwrap())?;

    let server = HttpServer::new(move || {
        let static_path = path.clone();
        App::new()
            .service(Files::new("/", static_path).index_file("index.html"))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run();

    let handle = server.handle();
    let comm_clone = Arc::clone(&comm);
    rt::spawn(async move {
        loop {
            if *comm_clone.lock().unwrap() == 1 {
                handle.stop(true);
                break;
            }
            rt::time::sleep(Duration::from_millis(500)).await;
        }
    });

    rt::System::new("http-server").block_on(server)?;
    Ok(())
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

    Ok(())
}
