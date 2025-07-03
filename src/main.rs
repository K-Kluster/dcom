use std::error::Error;

pub mod deser;
pub mod errors;
pub mod operations;
pub mod result;

fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
