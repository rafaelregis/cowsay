use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    message: String,
}

fn main() {
    let args = Cli::from_args();
    let horizontal_dialog_line = "-".repeat(args.message.len() + 2);

    let cow = "
      \\   ^__^
       \\  (oo)\\_______
          (__)\\       )\\/\\
              ||----w |
              ||     ||";
    
    println!(" {} \n< {} >\n {}{}", horizontal_dialog_line, args.message, horizontal_dialog_line, cow);
}