pub enum StdErrors {
    IOError(&'static str),
    SyntaxError(String, String, usize, usize)
}
pub fn std_error(error: StdErrors) {
    match error {
        StdErrors::IOError(message) => eprintln!("io: {}", message),
        StdErrors::SyntaxError(message, line, line_num, column) => eprintln!("syntax: {}\n{}\n\tAt Line: {} Column: {}", message, line, line_num, column),
    }
}
