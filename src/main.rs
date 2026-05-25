mod parser;
use parser::EnvPair;

fn main() {
    let mut native_str = String::new();
    for (k, v) in std::env::vars() {
       native_str.push_str(&format!("{}={}\n", k, v));
    }
    let env_pairs = EnvPair::parse(&native_str);
    // for env in env_pairs {
    //     println!("{env}");
    // }
    let val = EnvPair::search(&env_pairs, "VSCODE_GIT_ASKPASS_MAIN");
    match val {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}")
    }
}
