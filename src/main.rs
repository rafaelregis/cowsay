fn main() {
    let message = std::env::args().nth(1).expect("No message found");
    let horizontal_dialog_line = "-".repeat(message.len() + 2);

    let cow = "
      \\   ^__^
       \\  (oo)\\_______
          (__)\\       )\\/\\
              ||----w |
              ||     ||";
    
    println!(" {} \n< {} >\n {}{}", horizontal_dialog_line, message, horizontal_dialog_line, cow);
}