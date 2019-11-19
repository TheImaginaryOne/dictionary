mod cedict_parser;
mod types;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "dict-parser",
    about = "Parses a modified version of CEDICT and CC-Canto files."
)]
struct Opts {
    #[structopt(short = "i", long)]
    in_file: String,
    #[structopt(short = "d", long)]
    dictionary_id: i32,
}

fn main() {
    let opt = Opts::from_args();
    match std::fs::read_to_string(opt.in_file) {
        Ok(s) => {
            let result = cedict_parser::parse_cedict(&s);
            match result {
                Ok(mut i) => {
                    for entry in &mut i.1 {
                        entry.dictionary_id = opt.dictionary_id;
                    }
                    println!("{:?}", i);
                }
                Err(e) => {
                    println!("parse error {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }
}
