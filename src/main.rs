use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::sequence::{delimited, separated_pair};
use nom::IResult;
#[derive(Debug, PartialEq)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

fn parse_integer_pair(input: &str) -> IResult<&str, (i32, i32)> {
    separated_pair(i32, tag(", "), i32)(input)
}

fn parse_coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (remaining, (x, y)) = delimited(tag("("), parse_integer_pair, tag(")"))(input)?;

    Ok((remaining, Coordinate { x, y }))
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_, parsed) = parse_coordinate("(3,5)")?;

    assert_eq!(parsed, Coordinate { x: 3, y: 5 });

    println!("{:?}", parsed);

    Ok(())
}
