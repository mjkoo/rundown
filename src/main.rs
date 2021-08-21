use std::env;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use indexmap::IndexMap;
use markdown::{Block, Span};
use mdcat::{Environment, Settings, ResourceAccess, TerminalCapabilities, TerminalSize};
use pulldown_cmark::{Options, Parser};
use slugify::slugify;
use structopt::StructOpt;
use syntect::parsing::SyntaxSet;

const INTRO_SECTION: &'static str = "intro";

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
    let terminal_capabilities = TerminalCapabilities::detect();
    let terminal_size = TerminalSize::from_terminal().ok_or_else(|| anyhow!("Unable to detect terminal parameters"))?;
    let settings = Settings{
        terminal_capabilities,
        terminal_size,
        resource_access: ResourceAccess::LocalOnly,
        syntax_set: SyntaxSet::load_defaults_newlines(),
    };

    let parser = Parser::new_ext(
        content,
        Options::ENABLE_TASKLISTS | Options::ENABLE_STRIKETHROUGH,
    );
    let env = Environment::for_local_directory(&env::current_dir()?)?;

    let stdout = stdout();
    let mut handle = stdout.lock();
    mdcat::push_tty(&settings, &env, &mut handle, parser)?;

    Ok(())
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    let input = fs::read_to_string(&opt.input)?;
    let section_index = construct_index(&markdown::tokenize(&input));

    let mut pc = 0;
    loop {
        if let Some((name, section)) = section_index.get_index(pc) {
            println!("{}", name);
            for block in section {
                if let Block::CodeBlock(_, content) = block {
                    println!("GOT CODE: {}", content);
                } else {
                    let content = markdown::generate_markdown(vec![block.clone()]);
                    print_markdown(&content)?;
                }
            }

            pc += 1;
        } else {
            break;
        }
    }

    Ok(())
}
