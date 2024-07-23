use pest::Parser;
use std::fs;
use DML::DMLParser;
use DML::Processer;
use DML::Rule;

fn main() {
    let input = fs::read_to_string("./test.dml").expect("Failed to read file");
    let dml = DMLParser::parse(Rule::dml, &input).expect("Failed to parse");
    let mut process = Processer::new();
    process.process_dml(dml.into_iter());

    // println!("{:?}", process.variable_map);
    // println!("{:?}", process.item_map);
    // println!("{:?}", process.group_map);

    // to json
    let json = serde_json::to_string_pretty(&process).unwrap();
    println!("{}", json);

    // to file
    fs::write("./test.json", json).expect("Unable to write file");
}
