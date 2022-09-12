use std::error::Error;
use std::io::{stdout, ErrorKind, Write, BufWriter};
use std::process::exit;
use std::str::FromStr;
use std::{env, io};
use tinyrand::{Counter, Rand, Seeded, SplitMix, Wyrand, Xorshift};
use tinyrand_std::ClockSeed;

fn main() {
    match generate() {
        Ok(samples) => {
            eprintln!("{samples} samples emitted");
        }
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

enum Generator {
    Counter,
    SplitMix,
    Wyrand,
    Xorshift,
}

impl FromStr for Generator {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "counter" => Ok(Self::Counter),
            "splitmix" => Ok(Self::SplitMix),
            "wyrand" => Ok(Self::Wyrand),
            "xorshift" => Ok(Self::Xorshift),
            _ => Err(format!("unknown generator '{}'", s)),
        }
    }
}

enum OutputFormat {
    Text,
    Binary,
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text" => Ok(Self::Text),
            "binary" => Ok(Self::Binary),
            _ => Err(format!("unknown output format '{}'", s)),
        }
    }
}

fn generate() -> Result<u64, Box<dyn Error>> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 5 {
        Err("usage: {} <generator ∈ {xorshift, wyrand, counter}> <seed ∈ {clock} ∪ ℕ> <format ∈ {text, binary}> <count ∈ ℕ⁺>")?;
    }

    let generator = Generator::from_str(&args[1])?;
    let seed = if args[2].eq_ignore_ascii_case("clock") {
        ClockSeed::default().next_u64()
    } else {
        u64::from_str(&args[2])?
    };
    let format = OutputFormat::from_str(&args[3])?;
    let count = &args[4];
    let count = count.replace('K', "000");             // kilo-
    let count = count.replace('M', "000000");          // million/mega-
    let count = count.replace('B', "000000000");       // billion/giga-
    let count = count.replace('G', "000000000");
    let count = count.replace('T', "000000000000");    // trillion/tera-
    let count = count.replace('Q', "000000000000000"); // quadrillion/peta-
    let count = count.replace('P', "000000000000000");
    let count = u64::from_str(&count)?;

    let rand: Box<dyn Rand> = match generator {
        Generator::Counter => Box::new(Counter::seed(seed)),
        Generator::SplitMix => Box::new(SplitMix::seed(seed)),
        Generator::Wyrand => Box::new(Wyrand::seed(seed)),
        Generator::Xorshift => Box::new(Xorshift::seed(seed)),
    };

    match format {
        OutputFormat::Text => generate_text(&args[1], count, rand),
        OutputFormat::Binary => generate_bin(count, rand),
    }
}

fn generate_text(
    rand_name: &str,
    count: u64,
    mut rand: Box<dyn Rand>,
) -> Result<u64, Box<dyn Error>> {
    println!("#==================================================================");
    println!("# generator {}", rand_name);
    println!("#==================================================================");
    println!("type: d");
    println!("count: {count}");
    println!("numbit: 64");
    let mut out = stdout();
    let newline = "\n".as_bytes();
    for iter in 1..=count {
        let random = rand.next_u64();
        let s = random.to_string();
        if out.write(s.as_bytes()).suppress()?.is_broken_pipe()
            || out.write(newline).suppress()?.is_broken_pipe()
        {
            return Ok(iter);
        }
    }
    Ok(count)
}

fn generate_bin(count: u64, mut rand: Box<dyn Rand>) -> Result<u64, Box<dyn Error>> {
    let mut out = BufWriter::new(stdout());
    let mut buf = [0u8; 8];
    for iter in 1..=count {
        let rand = rand.next_u64();
        buf[0] = rand as u8;
        buf[1] = (rand >> 8) as u8;
        buf[2] = (rand >> 16) as u8;
        buf[3] = (rand >> 24) as u8;
        buf[4] = (rand >> 32) as u8;
        buf[5] = (rand >> 40) as u8;
        buf[6] = (rand >> 48) as u8;
        buf[7] = (rand >> 56) as u8;
        if out.write(&buf).suppress()?.is_broken_pipe() {
            return Ok(iter);
        }
    }
    Ok(count)
}

enum WriteOutcome {
    Written(usize),
    BrokenPipe,
}

impl WriteOutcome {
    fn is_broken_pipe(&self) -> bool {
        matches!(&self, Self::BrokenPipe)
    }
}

trait SuppressBrokenPipe {
    fn suppress(self) -> io::Result<WriteOutcome>;
}

impl SuppressBrokenPipe for io::Result<usize> {
    fn suppress(self) -> io::Result<WriteOutcome> {
        match self {
            Ok(bytes) => Ok(WriteOutcome::Written(bytes)),
            Err(err) => match err.kind() {
                ErrorKind::BrokenPipe => Ok(WriteOutcome::BrokenPipe),
                _ => Err(err),
            },
        }
    }
}
