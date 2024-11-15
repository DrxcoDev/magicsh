# Magicsh - A Custom Shell in Rust

**Magicsh** is a custom shell built from scratch using Rust. It is designed to be simple, efficient, and highly customizable. Magicsh allows you to execute system commands, change directories, and view command history, all while offering a visually appealing and user-friendly terminal experience. With additional features like showing the current date and time, listing files with sizes and colors, and a prompt with customizable colors, Magicsh is both functional and visually appealing.

## Features

### 1. **Customizable Colors**
   - The current directory is displayed in **green** to make it stand out.
   - The command prompt includes a **red arrow** for decoration, providing a more dynamic look.
   - The file sizes in the `ls` command are color-coded to indicate the size of the files:
     - Small files (below 100 KB) are shown in **blue**.
     - Medium files (100 KB to 10 MB) are displayed in **yellow**.
     - Large files (above 10 MB) are shown in **red**.
   
### 2. **Improved `ls` Command**
   - Unlike the standard `ls`, the **Magicsh** `ls` command not only lists files and directories but also shows their **sizes** in bytes.
   - The output is color-coded based on the size of the files, making it easy to identify large files at a glance.

### 3. **`now` Command**
   - The `now` command displays the **current date and time** in a numeric format, offering a quick way to check the current system time directly from the shell.

### 4. **Command History**
   - Magicsh maintains a **command history** that allows you to review past commands.
   - The `history` command shows a list of previously executed commands, making it easy to navigate through your command history.

### 5. **Change Directory (`cd`)**
   - You can navigate through the filesystem using the `cd` command, just like a standard shell.
   - The shell automatically updates the working directory without requiring you to open a new shell instance.

### 6. **Extensibility**
   - Magicsh is designed to be easily extensible. You can add new commands or modify existing ones to fit your needs.
   - The codebase is clean, well-organized, and follows Rust’s best practices, allowing developers to contribute or adapt it to their own projects.

## Requirements

To run Magicsh, you will need the following:

- **Rust 1.x or later**: Magicsh is built using Rust, so you'll need to have Rust and Cargo installed.
- **Cargo**: The package manager for Rust, which is used to build and manage the project dependencies.

## Installation

Follow these steps to install Magicsh on your system:

1. **Clone the Repository**
   First, clone the repository from GitHub to your local machine:
   ```bash
   git clone https://github.com/drxcodev/magicsh.git
   ```

2. **Change to into directory**
   ```bash
   cd magicsh
   ```
3. **Build the project**
   ```bash
   cargo build --release
   ```
4. **Run the shell**   
   ```bash
   ./target/release/magicsh
   ```
   
