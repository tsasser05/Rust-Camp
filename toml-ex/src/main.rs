use std::fs;
use toml;

//struct Config {
     ip: String,
    fqdn: String,
}

fn main() {
    let file_contents =
        fs::read_to_string("data/config.toml").expect("Error: Cannot read the file");
    //println!("data/config.toml context =\n{file_contents}");

    let config: &str = toml::from_str(file_contents.as_str()).unwrap();
} // main()

//assert!(config.ip, "127.0.0.1");
//assert!(config.fqdn, "foo.bar.com");

// Works from toml_parse
//use toml_parse::{parse_it, TomlKind};
//let root_node = parse_it(&file_contents).unwrap().syntax();
//assert_eq!(root_node.first_child().unwrap().kind(), TomlKind::Table)

//let config: HashMap<String, String>
