#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use lipsum::lipsum_words;

#[tauri::command]
fn lorem(words: usize) -> String {
    let p = lipsum_words(words);
    println!("Requested Lorem: {}", p);
    return p;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![lorem])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
