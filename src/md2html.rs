use std::collections::HashMap;

use crate::interpreter_facilities::*;
use crate::var_imports::import_variables;

pub fn get_frontmatter_block(
    origin: &[u8],
    last: &mut usize,
    current: &mut usize,
) -> Result<Vec<u8>, std::io::Error> {
    // Check for opening `---` possibly followed by spaces and a newline
    if *current + 3 > origin.len() || &origin[*current..*current + 3] != b"---" {
        return Err(ErrorKind::InvalidInput.into());
    }
    let mut pos = *current + 3;
    // skip trailing spaces before newline
    while pos < origin.len() && origin[pos] == b' ' {
        pos += 1;
    }
    if pos >= origin.len() || origin[pos] != b'\n' {
        return Err(ErrorKind::InvalidInput.into());
    }
    // move past the newline
    pos += 1;
    *current = pos;
    *last = pos;

    // scan for closing `---` line
    while !is_eof(origin, *current) {
        if origin[*current] == b'\n' {
            let after_nl = *current + 1;
            if after_nl + 3 <= origin.len() && &origin[after_nl..after_nl + 3] == b"---" {
                let mut end = after_nl + 3;
                // trailing spaces after the three dashes are allowed before the final newline
                while end < origin.len() && origin[end] == b' ' {
                    end += 1;
                }
                if end < origin.len() && origin[end] == b'\n' {
                    // found closing line
                    let content = origin[*last..*current].to_vec();
                    *current = end + 1; // move past closing newline
                    return Ok(content);
                }
            }
        }
        *current += 1;
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
