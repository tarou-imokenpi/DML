extern crate nom;
use nom::branch::alt;
use nom::bytes::complete::tag;
pub use nom::character::complete::space1;
use nom::IResult;
use std::error::Error;

fn parse_abc_or_def(input: &str) -> IResult<&str, &str> {
    alt((tag("abc"), tag("def")))(input)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (leftover_input, output) = parse_abc_or_def("abcWorld")?;
    assert_eq!(leftover_input, "World");
    assert_eq!(output, "abc");

    assert!(parse_abc_or_def("ghiWorld").is_err());
    Ok(())
}
