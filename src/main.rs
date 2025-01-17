use std::{sync::Mutex};
use windows::{
    Win32::{
        Foundation::{BOOL, HWND, LPARAM},
        System::{
            Memory::{VirtualQueryEx, MEMORY_BASIC_INFORMATION},
            Threading::{OpenProcess, PROCESS_ALL_ACCESS, PROCESS_VM_READ},
        },
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowTextA, GetWindowThreadProcessId, IsWindowVisible,
        },
    },
};

// Lista global para almacenar direcciones de las ventanas encontradas
lazy_static::lazy_static! {
    static ref FOUND_WINDOWS: Mutex<Vec<(String, u32)>> = Mutex::new(Vec::new());
}

fn main() {
    // Enumeramos todas las ventanas visibles
    unsafe {
        EnumWindows(Some(enum_windows_proc), LPARAM(0))
            .map_err(|e| e.to_string())
            .expect("Failed to enumerate windows");
    }

    let windows_list = FOUND_WINDOWS.lock().unwrap();
    if windows_list.is_empty() {
        println!("No visible windows found.");
        return;
    }

    // Mostrar las ventanas detectadas
    println!("Visible Windows:");
    for (index, (title, process_id)) in windows_list.iter().enumerate() {
        println!("{}. '{}' - Process ID: {}", index, title, process_id);
    }

    // Pedir al usuario que seleccione una ventana
    println!("\nEnter the number of the window you want to scan:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse::<usize>() {
        Ok(index) if index < windows_list.len() => {
            let (_, process_id) = &windows_list[index];
            println!("Selected process ID: {}", process_id);

            // Leer y mostrar el contenido de la memoria
            display_memory_paginated(*process_id);
        }
        _ => println!("Invalid selection."),
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
            let window_text = String::from_utf8_lossy(&buffer[..length as usize]).to_string();

            // Obtenemos el ID del proceso asociado a la ventana
            let mut process_id = 0u32;
            GetWindowThreadProcessId(hWnd, Some(&mut process_id));

            // Guardamos la ventana en la lista global
            let mut windows_list = FOUND_WINDOWS.lock().unwrap();
            windows_list.push((window_text, process_id));
        }
    }
    true.into()
}

/// Lee y muestra el contenido de la memoria del proceso, paginando los resultados
fn display_memory_paginated(process_id: u32) {
    // Abre el proceso con permisos de lectura
    let process_handle = unsafe {
        OpenProcess(PROCESS_ALL_ACCESS | PROCESS_VM_READ, false.into(), process_id)
    }
        .expect("Failed to open process");

    // Dirección base inicial
    let mut address = 0usize;
    let mut mem_info = MEMORY_BASIC_INFORMATION::default();
    let mut memory_regions = Vec::new();

    while unsafe {
        VirtualQueryEx(
            process_handle,
            Some(address as *const _),
            &mut mem_info,
            size_of::<MEMORY_BASIC_INFORMATION>(),
        )
    } != 0
    {
        memory_regions.push((
            mem_info.BaseAddress as usize,
            mem_info.RegionSize,
            mem_info.State.0,
            mem_info.Protect.0,
        ));

        // Avanzar a la siguiente región de memoria
        address = mem_info.BaseAddress as usize + mem_info.RegionSize;
    }

    // Paginación
    let page_size = 10;
    let total_pages = (memory_regions.len() + page_size - 1) / page_size;

    let mut current_page = 0;
    loop {
        println!("\nMemory Regions (Page {}/{})", current_page + 1, total_pages);

        // Mostrar los elementos de la página actual
        let start = current_page * page_size;
        let end = (start + page_size).min(memory_regions.len());
        for (i, region) in memory_regions[start..end].iter().enumerate() {
            println!(
                "{}. Base Address: 0x{:X}, Region Size: {} bytes, State: 0x{:X}, Protect: 0x{:X}",
                i + 1,
                region.0,
                region.1,
                region.2,
                region.3
            );
        }

        println!("\nOptions:");
        println!("n - Next page");
        println!("p - Previous page");
        println!("s - Select a memory region (1-10)");
        println!("q - Quit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "n" if current_page < total_pages - 1 => current_page += 1,
            "p" if current_page > 0 => current_page -= 1,
            "q" => break,
            "s" => {
                println!("Enter the number of the memory region to select:");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();

                if let Ok(num) = input.trim().parse::<usize>() {
                    if num >= 1 && num <= (end - start) {
                        let selected = &memory_regions[start + num - 1];
                        println!(
                            "Selected Memory Region:\nBase Address: 0x{:X}, Region Size: {} bytes, State: 0x{:X}, Protect: 0x{:X}",
                            selected.0, selected.1, selected.2, selected.3
                        );
                        break;
                    } else {
                        println!("Invalid selection.");
                    }
                }
            }
            _ => println!("Invalid input."),
        }
    }

    unsafe { windows::Win32::Foundation::CloseHandle(process_handle).unwrap() };
}
