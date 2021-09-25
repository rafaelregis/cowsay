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

fn main() {
    let args = Cli::from_args();

    let message = args.message.join(" ");

    let horizontal_dialog_line = "-".repeat(message.len() + 2);

    let cow = format!(r#"
      \   ^__^
       \  ({})\_______
          (__)\       )\/\
           {} ||----w |
              ||     ||"#, cow_eyes(&args), args.tongue_string);

    println!(" {} \n< {} >\n {}{}", horizontal_dialog_line, message, horizontal_dialog_line, cow);
}