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
    let mut delimeter = "\t";
    for flag in &flags {
        if flag.starts_with("-d") {
            delimeter = &flag[2..].trim_matches('"');
        }
    }

    let mut data = parse_tsv(&content, &delimeter);
    for flag in &flags {
        if flag.starts_with("-f") {
            let n = flag[2..].parse::<usize>().expect("Invalid flag");
            data = get_nth_field(&data, n - 1);
        }
    }

    for row in data {
        println!("{}", row.join(delimeter));
    }
}

fn get_nth_field(data: &Vec<Vec<String>>, n: usize) -> Vec<Vec<String>> {
    data.iter().map(|row| vec![row[n].clone()]).collect()
}

fn parse_tsv(content: &str, delimeter: &str) -> Vec<Vec<String>> {
    content
        .lines()
        .map(|line| line.split(delimeter).map(|s| s.to_string()).collect())
        .collect()
}
