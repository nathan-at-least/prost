use std::ffi::{OsStr, OsString};

/// Configuration for protobuf compilation
#[derive(Debug)]
pub enum CompileMode {
    // Do not use a protobuf compiler
    None,
    // Use external `protoc` binary with the given `args`
    Protoc {
        args: Vec<OsString>,
    },
    // Use the [protox] crate to compile
    #[cfg(protox)]
    Protox,
}

impl Default for CompileMode {
    fn default() -> Self {
        CompileMode::Protoc { args: vec![] }
    }
}

impl CompileMode {
    /// Append a `protoc` arg
    ///
    /// This ensures the mode is set to `protoc` overwriting any other mode.
    pub fn append_protoc_arg<S>(&mut self, arg: S)
    where
        S: AsRef<OsStr>,
    {
        use CompileMode::*;

        let mut new_args = vec![];

        let args: &mut Vec<OsString> = match self {
            None => &mut new_args,
            Protoc { args } => args,
            #[cfg(protox)]
            Protox => &mut new_args,
        };

        args.push(arg.as_ref().to_owned());
        let args: Vec<OsString> = std::mem::take(args);

        *self = Protoc { args };
    }
}
