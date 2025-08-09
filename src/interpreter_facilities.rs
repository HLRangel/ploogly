// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

// Shared word-wrangling facilities

use crate::docdata::*;
use crate::produce::*;

use std::collections::HashMap;
use std::{io::ErrorKind, str::FromStr};

pub fn is_eof(origin: &[u8], val: usize) -> bool {
    val >= origin.len()
}

/*
pub fn lineno(origin: &[u8], current: usize) -> u64 {
    let mut pos: usize = 0;
    let mut linec: u64 = 1;
    while !is_eof(origin, pos) && pos <= current {
        if origin[pos] == b'\n' {
            linec += 1;
        }

        pos += 1;
    }

    return linec;
} */

pub fn is_twobracket_r(origin: &[u8], current: usize) -> bool {
    origin[current] == b'}' && (!is_eof(origin, current) && origin[current + 1] == b'}')
}

pub fn is_terminator(origin: &[u8], current: &mut usize) -> bool {
    is_eof(origin, *current) || is_twobracket_r(origin, *current)
}

pub fn to_next_notwp(origin: &[u8], current: &mut usize) {
    while !is_terminator(origin, current) && origin[*current] == b' ' {
        *current += 1;
    }
}

pub fn to_next_wp_or_nl(origin: &[u8], current: &mut usize) {
    while !is_terminator(origin, current) && origin[*current] != b' ' && origin[*current] != b'\n' {
        *current += 1;
    }
}

/*  Given the position right after two opening brackets,
    match all upcoming brackets so as to complete the pair of braces.

    {{ {{ {{ }} }} }}
      ^              ^
      |current       |
                     | current, after function.

*/
pub fn match_twobrackets(origin: &[u8], current: &mut usize) {
    while !is_eof(origin, *current) {
        match origin[*current] {
            b'{' => {
                if !is_eof(origin, *current + 1) && origin[*current + 1] == b'{' {
                    *current += 2;
                    match_twobrackets(origin, current);
                }
            }

            b'}' => {
                if !is_eof(origin, *current + 1) && origin[*current + 1] == b'}' {
                    *current += 2;
                    return;
                }
            }

            _ => {
                *current += 1;
            }
        }
    }
}

// Commute all CR-LFs to LFs
pub fn to_normalized_vec(origin: &[u8]) -> Vec<u8> {
    let mut toret: Vec<u8> = Vec::new();
    let mut current: usize = 0;

    while !is_eof(origin, current) {
        if origin[current] == b'\r'
            && (!is_eof(origin, current + 1) && origin[current + 1] == b'\n')
        {
            toret.push(b'\n');
            current += 2;
        } else {
            toret.push(origin[current]);
            current += 1;
        }
    }

    toret
}

