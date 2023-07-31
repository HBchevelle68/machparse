// build.rs

use std::process::Command;

fn main() {
    let status = Command::new("clang")
        .args([
            "-std=c11",
            "-Wall",
            "cmach/main.c",
            "cmach/parse_header.c",
            "-o",
            "cmach/cmach",
        ])
        .status()
        .expect("<<< Failed to build cmach >>>");

    assert!(status.success());

    // C Mach Parser
    println!("cargo:rerun-if-changed=cmach/main.c");
}
