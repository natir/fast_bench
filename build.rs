use std::process::Command;

fn main() {
    let build_args = &[
        "-Wredundant-decls", "-Wcast-align", "-Wmissing-declarations",
        "-Wmissing-include-dirs", "-Wswitch-enum", "-Wswitch-default",
        "-Wextra", "-Wall", "-Werror", "-Winvalid-pch", "-Wredundant-decls",
        "-Wformat=2", "-Wmissing-format-attribute", "-Wformat-nonliteral",
        "-O3", "-flto", "-march=native", "-mtune=native", "-I", "src/",
        "-lpthread", "-lz"
    ];
    
    let output = Command::new("g++")
        .arg("src/kseq.cpp")
        .args(build_args)
        .arg("-o")
        .arg("target/kseq")
        .output()
        .expect("failled to build");

    if !output.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        return ;
    }
    
    println!("cargo:rerun-if-changed=src/kseq.cpp");
}
