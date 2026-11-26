use std::io::{self, BufRead, Write};
use std::process::ExitCode;

use pzprst::{Board, Options, Transform};

const USAGE: &str = "\
usage: pzprst [--strict] [--diag] [--transform NAME] <command> [input...]

Inputs are puzzle URLs, trace URLs, states or FileIO text. With no input, one
input per line is read from stdin and one result per line is written (newlines
in FileIO become `\\n`); errors are written as `ERR <kind>: <message>`.

commands:
  canon     canonical URL body
  fileio    pzpr FileIO text
  state     pzpr-trace state

options:
  --strict            reject inputs that pzprjs would silently repair
  --diag              print `DIAG <code>: <message>` lines to stderr
  --transform NAME    apply identity|rot90|rot180|rot270|flipx|flipy|transpose|antitranspose
                      before output";

struct Args {
    cmd: String,
    transform: Transform,
    diag: bool,
    opts: Options,
    inputs: Vec<String>,
}

fn parse_args() -> Result<Args, String> {
    let mut opts = Options::default();
    let (mut diag, mut transform, mut rest) = (false, Transform::Identity, Vec::new());
    let mut it = std::env::args().skip(1);
    while let Some(a) = it.next() {
        match a.as_str() {
            "--strict" => opts.strict = true,
            "--diag" => diag = true,
            "--transform" => {
                let name = it.next().ok_or("--transform needs a name")?;
                transform = name.parse().map_err(|e: pzprst::Error| e.to_string())?;
            }
            "-h" | "--help" => return Err(String::new()),
            _ => rest.push(a),
        }
    }
    if rest.is_empty() {
        return Err(String::new());
    }
    let cmd = rest.remove(0);
    if !matches!(cmd.as_str(), "canon" | "fileio" | "state") {
        return Err(format!("unknown command {cmd:?}"));
    }
    Ok(Args {
        cmd,
        transform,
        diag,
        opts,
        inputs: rest,
    })
}

fn run(args: &Args, input: &str) -> pzprst::Result<String> {
    let board = Board::parse(input, &args.opts)?;
    if args.diag {
        for d in board.diagnostics() {
            eprintln!("DIAG {d}");
        }
    }
    let board = board.try_transform(args.transform)?;
    match args.cmd.as_str() {
        "canon" => board.to_url(),
        "fileio" => board.to_fileio(),
        _ => board.to_state(),
    }
}

fn main() -> ExitCode {
    let args = match parse_args() {
        Ok(a) => a,
        Err(msg) => {
            if !msg.is_empty() {
                eprintln!("{msg}");
            }
            eprintln!("{USAGE}");
            return ExitCode::from(2);
        }
    };

    if !args.inputs.is_empty() {
        let mut failed = false;
        for input in &args.inputs {
            match run(&args, input) {
                Ok(out) => println!("{out}"),
                Err(e) => {
                    eprintln!("ERR {}: {e}", e.kind());
                    failed = true;
                }
            }
        }
        return if failed {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        };
    }

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    for line in io::stdin().lock().lines() {
        let line = line.expect("read stdin");
        let result = match run(&args, line.trim()) {
            Ok(s) => s.replace('\n', "\\n"),
            Err(e) => format!("ERR {}: {e}", e.kind()),
        };
        writeln!(out, "{result}").expect("write stdout");
    }
    ExitCode::SUCCESS
}