pub fn get_word(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<String, std::io::Error> {
    to_next_notwp(origin, current);
    *last = *current;

    if !is_terminator(origin, current) {
        to_next_wp_or_nl(origin, current);
    } else {
        return Err(ErrorKind::InvalidInput.into());
    }

    let ret: String =
        String::from_utf8(origin[*last..*current].to_vec()).expect("Err on utf8 conversion!");

    *last = *current;

    Ok(ret)
}

pub fn highlight_word_ltr(origin: &[u8], current: usize) -> String {
    let mut new_last: usize = current;
    if current > 0 {
        while new_last > 0 && origin[new_last] != b' ' && origin[new_last] != b'\n' {
            new_last -= 1;
        }

        if origin[new_last] == b'\n' {
            new_last += 1;
        }

        return String::from_utf8(origin[new_last..current].to_vec())
            .expect("Err on utf8 conversion!");
    }

    String::from_str("undefined").unwrap()
}

pub fn get_inner(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<Vec<u8>, std::io::Error> {
    while !is_eof(origin, *current) && origin[*current] != b'\n' && origin[*current] != b'\\' {
        *current += 1;
    }

    if !is_eof(origin, *current + 1) {
        *current += 1;
    } else {
        return Err(ErrorKind::InvalidInput.into());
    }

    *last = *current;

    match_twobrackets(origin, current);

    Ok(origin[*last..(*current - 2)].to_vec())
}

pub fn quote_literal(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<String, std::io::Error> {
    *last = *current;

    while !is_eof(origin, *current) && origin[*current] != b'"' {
        match origin[*current] {
            b'\\' => {
                *current += 2;
            }

            b'\n' => {
                return Err(ErrorKind::InvalidInput.into());
            }

            _ => {
                *current += 1;
            }
        }
    }

    if !is_eof(origin, *current) {
        *current += 1;
    }

    Ok(String::from_utf8(origin[*last..(*current - 1)].to_vec())
        .expect("Error on utf-8 conversion!"))
}

pub fn get_word_or_literal(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<String, std::io::Error> {
    to_next_notwp(origin, current);

    if origin[*current] == b'"' {
        *current += 1;
        quote_literal(origin, last, current)
    } else {
        get_word(origin, last, current)
    }
}

pub fn is_char_n(origin: &[u8], val: usize, n: u8) -> bool {
    !is_eof(origin, val) && origin[val] == n
}

pub fn is_n_chars(origin: &[u8], current: usize, charto: u8, occ: usize) -> bool {
    if occ < 1 {
        return false;
    }

    let mut ok: bool = true;
    for i in current..current + (occ - 1) {
        if is_eof(origin, i) || !is_char_n(origin, i, charto) {
            ok = false;
        }
    }

    ok
}

pub fn is_n_chars_before_n(
    origin: &[u8],
    current: usize,
    charto: u8,
    occ: usize,
    charbe: u8,
) -> bool {
    is_n_chars(origin, current, charto, occ)
        && !is_eof(origin, current + 1 + (occ - 1))
        && origin[current + 1 + (occ - 1)] == charbe
}

pub fn is_three_rarrow(origin: &[u8], current: usize) -> bool {
    is_char_n(origin, current, b'>')
        && is_char_n(origin, current + 1, b'>')
        && is_char_n(origin, current + 2, b'>')
}

pub fn nl_into_br(origin: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    let mut current: usize = 0;

    while !is_eof(origin, current) {
        match origin[current] {
            b'\n' => {
                result.append(&mut ("<br>".as_bytes().to_vec()));
                current += 1;
            }

            _ => {
                result.push(origin[current]);
                current += 1;
            }
        }
    }

    result
}

pub fn get_data_to_end(origin: &[u8], last: &mut usize, current: &mut usize) -> Vec<u8> {
    *last = *current;

    while !is_eof(origin, *current) {
        *current += 1;
    }

    origin[*last..*current].to_vec()
}

pub fn is_html_comment_start(origin: &[u8], current: usize) -> bool {
    is_char_n(origin, current, b'<')
        && is_char_n(origin, current + 1, b'!')
        && is_n_chars(origin, current + 2, b'-', 2)
}

pub fn is_html_comment_end(origin: &[u8], current: usize) -> bool {
    is_n_chars(origin, current, b'-', 2) && is_char_n(origin, current + 2, b'>')
}

pub fn get_worl_produce(
    origin: &[u8],
    current: &mut usize,
    last: &mut usize,
    vars: &mut HashMap<String, Vec<u8>>,
    cache: &mut HashMap<String, DocData>,
    anon_stack: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, std::io::Error> {
    let mut varsc: HashMap<String, Vec<u8>> = vars.clone();
    let toprod: String = get_word_or_literal(origin, last, current)?;

    let result: Vec<u8> = produce(toprod.as_bytes(), &mut varsc, cache, anon_stack)?;

    Ok(result)
}

pub fn get_separated_arguments(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<Vec<Vec<u8>>, std::io::Error> {
    if !is_eof(origin, *current) {
        to_next_notwp(origin, current);

        let mut argvc: Vec<Vec<u8>> = Vec::new();

        *last = *current;
        while !is_twobracket_r(origin, *current) {
            let arg: String = get_word_or_literal(origin, last, current)?;
            argvc.push(arg.as_bytes().to_vec());
            to_next_notwp(origin, current);
        }

        return Ok(argvc);
    }

    Err(ErrorKind::InvalidInput.into())
}
