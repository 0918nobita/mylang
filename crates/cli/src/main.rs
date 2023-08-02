use std::fs;

fn main() {
    println!("Compiling source.mylang ...");

    let source = fs::read_to_string("source.mylang").unwrap();
    let source = source.trim_end();
    let source = source.replace('\n', "\\n");

    let out = format!("'use strict';\nconsole.log('{}');\n", source);

    println!("> out.js");
    fs::write("out.js", out).unwrap();
}
