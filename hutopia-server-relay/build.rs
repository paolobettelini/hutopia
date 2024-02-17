use std::process::Command;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../hutopia-frontend/**");

    if !check_program_installed("bun") {
        panic!("bun is not installed! install it first.");
    }

    eprintln!("Building frontend");
    build_frontend()?;
    
    Ok(())
}

fn build_frontend() -> std::io::Result<()> {
    let node_modules = std::path::Path::new("../hutopia-frontend/node_modules");
    if !node_modules.exists() {
        let _exit_status = Command::new("bun")
            .current_dir("../hutopia-frontend")
            .arg("install")
            .status()?;
    }
    
    let _exit_status = Command::new("bun")
        .current_dir("../hutopia-frontend")
        .arg("run")
        .arg("build")
        .status()?;
    Ok(())
}

#[cfg(windows)]
fn check_program_installed(program: &str) -> bool {
    let output = Command::new("where")
        .arg(program)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

#[cfg(unix)]
fn check_program_installed(program: &str) -> bool {
    let output = Command::new("which")
        .arg(program)
        .output()
        .expect("failed to execute process");
    output.status.success()
}
