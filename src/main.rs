#[macro_use]
extern crate pest_derive;

use std::env;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;

use anyhow::Result;
use indexmap::IndexMap;
use lazy_static::lazy_static;
use markdown::{Block, Span};
use mdcat::{Environment, ResourceAccess, Settings, TerminalCapabilities, TerminalSize};
use pulldown_cmark::{Options, Parser as MdParser};
use slugify::slugify;
use structopt::StructOpt;
use syntect::parsing::SyntaxSet;

mod ast;
mod eval;

const INTRO_SECTION: &str = "intro";
const RUNDOWN_CODE_BLOCK_SYNTAX: &str = "rundown";

lazy_static! {
    static ref MDCAT_SETTINGS: Settings = Settings {
        terminal_capabilities: TerminalCapabilities::detect(),
        terminal_size: TerminalSize::from_terminal().unwrap(),
        resource_access: ResourceAccess::LocalOnly,
        syntax_set: SyntaxSet::load_defaults_newlines(),
    };
    static ref MDCAT_ENV: Environment =
        Environment::for_local_directory(&env::current_dir().unwrap()).unwrap();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "rundown", about = "Run your markdown adventure!")]
struct Opt {
    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

fn spans_to_string(spans: &[Span]) -> String {
    spans
        .iter()
        .map(|span| match span {
            Span::Text(ref s) | Span::Code(ref s) | Span::Image(ref s, _, _) => s.to_owned(),
            Span::Literal(c) => c.to_string(),
            Span::Link(s, _, _) | Span::RefLink(s, _, _) | Span::Emphasis(s) | Span::Strong(s) => {
                spans_to_string(s)
            }
            _ => "".to_owned(),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn construct_index(blocks: &[Block]) -> IndexMap<String, Vec<Block>> {
    let mut ret: IndexMap<String, Vec<Block>> = IndexMap::new();

    let mut current_section = ret.entry(INTRO_SECTION.to_owned()).or_default();
    for block in blocks {
        match block {
            Block::Header(spans, _) => {
                let name = slugify!(&spans_to_string(&spans).to_lowercase());
                current_section = ret.entry(name).or_default();
            }
            _ => {
                current_section.push(block.clone());
            }
        }
    }

    ret
}

fn print_markdown(content: &str) -> Result<()> {
    // TODO: Calculate this once?
    let parser = MdParser::new_ext(
        content,
        Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH,
    );

    let stdout = stdout();
    let mut handle = stdout.lock();
    mdcat::push_tty(&MDCAT_SETTINGS, &MDCAT_ENV, &mut handle, parser)?;

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)?;
    let section_index = construct_index(&markdown::tokenize(&input));

    let mut context: eval::Context = Default::default();

    let mut pc = 0;
    while let Some((name, section)) = section_index.get_index(pc) {
        println!("{}", name);
        for block in section {
            match block {
                Block::CodeBlock(Some(syntax), content) if syntax == RUNDOWN_CODE_BLOCK_SYNTAX => {
                    let statements = ast::parse(content)?;
                    let res = context.eval(&statements)?;
                    println!("{:?}", &res);
                }
                _ => {
                    let content = markdown::generate_markdown(vec![block.clone()]);
                    print_markdown(&content)?;
                }
            }
        }

        pc += 1;
    }

    Ok(())
}
