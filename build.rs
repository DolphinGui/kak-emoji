use serde::Deserialize;
use std::{fs::File, io::Write, path::Path};
use trie_rs::map::{Trie, TrieBuilder};

#[derive(Deserialize, Debug)]
struct Emoji {
    emoji: String,
    aliases: Vec<String>,
}

fn main() {
    println!("cargo::rerun-if-changed=resources/emoji.json");
    let emojidata = include_str!("resources/emoji.json");
    let emojidb: Vec<Emoji> =
        serde_json::from_str(emojidata).expect("Error parsing Emoji database");
    let mut builder = TrieBuilder::new();
    for emoji in emojidb {
        for alias in emoji.aliases {
            builder.push(alias, emoji.emoji.clone());
        }
    }
    let trie: Trie<u8, String> = builder.build();
    let data = bincode::serialize(&trie).expect("Error serializing trie");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let path = Path::new(&out_dir).join("emojidb.bin");
    let mut file = File::create(path).unwrap();
    file.write_all(&data).unwrap();
}
