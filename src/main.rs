use std::{fs::read_to_string, io::Write};
use serde::Serialize;
use serde_json::{ser::PrettyFormatter, Value, Serializer, from_str};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, about, version, name = "json-sort", arg_required_else_help = true)]
#[command(help_template = "\
{before-help}{name} 
{author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
")]
struct Args {
    #[arg(short, long, help = "Input json file")]
    input: String,
    #[arg(short, long, help = "Output json file")]
    output: String,
}


fn main() {
    let args: Args = Args::parse();

    // Read the file
    let file: String = match read_to_string(&args.input) {
        Ok(file) => file,
        Err(e) => panic!("Error reading file: {}", e),
    };

    let mut settings: Value = match from_str(&file) {
        Ok(settings) => settings,
        Err(e) => panic!("Error parsing file: {}", e),
    };

    // sort the connections
    let mut connections: Vec<Value> = match settings["datasource.connections"].as_array() {
        Some(connections) => connections.to_vec(),
        None => panic!("Error connections are not an array"),
    };

    let sort_key: &str = "connectionName";

    for connection in &mut connections {
        if connection["options"][sort_key].as_str().unwrap().is_empty() {
            connection["options"][sort_key] = connection["options"]["server"].clone();
        }
    };

    connections.sort_by(|a: &Value, b: &Value| {
        let a: &str = match a["options"][sort_key].as_str() {
            Some(a) => a,
            None => panic!("Error 'server' is not a string"),
        };

        let b: &str = match b["options"][sort_key].as_str() {
            Some(b) => b,
            None => panic!("Error 'server' is not a string"),
        };

        a.cmp(b)
    });

    // replace the connections
    settings["datasource.connections"] = Value::Array(connections);

    // write the file
    let mut file = match std::fs::File::create(args.output){
        Ok(file) => file,
        Err(e) => panic!("Error creating file: {}", e),
    };

    // formatter
    let mut buf: Vec<u8> = Vec::new();
    let formatter: PrettyFormatter = PrettyFormatter::with_indent(b"\t");
    let mut ser: Serializer<&mut Vec<u8>, PrettyFormatter> = Serializer::with_formatter(&mut buf, formatter);
    settings.serialize(&mut ser).unwrap();

    match file.write_all(&buf) {
        Ok(_) => (),
        Err(e) => panic!("Error writing file: {}", e),
    };
}
