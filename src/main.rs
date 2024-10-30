fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut flags: Vec<String> = Vec::new();
    let mut filename = String::new();

    let mut i = 1;
    while i < args.len() {
        if args[i].starts_with("-") {
            flags.push(args[i].clone());
        } else {
            filename = args[i].clone();
        }
        i += 1;
    }

    if filename.is_empty() || flags.is_empty() {
        panic!("Usage: {} [flags...] <filename>", args[0]);
    }

    let content = std::fs::read_to_string(&filename)
        .expect("Failed to read file")
        .trim()
        .to_string();
    let data = parse_tsv(&content);

    for flag in &flags {
        if flag.starts_with("-f") {
            let n = flag[2..].parse::<usize>().expect("Invalid flag");
            let new_data = get_nth_field(&data, n - 1);
            for row in new_data {
                println!("{}", row);
            }
            continue;
        }
        match flag.as_str() {
            "-flag1" => println!("Flag 1 is set"),
            "-flag2" => println!("Flag 2 is set"),
            _ => panic!("Unknown flag: {}", flag),
        }
    }
}

fn get_nth_field(data: &Vec<Vec<String>>, n: usize) -> Vec<String> {
    data.iter().map(|row| row[n].clone()).collect()
}

fn parse_tsv(content: &str) -> Vec<Vec<String>> {
    content
        .lines()
        .map(|line| line.split('\t').map(|s| s.to_string()).collect())
        .collect()
}
