extern crate serde_json;

mod clitc_error;
mod events;
mod params;

use std::{
    time::Instant,
    fs::File,
};
use crate::params::{
    ParamValue,
    CliParameters,
};
use crate::events::{
    WhitespaceSplitter,
    EventHandler,
    Event,
};

fn main() {
    let total = Instant::now();
    println!("CLI Toolchain");
    let data = r#"
    {
        "options": [
            {
                "short": "-e",
                "name": "--example",
                "descr": "Lorem ipsum\nLorem Ipsum dolor est",
                "params": [
                    {
                        "ord": 0,
                        "type": "string"
                    }
                ]
            },
            {
                "short": "-v",
                "name": "--verbose",
                "descr": "Lorem ipsum\nLorem Ipsum dolor est",
                "params": []
            },
            {
                "short": "-l",
                "name": "--lifetime",
                "descr": "Lorem ipsum\nLorem Ipsum dolor est",
                "params": [
                    {
                        "ord": 0,
                        "name": "secs",
                        "type": "int"
                    },
                    {
                        "ord": 1,
                        "name": "expected_val",
                        "type": "num"
                    }
                ]
            }
        ]
    }"#;

    let timer = Instant::now();
    let cli_params = CliParameters::from_str(data).expect("Error occurred");
    let elapsed = timer.elapsed();
    println!("File loading: {}µs", elapsed.as_micros());

    let timer = Instant::now();
    let args = cli_params.parse_args();
    println!("Parsed:");
    for (key, val) in args.iter() {
        println!("{}", key);
        for (k, v) in val {
            println!("\t{}\t{}", k, match v {
                ParamValue::Array(val) => val.join(", "),
                ParamValue::Int(val) => val.to_string(),
                ParamValue::Num(val) => val.to_string(),
                ParamValue::String(val) => val.clone(),
            });
        }
    }
    let elapsed = timer.elapsed();
    println!("Parsing cl args: {}µs", elapsed.as_micros());

    let timer = Instant::now();
    let args = cli_params.parse_str_whitespace("-l 2 2.345 --example MyName");
    println!("Parsed:");
    for (key, val) in args.iter() {
        println!("{}", key);
        for (k, v) in val {
            println!("\t{}\t{}", k, match v {
                ParamValue::Array(val) => val.join(", "),
                ParamValue::Int(val) => val.to_string(),
                ParamValue::Num(val) => val.to_string(),
                ParamValue::String(val) => val.clone(),
            });
        }
    }
    let elapsed = timer.elapsed();
    println!("Parsing command: {}µs", elapsed.as_micros());

    let elapsed = total.elapsed();
    println!("Total time spent: {}µs", elapsed.as_micros());

    println!("\n\n## EventHandler ##\n");
    let config_file = File::open("D:/Dateien/tobias/data/clitc/commands.json").expect("Could not open file");
    let cli_params = CliParameters::from_reader(config_file).expect("Could not parse params");
    let mut evt_handler = EventHandler::new(cli_params, WhitespaceSplitter, true);

    evt_handler.attach("start", Event::Callback(Box::new(|_| {
        println!("Starting service!");
    })));


    evt_handler.attach("exit", Event::Callback(Box::new(|_| {
        println!("Stopping service!");
    })));


    evt_handler.attach("show", Event::Callback(Box::new(|args| {
        match args.get(&String::from("index")) {
            Some(val) => println!("Showing value at index {}...", match val {
                ParamValue::Int(val) => val.to_string(),
                _ => String::from("Wrong value type!"),
            }),
            None => println!("[show] Command needs an index parameter!"),
        };
    })));

    evt_handler.attach("help", Event::InfoCallback(Box::new(|args, mut info| {
        match args.get(&String::from("cmd")) {
            Some(val) => {
                let cmd = val.to_string();
                // Print only for single command
                let lines = match info.get(&cmd) {
                    Some(ln) => ln.clone(),
                    None => vec![format!("Cannot display help for unknown command \"{}\"", cmd)],
                };
                println!("{}", lines.join("\n"));
            },
            None => {
                // Print for all commands
                let mut lines = Vec::new();
                for mut cmd in info.values_mut() {
                    lines.append(&mut cmd);
                }
                println!("{}", lines.join("\n"));
            },
        }
    })));

    evt_handler.pass_command("start".to_string()).expect("Could not pass command");
    evt_handler.pass_command("help".to_string()).expect("Could not pass command");
    evt_handler.pass_command("show 1".to_string()).expect("Could not pass command");
    println!("##");
    evt_handler.pass_command("help show".to_string()).expect("Could not pass command");
}
