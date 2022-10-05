use clap::Parser;
// use std::io;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    cidr: u32,
}

fn main() {
    let args = Args::parse();
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("failed to read from stdin");

    // let cidr = input.trim();
    // match cidr.parse::<u32>() {
    //     Ok(_i) => println!(""),
    //     Err(..) => println!("this was not an integer"),
    // };

    // let mut cidr = cidr.parse::<u32>().unwrap();
    let mut cidr = args.cidr;
    let mut count = 0;
    while cidr >= 8 {
        cidr = cidr - 8;
        count = count + 1;
    }
    let _base: u32 = 10;
    // let p = u32::pow(2, cidr);
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
