#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rfd::FileDialog;

#[tauri::command]
fn file_command() -> String {
    let file_path = FileDialog::new()
        .add_filter("Log File", &["log"])
        .set_directory("/")
        .pick_file();

    let file_path = match file_path {
        Some(file_path) => file_path,
        None => return "Failed to get path".to_string(),
    };

    format!("{}", file_path.display())
}

#[tauri::command]
fn dest_command() -> String {
    let folder_path = FileDialog::new().set_directory("/").pick_folder();

    let folder_path = match folder_path {
        Some(folder_path) => folder_path,
        None => return "Failed to get path".to_string(),
    };

    format!("{}", folder_path.display())
}

#[tauri::command]
fn send_command(file: &str, dest: &str, sess: &str, mode: &str, name: &str, desc: &str) -> String {
    // validate if file exists
    match std::fs::metadata(file) {
        Ok(_) => (),
        Err(_) => return "File does not exist".to_string(),
    }

    // validate if dest exists

    match std::fs::metadata(dest) {
        Ok(_) => (),
        Err(_) => return "Destination does not exist".to_string(),
    }

    // validate if sess is valid

    if sess.len() < 2 {
        return "Session ID is not valid".to_string();
    }

    // validate if mode is valid

    if mode.len() == 0 {
        return "Mode is not valid".to_string();
    }

    // validate if name is valid

    if name.len() == 0 {
        return "Name is not valid".to_string();
    }

    // validate if desc is valid

    if desc.len() == 0 {
        return "Description is not valid".to_string();
    }

    let desc = desc.replace(" ", "_");

    // copy file to dest

    match std::fs::copy(
        file,
        format!(
            "{}/{}[{}][{}]_{}",
            dest,
            sess.to_lowercase(),
            mode.to_lowercase(),
            name.to_lowercase(),
            desc.to_lowercase()
        ),
    ) {
        Ok(_) => return "Sent!".to_string(),
        Err(_) => return "Failed to copy file".to_string(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            file_command,
            dest_command,
            send_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
