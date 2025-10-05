use bincode::deserialize;
use trie_rs::map::Trie;

const HELP_STRING: &str = "kak-emoji: a kakoune emoji insertion tool:
  usage: kak-emoji [FLAG] [ARG]
  flags:
    --help: show this help
    --list-names: list names of all emojis
    --emit: print an emoji given it's name
    --script: print a kak-script to load this plugin";

const KAK_SCRIPT: &str = r#"
define-command insert-text -params 1 -hidden %{
  evaluate-commands -save-regs '"' %{
    set-register '"' %arg{1}
    execute-keys -draft ';P'
  }
}

define-command -docstring "Prompt ot insert emojis" emoji-insert-prompt %{
  prompt -shell-script-candidates %{kak-emoji --list-names} "Emoji name:" %{
    insert-text %sh{ kak-emoji --emit $kak_text }
  }
}


define-command -docstring "Insert emojis" -params 1 emoji-insert %{
  insert-text %sh{
    kak-emoji --emit $@
  }
}


complete-command -menu emoji-insert shell-script-candidates %{
  kak-emoji --list-names
}"#;

const BINARY: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/emojidb.bin"));

fn main() {
    let mut args = std::env::args();
    // skip the 0th argument, which is usually the bin
    let s = args.nth(1).unwrap_or("--help".into());
    let arg = args.next();
    match s.as_str() {
        "--list-names" => list_names(),
        "--emit" => {
            let arg = arg.expect("Expected emoji name");
            emit_emoji(&arg)
        }
        "--script" => println!("{}", KAK_SCRIPT),
        _ => println!("{}", HELP_STRING),
    }
}

fn list_names() {
    let emojidb: Trie<u8, String> = deserialize(BINARY).unwrap();
    let candidates: Vec<String> = emojidb.iter().map(|(name, _)| name).collect();
    for candidate in candidates {
        println!("{}", candidate);
    }
}

fn emit_emoji(name: &str) {
    let emojidb: Trie<u8, String> = deserialize(BINARY).unwrap();
    let emoji = emojidb
        .exact_match(name)
        .unwrap_or_else(|| panic!("Expected exact match, got {}", &name));
    println!("{}", emoji);
}
