#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use rfd::FileDialog;

#[tauri::command]
fn file_command() -> Option<String> {
    let file_path = FileDialog::new()
        .add_filter("Log File", &["log"])
        .pick_file();

    match file_path {
        Some(file_path) => Some(format!("{}", file_path.display())),
        None => None,
    }
}

#[tauri::command]
fn dest_command() -> Option<String> {
    let folder_path = FileDialog::new().pick_folder();

    match folder_path {
        Some(folder_path) => Some(format!("{}", folder_path.display())),
        None => None,
    }
}

#[tauri::command]
fn send_command(file: &str, dest: &str, sess: &str, mode: &str, name: &str, desc: &str) -> String {
    match std::fs::metadata(file) {
        Ok(_) => (),
        Err(_) => return "File does not exist".to_string(),
    }

    match std::fs::metadata(dest) {
        Ok(_) => (),
        Err(_) => return "Destination does not exist".to_string(),
    }

    if sess.len() < 2 {
        return "Session is not valid".to_string();
    }
    if mode.len() == 0 {
        return "Mode is not valid".to_string();
    }
    if name.len() == 0 {
        return "Name is not valid".to_string();
    }
    if desc.len() == 0 {
        return "Description is not valid".to_string();
    }

    let sess = sess.replace(" ", "_").to_lowercase();
    let mode = mode.replace(" ", "_").to_lowercase();
    let name = name.replace(" ", "_").to_lowercase();
    let desc = desc.replace(" ", "_").to_lowercase();

    // copy file to dest

    match std::fs::copy(
        file,
        format!("{}/{}[{}][{}]_{}", dest, sess, mode, name, desc),
    ) {
        Ok(_) => return "Sent!".to_string(),
        Err(e) => return e.to_string(),
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
