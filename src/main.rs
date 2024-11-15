use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::process::{Command, exit};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::path::Path;
use chrono::prelude::*;

mod commands;
mod history;
use history::CommandHistory;

fn list_files_with_size() {
    // Obtener el directorio actual
    let current_dir = env::current_dir().unwrap();
    let dir_str = current_dir.to_str().unwrap_or_default();
    let entries = fs::read_dir(current_dir).unwrap();

    // Crear un objeto de salida para escribir en la terminal
    let mut stdout = StandardStream::stdout(ColorChoice::Always);


    // Recorrer las entradas del directorio
    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = path.file_name().unwrap_or_default().to_str().unwrap_or_default();

                // Obtener el tamaño del archivo
                let file_size = fs::metadata(&path).unwrap().len();

                // Establecer un color basado en el tamaño del archivo
                let mut color = ColorSpec::new();
                if file_size < 1024 {
                    color.set_fg(Some(Color::Blue)); // Tamaños pequeños en azul
                } else if file_size < 1048576 {
                    color.set_fg(Some(Color::Yellow)); // Tamaños medianos en amarillo
                } else {
                    color.set_fg(Some(Color::Red)); // Tamaños grandes en rojo
                }

                stdout.set_color(&color).unwrap();
                
                // Mostrar el archivo y su tamaño
                write!(stdout, "{:<20} {:>10} bytes", file_name, file_size).unwrap();
                stdout.reset().unwrap();
                println!(); // Nueva línea después de cada archivo
            }
            Err(_) => continue,
        }
    }
}

fn main() {
    let mut command_history = CommandHistory::new();

    loop {
        // Obtener el directorio de trabajo actual
        let current_dir = env::current_dir().unwrap();
        let dir_str = current_dir.to_str().unwrap_or_default();

        // Crear un objeto de salida para escribir en la terminal
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        let now: fn() -> DateTime<Utc> = Utc::now();

        // Establecer el color verde para el prompt
        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green)).set_bold(true);
        stdout.set_color(&green).unwrap();

        // Imprimir el nombre de usuario y directorio actual como el prompt
        write!(stdout, "{:?} $ ", dir_str).unwrap();

        // Devolver el color al normal
        stdout.reset().unwrap();

        // Capturar el input del usuario
        let mut command_input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command_input).unwrap();

        let command_input = command_input.trim();

        // Si el comando está vacío, continuar al siguiente ciclo
        if command_input.is_empty() {
            continue;
        }

        // Añadir el comando al historial
        command_history.add_command(command_input);

        // Manejar comandos personalizados
        if command_input == "exit" {
            break;
        }

        if command_input == "history" {
            command_history.show_history();
            continue;
        }

        if command_input.starts_with("cd ") {
            commands::change_directory(&command_input[3..]);
            continue;
        }

        // Comando 'ls' con tamaños coloreados
        if command_input == "ls" {
            list_files_with_size();
            continue;
        }
        

        // Intentar ejecutar el comando
        let mut parts = command_input.split_whitespace();
        let command = parts.next().unwrap_or_default();

        let output = Command::new(command).args(parts).output();

        match output {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                eprintln!("Error al ejecutar el comando: {}", e);
                exit(1);
            }
        }
    }
}
