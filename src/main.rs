use std::io;
fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");

    let cidr = input.trim();
    match cidr.parse::<usize>() {
        Ok(_i) => println!(""),
        Err(..) => println!("this was not an integer"),
    };

    let mut cidr = cidr.parse::<usize>().unwrap();
    let mut count = 0;
    while cidr >= 8 {
        cidr = cidr - 8;
        count = count + 1;
    }
    let base:u32 =2;
    cidr = base.pow(cidr);
    printer(cidr, count);
}

fn printer(c: usize, cnt: i32) {
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
