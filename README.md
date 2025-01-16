# Memory Hacking Tool (Rewrite in Rust)

This project is a rewrite of a memory hacking tool originally developed in C#. The primary purpose of this tool is to perform memory manipulation on running processes, allowing users to search for specific values in memory, filter the results, and modify them directly. The rewrite leverages the power and safety of Rust, offering enhanced performance and better memory safety guarantees.

## Features

- **Enumerate Windows:** List all visible windows and their associated process IDs.
- **Search Memory:** Scan the memory of a selected process for specific values.
- **Filter Results:** Narrow down search results based on new values.
- **Modify Memory:** Change the value of specific memory addresses directly.
- **Handle Permissions:** Adjust memory protection settings to enable writing if necessary.

## Why Rust?

Rust is chosen for this rewrite due to its unique combination of low-level control and high-level safety features:

- **Memory Safety:** Prevent common bugs such as null pointer dereferences and buffer overflows.
- **Concurrency:** Rust’s ownership model ensures thread safety, making it easier to handle concurrent tasks like parallel memory scanning.
- **Performance:** Rust provides C-like performance, which is critical for real-time memory manipulation tasks.

## Roadmap

- Develop basic memory scanning and modification functionality.
- Add support for different data types (e.g., `int`, `float`, `double`).
- Implement a user-friendly GUI
- Enhance error handling and logging mechanisms.

## Disclaimer

This tool is intended for educational purposes only. Using it to modify the memory of third-party software may violate their terms of service or local laws. Use responsibly and at your own risk.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

