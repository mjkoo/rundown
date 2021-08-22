use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use std::thread;
use std::time;

use anyhow::{anyhow, bail, Result};

use crate::eval::{Builtin, Value};

pub fn builtins() -> HashMap<String, Builtin> {
    let mut h: HashMap<String, Builtin> = HashMap::new();
    h.insert("read".to_owned(), read);
    h.insert("print".to_owned(), print);
    h.insert("sleep".to_owned(), sleep);
    h
}

fn print(arguments: &[Value]) -> Result<Value> {
    println!(
        "{}",
        arguments
            .iter()
            .map(|arg| arg.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
    Ok(Value::Bool(true))
}

fn read(_arguments: &[Value]) -> Result<Value> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut buffer)?;

    // Do some dynamic typing magic
    if buffer.to_lowercase() == "true" {
        Ok(Value::Bool(true))
    } else if buffer.to_lowercase() == "false" {
        Ok(Value::Bool(false))
    } else if let Ok(i) = buffer.parse::<i64>() {
        Ok(Value::Int(i))
    } else {
        Ok(Value::Str(buffer))
    }
}

fn sleep(arguments: &[Value]) -> Result<Value> {
    if arguments.len() != 1 {
        bail!("Invalid arguments");
    }

    if let Value::Int(i) = arguments[0] {
        if i < 0 {
            bail!("Negative duration")
        }

        thread::sleep(time::Duration::from_secs(i as u64));
        Ok(Value::Bool(true))
    } else {
        Err(anyhow!("Type error"))
    }
}