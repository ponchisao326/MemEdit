use windows::{
    core::PSTR,
    Win32::{
        Foundation::{BOOL, HWND, LPARAM},
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextA, GetWindowThreadProcessId, IsWindowVisible,
        },
    },
};

fn main() {
    // Enumeramos todas las ventanas visibles
    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(0));
    }
}

/// Callback para EnumWindows
unsafe extern "system" fn enum_windows_proc(hWnd: HWND, _lParam: LPARAM) -> BOOL {
    // Comprobamos si la ventana es visible
    if IsWindowVisible(hWnd).as_bool() {
        // Reservamos un búfer para el texto de la ventana
        let mut buffer = [0u8; 256];
        let length = GetWindowTextA(hWnd, &mut buffer);

        if length > 0 {
            // Convertimos el texto de la ventana a una cadena de Rust
            let window_text = String::from_utf8_lossy(&buffer[..length as usize]);

            // Obtenemos el ID del proceso asociado a la ventana
            let mut process_id = 0u32;
            GetWindowThreadProcessId(hWnd, Some(&mut process_id));

            // Imprimimos la información de la ventana
            println!("Window: '{}', Process ID: {}", window_text, process_id);
        }
    }
    // Continuamos enumerando
    true.into()
}
