use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize,Debug)]
struct ServerConfig {
    worker: u64,
    ignore: bool,
    auth_server:Option<String>,
}

fn main() {
    let config = ServerConfig{
        worker: 100,
        ignore: false,
        auth_server:Some("auth.server.io".to_string()),
    };

    {
        println!("json:");
        let serialized = serde_json::to_string(&config).unwrap();
        println!("serialized:\n{}", serialized);

        let deserialized: ServerConfig = serde_json::from_str(&serialized).unwrap();
        println!("deserialized:\n{:#?}", deserialized);
    }

    {
        println!("yaml:");
        let serialized = serde_yaml::to_string(&config).unwrap();
        println!("serialized:\n{}", serialized);

        let deserialized: ServerConfig = serde_yaml::from_str(&serialized).unwrap();
        println!("deserialized:\n{:#?}", deserialized);
    }

    println!("Hello, world!");
}
