pub fn lg(msg: &str) {
    println!("\x1b[1;32m{}\x1b[0m", msg);
}

pub fn ly(msg: &str) {
    println!("\x1b[1;33m{}\x1b[0m", msg);
}

pub fn lr(msg: &str) {
    println!("\x1b[1;31m{}\x1b[0m", msg);
}

pub fn sanitize(text: &str) -> String {
    text.replace('[', " ")
        .replace(']', " ")
        .replace('#', " ")
        .replace('-', " ")
}
