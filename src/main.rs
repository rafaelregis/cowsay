use std::{fs::File, io::Read};

use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    borg_mode: bool,
    #[structopt(short, long)]
    dead_mode: bool,
    #[structopt(short, long)]
    greedy_mode: bool,
    #[structopt(short, long)]
    paranoid_mode: bool,
    #[structopt(short, long)]
    stoned_mode: bool,
    #[structopt(short, long)]
    tired_mode: bool,
    #[structopt(short, long)]
    wired_mode: bool,
    #[structopt(short, long)]
    youthful_mode: bool,
    #[structopt(short, long, default_value="")]
    eye_string: String,
    #[structopt(short = "T", long, default_value="  ")]
    tongue_string: String,
    #[structopt(short, long, default_value="")]
    file_path: String,
    message: Vec<String>,
}

fn cow_eyes(args: &Cli) -> String {
    if !args.eye_string.is_empty() {
        return args.eye_string.clone()
    }
    if args.borg_mode {
        return String::from("==")
    }
    if args.dead_mode {
        return String::from("XX")
    }
    if args.greedy_mode {
        return String::from("$$")
    }
    if args.paranoid_mode {
        return String::from("@@")
    }
    if args.stoned_mode {
        return String::from("**")
    }
    if args.tired_mode {
        return String::from("--")
    }
    if args.wired_mode {
        return String::from("OO")
    }
    if args.youthful_mode {
        return String::from("..")
    }
    String::from("oo")
}

fn cow_tongue(args: &Cli) -> String {
    if args.dead_mode {
        return "U ".to_string();
    }
    let mut tongue_string= args.tongue_string.clone();
    tongue_string.truncate(2);
    return format!("{: <2}", tongue_string);
}

fn main() {
    let args = Cli::from_args();

    let message = args.message.join(" ");
    let cow_eyes = cow_eyes(&args);
    let cow_tongue = cow_tongue(&args);

    let horizontal_dialog_line = "-".repeat(message.len() + 2);

    let file_path: String;
    if args.file_path.is_empty() {
        file_path = "src/cows/cow.cow".to_string();
    } else if args.file_path.ends_with(".cow") {
        file_path = args.file_path;
    } else {
        file_path = format!("src/cows/{}.cow", args.file_path);
    }

    let mut body_template = String::new();

    let mut f = File::open(&file_path).unwrap();
    f.read_to_string(&mut body_template).expect(
        &format!("Couldn't read cowfile {}", file_path));

    let cow = body_template.replace("%eye%", &cow_eyes).replace("%tongue%", &cow_tongue);

    println!(" {} \n< {} >\n {}{}", horizontal_dialog_line, message, horizontal_dialog_line, cow);
}