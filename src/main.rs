use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use markdown::Block;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rundown", about = "Run your markdown adventure!")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)?;
    let blocks = markdown::tokenize(&input);
    let splits: Vec<Vec<Block>> = blocks
        .split_inclusive(|token| match token {
            Block::CodeBlock(_, _) => true,
            _ => false,
        })
        .map(|s| s.to_vec())
        .collect();

    for split in splits.into_iter() {
        if let Some((Block::CodeBlock(Some(name), content), rest)) = split.split_last() {
            println!("{}", termimad::term_text(&markdown::generate_markdown(rest.to_vec())));
            println!("GOT CODE: {} -> {}", name, content);
        } else {
            println!("{}", termimad::term_text(&markdown::generate_markdown(split)));
        }
    }

    Ok(())
}
