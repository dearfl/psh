use std::process::Command;

fn main() {
    // Update the psh-proto submodule
    let _ = Command::new("git")
        .args(["submodule", "update", "--init", "--recursive"])
        .status();

    tonic_build::compile_protos("src/services/proto/psh.proto").unwrap();
}
