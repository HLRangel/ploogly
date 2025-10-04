// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::interpreter_facilities::*;

use crate::commands::if_n_def::*;
use crate::commands::include::*;
use crate::commands::redir::*;
use crate::commands::set::*;
use crate::commands::template::*;
use crate::commands::truncate::*;
use crate::commands::unset::*;
use crate::commands::var::*;
use crate::commands::iter_dir::*;
use crate::commands::add_document::*;
use crate::commands::produce_base::*;
use crate::commands::load_base::*;
use crate::commands::cutbase_extension::*;
use crate::commands::sortbase_by_key::*;
use crate::commands::iter_base::*;
use crate::commands::gen_doc::*;
use crate::commands::reverse_base_order::*;
use crate::commands::ltrim::*;
use crate::commands::rtrim::*;
use crate::commands::call::*;

use std::collections::HashMap;
use std::io::ErrorKind;

/* Some obvious issues here... particularly the inconsistencies
between passing the result vec pointer and copying + appending*/

pub fn produce(
    origin: &[u8],
    vars: &mut HashMap<String, Vec<u8>>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();
    let mut current: usize = 0;
    let mut last: usize = 0;

    while current < origin.len() {
        match origin[current] {
            b'{' => {
                if !is_eof(origin, current + 1) && origin[current + 1] == b'{' {
                    current += 2;

                    let command: String = get_word(origin, &mut last, &mut current)?;

                    if !is_eof(origin, current) {
                        match command.as_str() {
                            "include" => {
                                include(
                                    &mut result,
                                    origin,
                                    &mut last,
                                    &mut current,
                                    vars,
                                    anon_stack,
                                )?;
                            }

                            "var" => {
                                var(
                                    &mut result,
                                    origin,
                                    &mut last,
                                    &mut current,
                                    vars,
                                    anon_stack,
                                )?;
                            }

                            "set" => {
                                set(origin, &mut current, &mut last, vars, anon_stack)?;
                            }

                            "unset" => {
                                unset(origin, &mut last, &mut current, vars)?;
                            }

                            "ifdef" => {
                                let mut tores: Vec<u8> = ifdef(
                                    origin,
                                    &mut current,
                                    &mut last,
                                    vars,
                                    anon_stack,
                                )?;
                                result.append(&mut tores);
                            }

                            "ifndef" => {
                                let mut tores: Vec<u8> = ifndef(
                                    origin,
                                    &mut current,
                                    &mut last,
                                    vars,
                                    anon_stack,
                                )?;
                                result.append(&mut tores);
                            }

                            "template" => {
                                let mut tores: Vec<u8> = template(
                                    origin,
                                    &mut last,
                                    &mut current,
                                    vars,
                                    anon_stack,
                                )?;
                                result.append(&mut tores);
                            }

                            "truncate" => {
                                let mut tores: Vec<u8> = truncate(
                                    origin,
                                    &mut last,
                                    &mut current,
                                    vars,
                                    anon_stack,
                                )?;
                                result.append(&mut tores);
                            }

                            "redir" => {
                                redir(origin, &mut last, &mut current, vars)?;
                            }

                            "iter_dir" => {
                                let mut tores: Vec<u8> = iter_dir(origin, &mut last, &mut current, vars, anon_stack)?;

                                result.append(&mut tores);
                            }

			    "add_document" => {
				add_document(origin, &mut last, &mut current, vars, anon_stack)?;
			    }

			    "produce_base" => {
				produce_base_cmd(origin, &mut last, &mut current, vars, anon_stack)?;
			    }

			    "load_base" => {
				load_base(&mut result, origin, &mut last, &mut current)?;
			    }

			    "cutbase_extension" => {
				cutbase_extension(origin, &mut result, &mut current, &mut last, vars, anon_stack)?;
			    }

			    "sortbase_by_key" => {
				sortbase_by_key(origin, &mut result, &mut current, &mut last, vars, anon_stack)?;
			    }

			    "reverse_base_order" => {
				reverse_base_order(origin, &mut result, &mut current, &mut last, vars,anon_stack)?;
			    }

			    "iter_base" => {
				iter_base(origin, &mut result, &mut last, &mut current, vars, anon_stack)?;
			    }

			    "gen_doc_from_template" => {
				gen_doc_from_template(origin, &mut last, &mut current, vars, anon_stack)?;
			    }

			    "ltrim" => {
				ltrim(origin, &mut result, &mut current, &mut last, vars, anon_stack)?;
			    }

			    "rtrim" => {
				rtrim(origin, &mut result, &mut current, &mut last, vars, anon_stack)?;
			    }

			    "call" => {
				call(origin, &mut current, &mut last, vars, anon_stack)?;
			    }

                            _ => {
                                return Err(ErrorKind::InvalidInput.into());
                            }
                        }
                    } else {
                        panic!("Unexpected end of command.");
                    }

                    to_next_notwp(origin, &mut current);
                    if is_twobracket_r(origin, current) {
                        current += 2;
                    }

                    last = current;
                } else {
                    result.push(origin[current]);
                    current += 1;
                }
            }

            b'<' => {
                if is_html_comment_start(origin, current) {
                    current += 4;

                    while !is_html_comment_end(origin, current) && !is_eof(origin, current) {
                        current += 1;
                    }

                    if !is_eof(origin, current) {
                        current += 3;
                    }
                } else {
                    result.push(origin[current]);
                    current += 1;
                }
            }

            _ => {
                result.push(origin[current]);
                current += 1;
            }
        };
    }

    return Ok(result);
}
