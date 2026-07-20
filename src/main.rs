/*
    Project: Ploogly
    Description: Ad-hoc templating engine for HTML documents.
    Author: HLRangel

    Project notes:

        - We need a way to determine if something is HTML, MD,
        etc, that doesn't just involve blind trust is .ends_with()
        Issue present in: bases/data.rs, file.rs
*/

mod bases;
mod build;
mod commands;
mod file;
mod interpreter_facilities;
mod md2html;
mod new;
mod produce;
mod serve;
mod var_imports;
mod misc;

#[macro_use]
mod debug;

use crate::build::*;
use crate::new::*;
use crate::serve::listen::*;
use crate::commands::command_table::COMMANDS;

use std::env::args;

fn main() -> Result<(), u8> {
    let args: Vec<String> = args().collect();

    println!(
        "Ploogly Distribution/Rolling (Version {}, nominally {})\n\n\
            This build of Ploogly contains third-party software,\n\
            which may be subject to different licensing terms.\n\
            run Ploogly with the \"licenses\" argument for details.\n\n",
	env!("PLOOGLY_COMMIT_ID"),
	env!("CARGO_PKG_VERSION")
    );
    if args.len() >= 2 {
        match args[1].as_str() {
            "new" => {
                if args.len() >= 3 {
                    match new(args[2].clone()) {
                        Err(err) => {
                            eprintln!("Error when generating new project (bare): {err}");
                            return Err(1);
                        }

                        _ => {
                            println!("Project created!");
                        }
                    }
                } else {
                    println!("Directory name not specified!");
                    return Err(1);
                }
            }

            "build" => {
                match build() {
                    Err(err) => {
                        eprintln!("Error when building project: {err}");
                        return Err(1);
                    }

                    _ => (),
                };

                println!("Project built!");
            }

            "serve" => {
                if args.len() >= 3 {
                    if args[2].parse::<u16>().is_ok() {
                        match serve_control(args[2].clone()) {
                            Err(err) => {
                                eprintln!("Error when serving: {err}");
                                return Err(1);
                            }

                            _ => (),
                        };
                    } else {
                        eprintln!("Invalid serve port!");
                        return Err(1);
                    }
                } else {
                    eprintln!("Serve port not specified!");
                    return Err(1);
                }
            }

            "licenses" => {
                println!(
                    "Ploogly (c) 2025-2026 HLRangel and Contributors. All Rights Reserved.\n\
                     This is a version of Ploogly for PRIVATE USE in testing.\n\
                    ---\n\n\
                    Ploogly uses the following third-party packages:\n\
                    markdown-rs Copyright (c) 2022 Titus Wormer <tituswormer@gmail.com>\n\
                    tiny_http Copyright (c) 2014-2019 The tiny-http contributors\n\
		    serde Copyright (c) The Serde contributors\n\
                    mlua Copyright (c) 2019-2021 A. Orlenko, Copyright (c) 2017 rlua\n\n\
                    markdown-rs, mlua, serde, and tiny_http are under the\n\
		    \"MIT License\":\n\
                    \n\
                    Permission is hereby granted, free of charge, to any person obtaining\n\
                    a copy of this software and associated documentation files (the\n\
                    'Software'), to deal in the Software without restriction, including\n\
                    without limitation the rights to use, copy, modify, merge, publish,\n\
                    distribute, sublicense, and/or sell copies of the Software, and to\n\
                    permit persons to whom the Software is furnished to do so, subject to\n\
                    the following conditions:\n\
                    \n\
                    The above copyright notice and this permission notice shall be\n\
                    included in all copies or substantial portions of the Software.\n\
                    \n\
                    THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND,\n\
                    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF\n\
                    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.\n\
                    IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY\n\
                    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,\n\
                    TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE\n\
                    SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE."
                )
            }

            "commands" => {
                println!("List of all available commands.\n\
                Many of these are machine-generated. REFER TO OFFICIAL DOCUMENTATION!\n");

                for cmd in COMMANDS {
                    println!("  {} {}", cmd.name, cmd.signature);
                    println!("      {}\n", cmd.description);
                }
            }

            _ => {
                eprintln!(
                    "Unrecognized argument! Available arguments:\n\
                new, build, serve, licenses"
                );
                return Err(1);
            }
        };
    } else {
        eprintln!("No arguments provided.");
        return Err(1);
    }

    return Ok(());
}
