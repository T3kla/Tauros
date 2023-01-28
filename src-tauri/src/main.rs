#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[tauri::command]
fn send_command(dest: &str, file: &str, sess: &str, mode: &str, name: &str, desc: &str) -> String {
    println!("I was invoked from JS, with this message",);
    format!("{} {} {} {} {} {}", dest, file, sess, mode, name, desc)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![send_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
