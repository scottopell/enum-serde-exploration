use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct DogStatsDConfig {
    num_contexts: u32,
    num_messages: u32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct Config {
    generator: GeneratorConfig,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GeneratorConfig {
    Static {
        static_path: PathBuf,
    },
    Ascii,
    #[serde(alias = "dogstatsd")]
    DogStatsD(DogStatsDConfig),
}

fn serialize_deserialize(c: Config) -> Result<(), Box<dyn std::error::Error>> {
    let str_yaml_config = serde_yaml::to_string(&c)?;
    let str_json_config = serde_json::to_string_pretty(&c)?;
    println!("Serialized yaml:\n {}", str_yaml_config);
    println!("Serialized json:\n {}", str_yaml_config);

    match serde_yaml::from_str::<Config>(&str_yaml_config) {
        Ok(c) => println!("De-serialized the yaml just fine: {c:?}"),
        Err(e) => println!("Failed to deserialize yaml with err: {e}"),
    };

    match serde_json::from_str::<Config>(&str_json_config) {
        Ok(c) => println!("De-serialized the json just fine: {c:?}"),
        Err(e) => println!("Failed to deserialize json with err: {e}"),
    };

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let c: Config = Config {
        generator: GeneratorConfig::DogStatsD(DogStatsDConfig {
            num_contexts: 20,
            num_messages: 20,
        }),
    };
    serialize_deserialize(c)?;

    Ok(())
}
