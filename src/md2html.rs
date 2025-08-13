// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/
use std::collections::HashMap;

use crate::interpreter_facilities::*;
use crate::var_imports::import_variables;

pub fn get_frontmatter_block(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<Vec<u8>, std::io::Error> {
    if is_n_chars_before_n(origin, *current, b'-', 3, b'\n') {
        *current += 4;

        *last = *current;

        while !is_eof(origin, *current) {
            if origin[*current] == b'\n'
                && is_n_chars_before_n(origin, *current + 1, b'-', 3, b'\n')
            {
                *current += 4;

                return Ok(origin[*last..*current - 4].to_vec());
            }

            *current += 1;
        }

        return Err(ErrorKind::InvalidInput.into());
    }

    Err(ErrorKind::InvalidInput.into())
}

pub fn get_frontmatter_ctx(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<HashMap<String, Vec<u8>>, std::io::Error> {
    let fmtxt: Vec<u8> = match get_frontmatter_block(origin, last, current) {
        Ok(res) => res,
        Err(_) => "title: Undefined Title".as_bytes().to_vec(),
    };

    import_variables(&fmtxt)
}

/*
    The Ploogly distribution depends on markdown-rs,
    released under the MIT License


    markdown-rs

    Copyright (c) 2022 Titus Wormer <tituswormer@gmail.com>

    Permission is hereby granted, free of charge, to any person obtaining
    a copy of this software and associated documentation files (the
    'Software'), to deal in the Software without restriction, including
    without limitation the rights to use, copy, modify, merge, publish,
    distribute, sublicense, and/or sell copies of the Software, and to
    permit persons to whom the Software is furnished to do so, subject to
    the following conditions:

    The above copyright notice and this permission notice shall be
    included in all copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND,
    EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
    MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
    IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
    CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
    TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
    SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

*/

use markdown::{
    CompileOptions, LineEnding::LineFeed, Options, message::Message, to_html_with_options,
};
use std::io::ErrorKind;

pub fn to_md(origin: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    let mut result: Vec<u8> = Vec::new();

    let res: Result<String, Message> = to_html_with_options(
        String::from_utf8(origin.to_vec()).unwrap().as_str(),
        &Options {
            compile: CompileOptions {
                allow_any_img_src: true,
                allow_dangerous_html: true,
                allow_dangerous_protocol: true,
                default_line_ending: LineFeed,
                gfm_footnote_back_label: Some("To content".to_string()), // get this from project.ssg
                ..CompileOptions::default()
            },
            ..Options::default()
        },
    );

    match res {
        Ok(res) => {
            result.append(&mut res.as_bytes().to_vec());
        }

        Err(_) => {
            return Err(ErrorKind::Other.into());
        }
    }

    Ok(result)
}
