// build.rs

use std::process::Command;

fn main() {
    let status = Command::new("clang")
        .args(["cmach/main.c", "-o", "cmach/cmach"])
        .status()
        .expect("<<< Failed to build cmach >>>");

    assert!(status.success());

    // C Mach Parser
    println!("cargo:rerun-if-changed=cmach/main.c");
}
