use pest::Parser;
use pest_derive::Parser;
use std::fs;

#[derive(Parser)]
#[grammar = "dml.pest"]
pub struct DMLParser;

fn main() {
    let input = fs::read_to_string("./test.dml").expect("Failed to read file");
    let dml = DMLParser::parse(Rule::dml, &input)
        .expect("Failed to parse")
        .next()
        .expect("No dml found");

    process_dml(dml);
}

fn process_dml(dml: pest::iterators::Pair<Rule>) {
    for record in dml.into_inner() {
        for object in record.into_inner() {
            match object.as_rule() {
                Rule::variable => process_variable(object),
                Rule::item => process_item(object),
                Rule::group => process_group(object),
                _ => println!("Unknown rule: {:?}", object.as_rule()),
            }
        }
    }
}

fn process_variable(object: pest::iterators::Pair<Rule>) {
    println!("Matched variable");
    let mut inner_rules = object.into_inner();
    let id = inner_rules.next().expect("No ID found").as_str();
    let value = inner_rules.next().expect("No value found").as_str();
    println!("Variable: {} = {}", id, value);
}

fn process_item(object: pest::iterators::Pair<Rule>) {
    println!("Matched item");
    let mut inner_rules = object.into_inner();
    let item_name = inner_rules.next().expect("No item name found").as_str();
    println!("Item: {}", item_name);

    for block in inner_rules {
        parse_block(block);
    }
}

fn process_group(object: pest::iterators::Pair<Rule>) {
    println!("Matched group");
    let mut inner_rules = object.into_inner();
    let group_name = inner_rules.next().expect("No group name found").as_str();
    println!("Group: {}", group_name);

    for ref_block in inner_rules {
        parse_ref_block(ref_block);
    }
}

fn parse_block(block: pest::iterators::Pair<Rule>) {
    let block_name = block.as_str();
    println!("  Block:{}", block_name);

    for nested_block in block.into_inner() {
        match nested_block.as_rule() {
            Rule::field => {
                let mut inner_fields = nested_block.into_inner();
                let field_name = inner_fields.next().unwrap().as_str();
                let field_value = inner_fields.next().unwrap().as_str();
                println!("    Field: {} = {}", field_name, field_value);
            }
            Rule::block => parse_block(nested_block),
            _ => {}
        }
    }
}

fn parse_ref_block(ref_block: pest::iterators::Pair<Rule>) {
    let mut inner_rules = ref_block.into_inner();

    while let Some(ref_item) = inner_rules.next() {
        match ref_item.as_rule() {
            Rule::reference => {
                let mut ref_inner = ref_item.into_inner();
                let item_name = ref_inner.next().unwrap().as_str();
                let value = ref_inner.next().unwrap().as_str();
                println!("  Ref: {} = {}", item_name, value);
            }
            Rule::nested_block => {
                for nested in ref_item.into_inner() {
                    match nested.as_rule() {
                        Rule::reference => {
                            let mut ref_inner = nested.into_inner();
                            let item_name = ref_inner.next().unwrap().as_str();
                            let value = ref_inner.next().unwrap().as_str();
                            println!("  Ref: {} = {}", item_name, value);
                        }
                        Rule::block => parse_block(nested),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
