use std::{ time::Instant, env, io, fs };
use rand::{ thread_rng, Rng };

mod lib;

#[derive(PartialEq, Clone)]
enum Mode {
    FromFile,
    Generate,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Generate => write!(f, "Generate"),
            Mode::FromFile => write!(f, "FromFile")
        }

    }

}

struct Config {
    pub mode: Option<Mode>,
    pub count: u64,
    pub iterations: u64,
    pub radix: u64,
    pub range: (u64, u64),
    pub file: String,
    pub is_concurrent: bool,
}

fn parse_args(args: &[String]) -> Config {
    let mut config = Config { 
        mode: None, 
        count: 1_000_000, 
        iterations: 1000, 
        radix: 10,
        range: (0, 1_000_000_000),
        file: String::new(), 
        is_concurrent: false
    };
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "-c" | "--count" => {
                if let Some(Mode::FromFile) = config.mode {
                    panic!("[ERROR] Choose whether to generate random values or to read from file, but not both");

               }

                let str_count = &args[i + 1];
                let len = str_count.len();
                let prefix = str_count.as_bytes()[len - 1] as char;
            
                if !prefix.is_alphabetic() {
                    let count: u64 = str_count.parse().unwrap_or_else(|_| panic!("[ERROR] couldn't parse '{}'", str_count));
                    config.count = count;
                    i += 1;
                    continue;

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
                config.mode = Some(Mode::Generate);
                i += 1;

            }

            "-i" | "--iterations" => {
                let iterations = &args[i + 1];
                config.iterations = iterations.parse().unwrap_or_else(|_| panic!("[ERROR] couldn't parse '{}'", iterations));
                i += 1;

            }
        
            "-f" | "--file" => {
                if let Some(Mode::Generate) = config.mode {
                    panic!("[ERROR] Choose whether to generate random values or to read from file, but not both");

                }

                config.file = String::from(&args[i + 1]);            
                config.mode = Some(Mode::FromFile);
                i += 1;
            }

            "-C" | "--concurrent" => {
                config.is_concurrent = true;
            }

            "-r" | "--radix" => {
                let radix = &args[i + 1];
                config.radix = radix.parse().unwrap_or_else(
                    |_| panic!("[ERROR] couldn't parse '{}'", radix)
                );
                i += 1;
            }
            
            _ => { panic!("[ERROR] Unrecognized arguments {}", args[i]); }
        }

        
        i += 1;
    }

    config
}

/// # read_file
/// reads a file. It expect a CSV of numbers
/// it fails if the contents cannot be parsed of the size
/// of the file is larger than 1GiB
fn read_file(filename: &str) -> Result<Vec<u64>, &str> {
    let mut arr: Vec<u64> = Vec::new();
    let metadata = fs::metadata(filename).expect("shit");

    if metadata.len() > (1 << 9) {
        return Err("File too big");
    }

    if !metadata.is_file() {
        return Err("Not a file");
    }

    let contents = fs::read_to_string(filename).expect("cannot read");

    let mut temp: String = "".into();

    for letter in String::from(contents).drain(..) {
        if letter == ',' {
            arr.push(temp.parse().expect("cannot parse stuff"));
            temp = "".into();
            continue
        }

        temp += &letter.to_string()[..];
    }

    Ok(arr)
}

fn print_stats(config: &Config, 
               min: Option<f64>, 
               max: Option<f64>, 
               avg: Option<f64>, 
               sigma: Option<f64>) {
    println!("================================================================");
    println!("-------------------------Sorter---------------------------------");
    println!("Config: iterations = {}, mode = {}", config.iterations, config.mode.clone().unwrap());
    match config.mode.clone().unwrap() {
        Mode::Generate => { 
            println!("        count = {}", config.count);

            println!("Results: min = {:.5}s, max = {:.5}s, avg = {:.5}s", 
                min.unwrap(), 
                max.unwrap(), 
                avg.unwrap());
            
            println!("         \u{03c3} = {:.5}", sigma.unwrap());
        },
        
        Mode::FromFile => { 
            println!("        filename = {}", config.file);
            println!("        time = {}", avg.unwrap());
        }
    }
    println!("================================================================");

}

fn sigma(arr: &[f64]) -> f64 {
    let mut avg = 0f64;

    arr.iter().for_each(|v| avg += v);
    avg /= arr.len() as f64;

    (arr.iter()
     .map(|v| (v - avg).powi(2))
     .reduce(|a, v| a + v).unwrap() / (arr.len() as f64)).sqrt()

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = parse_args(&args);

    match &config.mode.clone().unwrap() {
        Mode::Generate => {
            let mut arr: Vec<u64> = Vec::with_capacity(config.count as usize);
            let mut durations: Vec<f64> = Vec::with_capacity(config.iterations as usize);

            for _ in 0 .. arr.capacity() {
                arr.push(thread_rng().gen_range(0..1_000_000_000));
            }

            for _ in 0 .. config.iterations {
                let rdx = (arr.iter().max().unwrap().to_owned() as f64).log2() as u64;
                let t1 = Instant::now();
                sorter::radix_sort(&arr, rdx);
                let t2 = t1.elapsed();

                durations.push(t2.as_secs_f64());
                
                arr.clear();
                for _ in 0 .. arr.capacity() {
                    arr.push(thread_rng().gen_range(0..1_000_000_000));
                }

            }
            
            let mut max = f64::MIN;
            let mut min = f64::MAX;
            let sum: f64 = durations.iter().sum();

            durations.iter().for_each(|v| {
                max = f64::max(max, *v);
                min = f64::min(min, *v);
            });

            let avg = sum / durations.len() as f64;

            print_stats(&config,
                        Some(min), 
                        Some(max), 
                        Some(avg), Some(sigma(&durations)));
        }
        Mode::FromFile => {
            let arr = read_file(&config.file[..]).expect("ERROR");

            let t1 = Instant::now();
            let sorted_arr = sorter::radix_sort(&arr, 10);
            let t2 = t1.elapsed();

            print_stats(&config, None, None, Some(t2.as_secs_f64()), None);

            fs::write(config.file + ".sorted", 
                      sorted_arr
                        .iter()
                        .map(|v| format!("{}, ", v))
                        .reduce(|a, v| format!("{}{}", a, v))
                        .unwrap());

        }
    }
    /*
    let mut arr: Vec<u64> = Vec::with_capacity(10_000_000);

    for _ in 0 .. arr.capacity() {
        arr.push(thread_rng().gen_range(0..1_000_000));
    }

    let t1 = Instant::now();
    let sorted_arr = sorter::radix_sort(&arr, 10);
    let t2 = t1.elapsed();

    println!("time = {:.5}s", t2.as_secs_f64());
    */
}
