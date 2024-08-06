use std::fs;
use std::path::Path;
use markdown;

fn main() -> Result<(), markdown::message::Message> {
    println!(
        "{:?}",
        markdown::to_mdast("# Hey, *you*!", &markdown::ParseOptions::default())?
    );

    Ok(())
}
