use pest::Parser;
use pest_derive::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser)]
#[grammar = "dml.pest"]
pub struct DMLParser;

#[derive(Debug, serde::Serialize)]
struct Group {
    id: String,
    pairs: HashMap<String, String>,
    blocks: HashMap<String, Block>,
    reference: HashMap<String, String>,
}

#[derive(Debug, serde::Serialize)]
struct Item {
    id: String,
    pairs: HashMap<String, String>,
    blocks: HashMap<String, Block>,
}

#[derive(Debug, serde::Serialize)]
struct Block {
    id: String,
    pairs: HashMap<String, String>,
    blocks: HashMap<String, Block>,
}

#[derive(Debug, serde::Serialize)]
struct Processer {
    variable_map: HashMap<String, String>,
    item_map: HashMap<String, Item>,
    group_map: HashMap<String, Group>,
}

impl Processer {
    fn new() -> Processer {
        Processer {
            variable_map: HashMap::new(),
            item_map: HashMap::new(),
            group_map: HashMap::new(),
        }
    }

    fn add_variable(&mut self, name: String, variable: String) {
        self.variable_map.insert(name, variable);
    }

    fn process_dml(&mut self, dml: pest::iterators::Pairs<Rule>) {
        for record in dml {
            match record.as_rule() {
                Rule::variable => {
                    self.process_variable(record.into_inner());
                }
                Rule::item => {
                    let item = self.process_item(record.into_inner());
                    self.item_map.insert(item.id.clone(), item);
                }
                Rule::group => {
                    let group = self.process_group(record.into_inner());
                    self.group_map.insert(group.id.clone(), group);
                }
                _ => {}
            }
        }
    }

    fn process_variable(&mut self, variable: pest::iterators::Pairs<Rule>) {
        let mut id = String::new();
        let mut value = String::new();
        for record in variable {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    id = record.as_str().to_string();
                }
                Rule::number => {
                    println!("Number: {}", record.as_str());
                    value = record.as_str().to_string();
                }
                Rule::string => {
                    println!("String: {}", record.as_str());
                    value = self.process_string(record.into_inner());
                }
                _ => {}
            }
        }
        self.add_variable(id, value);
    }

    fn process_string(&mut self, string: pest::iterators::Pairs<Rule>) -> String {
        string.as_str().to_string()
    }

    fn process_item(&mut self, item: pest::iterators::Pairs<Rule>) -> Item {
        let mut item_strust = Item {
            id: String::new(),
            pairs: HashMap::new(),
            blocks: HashMap::new(),
        };
        for record in item {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    item_strust.id = record.as_str().to_string();
                }
                Rule::pair => {
                    let (pair_name, value) = self.process_pair(record.into_inner());
                    item_strust.pairs.insert(pair_name, value);
                }
                Rule::block => {
                    let block = self.process_block(record.into_inner());
                    item_strust.blocks.insert(block.id.clone(), block);
                }
                _ => {}
            }
        }
        item_strust
    }

    fn process_group(&mut self, group: pest::iterators::Pairs<Rule>) -> Group {
        let mut group_strust = Group {
            id: String::new(),
            pairs: HashMap::new(),
            blocks: HashMap::new(),
            reference: HashMap::new(),
        };
        for record in group {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    group_strust.id = record.as_str().to_string();
                }
                Rule::pair => {
                    let (pair_name, value) = self.process_pair(record.into_inner());
                    group_strust.pairs.insert(pair_name, value);
                }
                Rule::block => {
                    let block = self.process_block(record.into_inner());
                    group_strust.blocks.insert(block.id.clone(), block);
                }
                Rule::reference => {
                    let (reference_name, value) = self.process_reference(record.into_inner());
                    group_strust.reference.insert(reference_name, value);
                }
                _ => {}
            }
        }
        group_strust
    }

    fn process_reference(&mut self, reference: pest::iterators::Pairs<Rule>) -> (String, String) {
        let mut id = String::new();
        let mut value = String::new();
        for record in reference {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    id = record.as_str().to_string();
                }
                Rule::number => {
                    println!("Number: {}", record.as_str());
                    value = record.as_str().to_string();
                }
                Rule::string => {
                    println!("String: {}", record.as_str());
                    value = self.process_string(record.into_inner());
                }
                Rule::variable_reference => {
                    let (ref_id, _) = self.process_pair(record.into_inner());
                    if let Some(v) = self.variable_map.get(&ref_id) {
                        value = v.clone();
                    }
                }
                _ => {}
            }
        }
        (id, value)
    }

    fn process_block(&mut self, block: pest::iterators::Pairs<Rule>) -> Block {
        let mut block_data = Block {
            id: String::new(),
            pairs: HashMap::new(),
            blocks: HashMap::new(),
        };
        for record in block {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    block_data.id = record.as_str().to_string();
                }
                Rule::pair => {
                    let (pair_name, value) = self.process_pair(record.into_inner());
                    block_data.pairs.insert(pair_name, value);
                }
                Rule::block => {
                    let block = self.process_block(record.into_inner());
                    block_data.blocks.insert(block.id.clone(), block);
                }
                _ => {}
            }
        }
        block_data
    }

    fn process_pair(&mut self, pair: pest::iterators::Pairs<Rule>) -> (String, String) {
        let mut id = String::new();
        let mut value = String::new();
        for record in pair {
            match record.as_rule() {
                Rule::id => {
                    println!("ID: {}", record.as_str());
                    id = record.as_str().to_string();
                }
                Rule::number => {
                    println!("Number: {}", record.as_str());
                    value = record.as_str().to_string();
                }
                Rule::string => {
                    println!("String: {}", record.as_str());
                    value = self.process_string(record.into_inner());
                }
                Rule::variable_reference => {
                    let (ref_id, _) = self.process_pair(record.into_inner());
                    if let Some(v) = self.variable_map.get(&ref_id) {
                        value = v.clone();
                    }
                }
                _ => {}
            }
        }
        (id, value)
    }
}

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
