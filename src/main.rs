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
    message: String,
}

fn cow_eyes(args: &Cli) -> String {
    if args.borg_mode {
        return String::from("==");
    }
    if args.dead_mode {
        return String::from("XX");
    }
    if args.greedy_mode {
        return String::from("$$");
    }
    if args.paranoid_mode {
        return String::from("@@");
    }
    if args.stoned_mode {
        return String::from("**");
    }
    if args.tired_mode {
        return String::from("--");
    }
    if args.wired_mode {
        return String::from("OO");
    }
    if args.youthful_mode {
        return String::from("..");
    }
    return String::from("oo");
}

fn main() {
    let args = Cli::from_args();
    let horizontal_dialog_line = "-".repeat(args.message.len() + 2);

    let cow = format!("
      \\   ^__^
       \\  ({})\\_______
          (__)\\       )\\/\\
              ||----w |
              ||     ||", cow_eyes(&args));
    
    println!(" {} \n< {} >\n {}{}", horizontal_dialog_line, args.message, horizontal_dialog_line, cow);
}