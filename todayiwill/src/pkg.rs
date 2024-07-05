use std::str;

use curl::easy::Easy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Tag {
    pub name: String,
}

pub fn update() {
    let mut easy = Easy::new();
    easy.url("https://api.github.com/repos/vncsmyrnk/todayiwill/tags").unwrap();
    easy.useragent("todayiwill-agent").unwrap();
    easy.write_function(|data| {
        println!("{}", str::from_utf8(data).unwrap());
        let mut tags: Vec<Tag> = serde_json::from_str(str::from_utf8(data).expect("Failed to covert &[u8] into &str")).expect("Failed to parse JSON");
        println!("latest release: {}; Current version: {}", tags.pop().unwrap().name, env!("CARGO_PKG_VERSION"));
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();
    println!("End of update");
}
