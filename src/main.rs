mod parser;
use parser::EnvPair;

fn main() {
    let raw_env_data = "USER=seungwon.baek\nPATH=/usr/bin:/bin\nSHELL=/bin/zsh";
    let env_pairs = EnvPair::parse(raw_env_data);
    // for env in env_pairs {
    //     println!("{env}");
    // }
    let val = EnvPair::search(&env_pairs, "USER");
    match val {
        Ok(v) => println!("{v}"),
        Err(e) => eprintln!("{e}")
    }
}
