use std::env::args;
use std::ffi::{CStr, CString};
use std::process::exit;

use nix::unistd::execv;
use serde_json::from_str;


const USAGE: &str = r#"Invocation error. Usage example (from a shell):
arghsh -c '["/bin/ls", "-la"]'
But the intended use is as a login shell for SSH to launch programs with.
See README.md for details.
"#;

fn cstringify(argv: &Vec<String>) -> Vec<Box<CString>> {
    let mut cstrings: Vec<Box<CString>> = vec![];
    for arg in argv {
        let arg_bytes: &[u8] = &arg.as_bytes();
        let cstring = CString::new(arg_bytes).expect("Not a nice C string");
        cstrings.push(Box::new(cstring));
    }
    return cstrings;
}

fn execv_stringvec(argv: &Vec<String>) {
    let mut argv_c_boxed = cstringify(argv);
    let mut argv_c_ptrs: Vec<&CStr> = vec![];
    for arg in &mut argv_c_boxed {
        argv_c_ptrs.push(&**arg);
    }
    let _ = execv(argv_c_ptrs[0], &argv_c_ptrs);
}

fn main() {
    let args: Vec<String> = args().collect();
    if args.len() < 3 || args[1] != "-c" {
        eprintln!("{}", USAGE);
        exit(101);
    }
    let argv: Vec<String> = from_str(&args[2]).expect("That's not a JSON-array of strings");
    execv_stringvec(&argv); // on the happy path execution of this program will stop here
    panic!(format!("Exec failed: {}", argv[0]));
}
