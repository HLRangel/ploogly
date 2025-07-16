// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use crate::file::*;

use std::env::set_current_dir;
use std::fs::*;
use std::io::{Write, stdin, stdout, ErrorKind};

fn new_project(bare: bool) -> Result<(), std::io::Error>{
    if !exists("project.ssg")? {
    create_dir("./site")?;
    create_dir("./out")?;

    create_file_from_str("./project.ssg", 
"name: Example Website\n\
url: https://example.com/");

    if !bare {
    create_dir("./templates")?;
    create_dir("./posts")?;
    
    create_file_from_str("./posts/welcome.md",
"---\n\
title: My first ploogly post\n\
description: Templating for creativity\n\
---\n\n\
# Hello!\n\
This is an example post, written using Markdown, please refer\
to the documentation for support.");

    create_file_from_str("./templates/post_example.html",
"<html>\n\
    \t<h1>{{ var title }}</h1>\n\
    \t{{ var docdata }}\n\
</html>\n");
    
    create_file_from_str("./templates/head.html",
"\
<head>
    <style>
        body {
            font-family: 'Georgia', serif, sans-serif;
            font-size: 14pt;
        }

        .main-text {
            width: 40vw;
        }
    </style>

    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">

    <meta property=\"og:title\" content=\"{{ var title }} | {{ var name }}\" />
    <meta property=\"og:type\" content=\"website\" />
    <meta property=\"og:url\" content=\"{{ var url }}{{ var path }}\"/>

    <title>{{ var title }} | {{ var name }}</title>
</head>");
    
    create_file_from_str("./templates/post.html", 
"\
<!DOCTYPE html>
<html>
    {{ include templates/head.html }}

    <body>
        <h1>{{ var title }}</h1>
        <p><a href=\"/index.html\">To Index</a></p>

        <hr>

        {{ var docdata }}
    </body>
</html>");

    create_file_from_str("./posts/post1.md", 
"\
---
title: My First Blog Post
---

# Hi!

This is a post to test out the Ploogly templating engine.");

    create_file_from_str("./site/index.html", 
"\
<!DOCTYPE html>
<html>
    {{ set title \"Main Page\" }}
    {{ include templates/head.html }}

    <body>
        <h1>{{ var name }}</h1>
        <hr>

        <p>Welcome to {{ var name }}, it's an example website made with Ploogly.</p>
        {{ set motd \"Live, Laugh, Love\" }}
        
        {{ ifdef motd 
            <p>Message of the Day: <i>{{ var motd }}</i></p>
        }}
        
        <h2>Featured Post</h2>

        {{ list_doc doc posts/post1.md templates/post.html
            <h3><a href=\"{{ var path }}\">{{ var title }}</a></h3>
        }}

        <h2>Posts</h2>

        {{ list_doc docs_in posts templates/post.html title
            <h3><a href=\"{{ var path }}\">{{ var title }}</a></h3>
        }}
    </body>
</html>");
    }
    } else {
        set_current_dir("..")?;
        return Err(ErrorKind::AlreadyExists.into());
    }

    return Ok(());
}

fn message_line_input(msg: &str) -> Result<String, std::io::Error> {
    let mut conf: String = String::new();
    
    print!("{msg}");

    stdout().flush()?;
    stdin().read_line(&mut conf)?;

    return Ok(conf);
}

pub fn new() -> Result<(), std::io::Error> {
    if !exists("./project.ssg")? {
        let conf: String = message_line_input("This will create a new \
        project\x1b[1m in this directory.\x1b[0m Are you sure? [y/N] ")?;
    
        if conf.as_bytes()[0] == b'y' || conf.as_bytes()[0] == b'Y' {
            println!("Ok, creating...");
            new_project(false)?;
            println!("Project created successfully.");
        } else {
            println!("Project creation cancelled.");
        }
    } else {
        return Err(ErrorKind::AlreadyExists.into());
    }

    return Ok(());
}

pub fn new_bare(name: String) -> Result<(), std::io::Error> {
    if !exists(&name)? {
        create_dir(&name)?;
    }

    set_current_dir(&name)?;
    new_project(true)?;

    return Ok(());
}