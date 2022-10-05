use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cidr: u32,
}

fn main() {
    let args = Args::parse();

    let mut cidr = args.cidr;
    let mut count = 0;
    while cidr >= 8 {
        cidr = cidr - 8;
        count = count + 1;
    }

    printer(cidr, count);
}

fn printer(c: u32, cnt: i32) {
    let (one, two, three, four);
    match cnt {
        4 => (one, two, three, four) = (8, 8, 8, 8),
        3 => (one, two, three, four) = (8, 8, 8, c),
        2 => (one, two, three, four) = (8, 8, c, 0),
        1 => (one, two, three, four) = (8, c, 0, 0),
        _ => (one, two, three, four) = (c, 0, 0, 0),
    };

    println!("{}.{}.{}.{}", one, two, three, four)
}
