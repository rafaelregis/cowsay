use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    borg_mode: bool,
    message: String,
}

fn cow_eyes(args: &Cli) -> String {
    if args.borg_mode {
        return String::from("==");
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