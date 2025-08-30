use std::fs;
use std::io;
use std::path::Path;
use std::process::{Command, Stdio};

use rand::{distributions::Alphanumeric, Rng};

use crate::ports::get_free_port;

/// Creates a new file with a random name and copies the contents of `path` into it.
pub fn create_new_file_copy(path: &str) -> io::Result<String> {
    let hash: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();

    println!("[INFO] Generated hash for new file: {}", hash);

    let original = Path::new(path);

    let stem = original.file_stem().unwrap_or_default().to_string_lossy();
    let ext = original
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();

    println!(
        "[INFO] Original file stem: '{}', extension: '{}'",
        stem, ext
    );

    let new_path = original.with_file_name(format!(
        "SUBPROCESS_{}_{}{}",
        stem, hash, ext
    ));

    println!(
        "[INFO] Copying from '{}' to '{}'",
        original.display(),
        new_path.display()
    );

    // Copy the file
    fs::copy(original, &new_path)?;
    println!("[SUCCESS] File copy complete!");

    let server_path = run_new_sub(new_path.to_string_lossy().to_string());
    
    return Ok(server_path);
}

fn run_new_sub(new_path: String) -> String{
    let path = Path::new(&new_path);

    // Extract file name
    let filename = path
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .expect("No file name in path");

    // Extract parent dir
    let cd_path = path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .expect("No parent directory in path");

    println!("[INFO] Preparing to launch subprocess:");
    println!("        File name   : {}", filename);
    println!("        Working dir : {}", cd_path);

    let port: u32 = get_free_port();
    println!("[INFO] Using free port: {}", port);

    // ✅ Always use the full absolute path
    let full_path = path.to_string_lossy().to_string();

    println!("[INFO] Launching binary (detached):");
    println!("        Executable : {}", full_path);
    println!("        Args       : [{}]", port);

    // Spawn without waiting (detached mode)
    let child = if cfg!(target_os = "windows") {
        Command::new(&full_path)
            .current_dir(&cd_path)
            .arg(port.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit()) // stream logs to parent console
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn process")
    } else {
        Command::new(&full_path)
            .current_dir(&cd_path)
            .arg(port.to_string())
            .stdin(Stdio::null())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()
            .expect("failed to spawn process")
    };

    println!(
        "[SUCCESS] Subprocess started with PID: {} (running in background)",
        child.id()
    );

    let server_path = format!("http://localhost:{}", port);

    return server_path;
    
}
