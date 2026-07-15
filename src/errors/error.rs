use super::associate::CodeAssociate;

pub enum StdErrors {
    IOError(&'static str),
    SyntaxError(String, CodeAssociate),
}

pub enum CompileErrors {
    SyntaxError(CodeAssociate),
    TypeError(CodeAssociate),
    NameError(CodeAssociate),
    ValueError(CodeAssociate),
    ImportError(CodeAssociate),
    ExportError(CodeAssociate),
    FunctionError(CodeAssociate),
    UnknownIdentifier(CodeAssociate),
    InstantiationError(CodeAssociate),
    MissingParams(CodeAssociate),
    UnknownError(CodeAssociate),
}

pub fn std_error(error: StdErrors) {
    match error {
        StdErrors::IOError(message) => eprintln!("io: {}", message),
        StdErrors::SyntaxError(message, associate) => eprintln!(
            "syntax: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            message,
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
    }
}

pub fn compile_error(error: CompileErrors) {
    match error {
        CompileErrors::SyntaxError(associate) => eprintln!(
            "syntax: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Syntax Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::TypeError(associate) => eprintln!(
            "type: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Type Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::NameError(associate) => eprintln!(
            "name: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Name Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::ValueError(associate) => eprintln!(
            "value: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Value Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::ImportError(associate) => eprintln!(
            "import: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Import Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::ExportError(associate) => eprintln!(
            "export: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Export Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::FunctionError(associate) => eprintln!(
            "function: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Function Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::InstantiationError(associate) => eprintln!(
            "instantiation: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "No instantiation function found in object.",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::MissingParams(associate) => eprintln!(
            "missing: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Missing Parameters in function call.",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::UnknownIdentifier(associate) => eprintln!(
            "unknown: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Unknown identifier found in call.",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
        CompileErrors::UnknownError(associate) => eprintln!(
            "unknown: {}\n{}\n\tFile: {}\n\tAt Line: {}:{}-{}",
            "Unknown Error",
            associate.lines,
            associate.file,
            associate.line,
            associate.start_column,
            associate.end_column
        ),
    }
}
