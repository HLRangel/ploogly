use crate::interpreter_facilities::*;
use crate::commands::command_table::COMMANDS;
use crate::commands::CommandContext;

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
                        let mut ctx = CommandContext {
                            origin,
                            current,
                            last,
                            vars,
                            anon_stack,
                            result: &mut result,
                        };

                        let mut handled = false;
                        for cmd in COMMANDS {
                            if command == cmd.name {
                                (cmd.handler)(&mut ctx)?;
                                handled = true;
                                // update local variables from context
                                current = ctx.current;
                                last = ctx.last;
                                break;
                            }
                        }
                        if !handled {
                            return Err(ErrorKind::InvalidInput.into());
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
