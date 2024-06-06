use yaml_rust2::{YamlLoader, YamlEmitter};

fn main() {
    let file = 
"
name: test
list:
    - one
    - 2
";
    let docs = YamlLoader::load_from_str(file).unwrap();

    let doc = &docs[0];

    println!("{:?}", doc);
}
