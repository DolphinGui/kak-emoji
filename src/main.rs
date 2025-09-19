use bincode::{deserialize, deserialize_from};
use trie_rs::map::Trie;

fn main() {
    let binary = include_bytes!(concat!(env!("OUT_DIR"), "/emojidb.bin"));
    let emojidb: Trie<u8, String> = deserialize(binary).unwrap();
    let mut args = std::env::args();
    let s = args.nth(1).expect("Expected one argument");
    let results: Vec<(String, &String)> = emojidb.predictive_search(s).collect();
    println!("{:?}", results);
}
