






fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <message>", args[0]);
        std::process::exit(1);
    }

    let msg = args.iter()
        .skip(1)
        .map(|s| s.to_owned())
        .collect::<Vec<String>>()
        .join(" ");
    let hl = &msg[1..msg.len()-1];
    println!("Primelight: {}", hl);
}
