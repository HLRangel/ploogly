// SPDX-License-Identifier: MPL-2.0

/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[derive(Debug, Clone)]
pub struct ReqData {
    pub path: String,
    pub query: Option<String>
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub key: String,
    pub value: String
}

#[derive(Debug, Clone)]
pub struct ReqInfo {
    pub data: ReqData,
    pub entries: Option<Vec<Entry>>
}

pub fn url_query(url: &str) -> ReqData {    
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

pub fn url_query_to_entries(query: &str) -> Vec<Entry> {
    let mut result: Vec<Entry> = Vec::new();

    for entry in query.split("&") {
        let kv: Vec<&str> = entry.split("=").collect();
        result.push(Entry { 
            key: kv[0].to_string(), value: kv[1].to_string() 
        });
    }

    return result;
}

pub fn getreqinfo(path: &str) -> ReqInfo {
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