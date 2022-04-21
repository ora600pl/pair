use std::fs;
use object::Object;
use object::read::ObjectSymbol;
use rayon::prelude::*;
use std::time::SystemTime;
use std::fs::File;
use std::io::Write;
use clap::Parser;


struct Symbols {
    name: String,
    b_addr: u64,
    e_addr: u64,
}

/// Annotate instruction pointers with symbols from binary file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	///Binary file to get symbols
	#[clap(short, long)]
	binary_file: String,
	
	///Output file from pinatrace to annotate 
	#[clap(short, long)]
	pinatrace_file: String,

	///Output file to be created with annotations
	#[clap(short, long)]
	output_file: String,
	
}

fn get_symbols(fname: &str) -> Vec<Symbols> {
    let ofile = fs::read(fname).unwrap();
    let parsed_obj = object::File::parse(&*ofile).unwrap();

    let mut symbols: Vec<Symbols> = Vec::new();

    for symbol in parsed_obj.symbols() {
        let symbol_end_addr = symbol.address() + symbol.size();
        let symbol_addr = symbol.address();
        let symbol_name = match symbol.name() {
                                Ok(n) => n, 
                                Err(_e) => "unknown" 
                        };
        symbols.push(Symbols{name: symbol_name.to_string(), b_addr: symbol_addr, e_addr: symbol_end_addr});
    }
    symbols
}

fn annotate(symbols: Vec<Symbols>, pin_trace: Vec<&str>, file_to_create: &str) {
    let mut out_file = File::create(file_to_create).unwrap();
    for line in pin_trace {
        if line.len() > 1 && &line[0..1] != "#" {
            let fields = line.split_whitespace().collect::<Vec<&str>>();
            let ip = &fields[0][2..18];
            let ip = u64::from_str_radix(ip, 16);
            let ip = match ip {
                    Ok(x) => x,
                    Err(_e) => 0
            };
            if ip > 0 {
                let found = symbols.par_iter().find_any(|&x| ip >= x.b_addr && ip <= x.e_addr);
                let annotation: String;
                match found {
                    Some(found) => annotation = format!("({}+{})", found.name, ip-found.b_addr),
                    None => annotation = format!("({})", "unknown"),
                };
                writeln!(&mut out_file, "{:<42}{}", annotation, line).unwrap();
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let fname = &args.binary_file;
    let file_to_annotate = &args.pinatrace_file;
    let file_to_create = &args.output_file;

    let now = SystemTime::now();

    let pin_trace = fs::read_to_string(file_to_annotate).unwrap();
    let pin_trace = pin_trace.split("\n").collect::<Vec<&str>>();
    
    let symbols = get_symbols(fname);
    annotate(symbols, pin_trace, file_to_create);
    println!("Done!");

    match now.elapsed() {
        Ok(e) => println!("Elapsed: {}s", e.as_secs()),
        Err(e) => println!("{}", e)
    }
    
}


