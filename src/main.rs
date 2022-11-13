use std::{ time::Instant, env };
use rand::{ thread_rng, Rng };

mod lib;

#[derive(PartialEq, Clone)]
enum Mode {
    FromFile,
    Generate,
}

struct Config {
    pub mode: Mode,
    pub count: u64,
    pub iterations: u64,
    pub file: String,
    pub is_concurrent: bool,
}

fn parse_args(args: &[String]) -> Config {
    let mut config = Config { mode: Mode::Generate, count: 0, iterations: 1000, file: String::new(), is_concurrent: false };
    let mut i = 0;

    while i < args.len() {
        if args[i] == "-c" || args[i] == "--count" {
            if let Mode::FromFile = config.mode {
                panic!("[ERROR] Choose whether to generate random values or to read from file, but not both");

            }

            let str_count = &args[i + 1];
            let len = str_count.len();
            let prefix = str_count.as_bytes()[len - 1] as char;
            
            if !prefix.is_alphabetic() {
                let count: u64 = str_count.parse().unwrap_or_else(|_| panic!("[ERROR] couldn't parse '{}'", str_count));
                config.count = count;
                println!("{}", count);

            }

            let mut count  = String::from(str_count); count.truncate(len - 1);
            let count: u64 = count.parse().unwrap_or_else(|_| panic!("[ERROR] couldn't parse '{}'", str_count));

            let count = count * match prefix {
                'k' | 'K' => 1_000,
                'm' | 'M' => 1_000_000,
                't' | 'T' => 1_000_000_000,
                _ => 1
            };

            config.count = count;
            println!("{}", count);

        }

        else if args[i] == "-i" || args[i] == "--iterations" {
            let iterations = &args[i + 1];
            config.iterations = iterations.parse().unwrap_or_else(|_| panic!("[ERROR] couldn't parse '{}'", iterations));

        }
        
        else if args[i] == "-f" || args[i] == "--file" {
            if let Generate = config.mode {
                panic!("[ERROR] Choose whether to generate random values or to read from file, but not both");

            }

            config.file = String::from(&args[i + 1]);            
        }

        else if args[i] == "-C" || args[i] == "--concurrent" {
            config.is_concurrent = true;
        }

        i += 1;
    }

    config
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);
    /*
    let mut arr: Vec<u64> = Vec::with_capacity(10_000_000);

    for _ in 0 .. arr.capacity() {
        arr.push(thread_rng().gen_range(0..1_000_000));
    }

    let t1 = Instant::now();
    let sorted_arr = lib::radix_sort(&arr, 10);
    let t2 = t1.elapsed();

    println!("time = {:.5}s", t2.as_secs_f64());
    */
}
