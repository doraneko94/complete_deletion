use std::{env, format};
use file_destroy::destroy;
use windows::{core::*, Win32::UI::WindowsAndMessaging::*};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n = args.len();

    if n <= 1 {
        unsafe { MessageBoxW(None, 
            w!("Please drop the file you want to delete onto the icon."), 
            w!("No file has been specified."), 
            MB_OK); }
        return
    } else {
        let message = if n == 2 {
            format!("Do you really want to delete '{}'?", args[1])
        } else {
            format!("Do you really want to delete these {} files?", n - 1)
        } + " Deleted files cannot be restored.";
        unsafe {
            match MessageBoxW(None, 
                &HSTRING::from(message),
                w!("Notice!"), 
                MB_OKCANCEL) {
                MESSAGEBOX_RESULT(1) => {}
                MESSAGEBOX_RESULT(_) => { return; }
            }
        }
    } 

    for i in 1..n {
        let file_path = args[i].as_str();
        match destroy(file_path, 64, true) {
            Ok(_) => { println!("✅{} was successfully deleted.", file_path); }
            Err(e) => {
                eprintln!("❌{} was failed to delete.", file_path);
                eprintln!("Error: {}", e);
            }
        }
    }

    println!("Press any key to quit.");
    let mut word = String::new();
    std::io::stdin().read_line(&mut word).ok();
}
