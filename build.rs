use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let make = Command::new("make")
        .arg("-C")
        .arg("cpp")
        .arg("-k")
        .output()
        .expect("failled to build");

    println!("cargo:rerun-if-changed=cpp/makefile");
    println!("cargo:rerun-if-changed=cpp/kseq.cpp");
    println!("cargo:rerun-if-changed=cpp/kseqpp.cpp");
    println!("cargo:rerun-if-changed=cpp/seqan.cpp");
    println!("cargo:rerun-if-changed=cpp/bioparser.cpp");

    if !make.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&make.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&make.stderr));
    }

    let make = Command::new("make")
        .arg("-C")
        .arg("golang")
        .arg("-k")
        .output()
        .expect("failled to build");

    println!("cargo:rerun-if-changed=golang/makefile");
    println!("cargo:rerun-if-changed=golang/bio_go.go");

    if !make.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&make.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&make.stderr));
    }
}
