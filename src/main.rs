use std::thread;
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW};
use enigo::{Enigo, KeyboardControllable};

fn get_active_window_title() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        let len = GetWindowTextLengthW(hwnd);
        if len > 0 {
            let mut buffer = vec![0u16; len as usize + 1];
            GetWindowTextW(hwnd, &mut buffer);
            String::from_utf16_lossy(&buffer[..len as usize])
        } else {
            String::new()
        }
    }
}

fn main() {
    println!("Press F3 to toggle automatic E key pressing. Press F1 to exit.");
    
    let device_state = DeviceState::new();
    let mut enigo = Enigo::new();
    let running = true;
    let mut active = false;
    
    while running {
        let keys: Vec<Keycode> = device_state.get_keys();
        
        if keys.contains(&Keycode::F1) {
            println!("Script terminated.");
            break;
        }
        
        if keys.contains(&Keycode::F3) {
            active = !active;
            println!("Automatic E key pressing is {}.", if active { "enabled" } else { "disabled" });
            // Wait until F3 is released
            while device_state.get_keys().contains(&Keycode::F3) {
                thread::sleep(Duration::from_millis(100));
            }
        }
        
        if active {
            let window_title = get_active_window_title();
            if window_title.contains("Minecraft") || window_title.contains("Roblox") {
                // Simulate pressing 'e'
                enigo.key_click(enigo::Key::Layout('e'));
                thread::sleep(Duration::from_millis(10));
            } else {
                thread::sleep(Duration::from_millis(5));
            }
        } else {
            thread::sleep(Duration::from_millis(5));
        }
    }
}
