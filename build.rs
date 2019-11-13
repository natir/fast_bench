use std::process::Command;

fn main() {

    let output = Command::new("make")
        .arg("-C")
        .arg("cpp")
        .output()
        .expect("failled to build");
    
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=cpp/makefile");
    println!("cargo:rerun-if-changed=cpp/kseq.cpp");
    println!("cargo:rerun-if-changed=cpp/seqan.cpp");
    println!("cargo:rerun-if-changed=cpp/bioparser.cpp");
    
    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        panic!("Error durring cpp build");
    }
}
