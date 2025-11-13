# Comments Remover

Effortlessly clean up your Python code!

Comments Remover is a small, yet powerful, utility written in **Rust** for the primary purpose of **removing comments from Python scripts**. Whether you're decluttering a large codebase, preparing code for presentations, or simply want a cleaner view of the logic, this tool is designed to help.

### Purpose & Technology

*   **Educational Focus:** Developed with education in mind, this project demonstrates practical application of string manipulation and regular expressions in Rust.
*   **Implementation:** The tool is built entirely in Rust and leverages the power of **Regular Expressions (Regex)** to accurately identify and remove various comment types.
*   **Supported Comment Types:**
    *   Single-line comments (`# ...`)
    *   Multi-line string literals used as comments (`""" ... """` and `''' ... '''`)

### Why Use Comments Remover?

*   **Improved Readability:** Focus on the core Python logic without the distraction of comments.
*   **Code Streamlining:** Quickly prepare code for sharing, documentation, or specific processing steps.
*   **Learning Rust:** A hands-on example of building a command-line utility in Rust.

### Getting Started

#### Building the Project

To build the Comments Remover yourself, you'll need Rust and Cargo installed.
1.  Clone this repository:
    ```bash
    git clone <URL>
    cd comments-remover
    ```
2.  Compile the project:
    ```bash
    cargo build --release
    ```

#### Using Pre-compiled Binaries

For immediate use, you can download a pre-compiled executable from the **[Releases](<add_link>)** page of this repository.

### License

This project is proudly licensed under the **Creative Commons Zero (CC0) License**.
This means you can use, copy, modify, distribute, and perform the work, even for commercial purposes, all without asking permission. It's dedicated to the public domain.

---