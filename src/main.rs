use std::env;
use std::fs::{self, DirEntry};
use std::io::{self, Write};
use std::process::{Command, exit};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use chrono::Local; // Importamos `chrono` para manejar la fecha y hora
use std::path::Path;

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

    // Establecer el color rojo para la flecha
    let mut arrow_color = ColorSpec::new();
    arrow_color.set_fg(Some(Color::Red));

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

                stdout.set_color(&arrow_color).unwrap(); // Establecer color de la flecha
                write!(stdout, "↪ ").unwrap(); // Imprimir la flecha
                stdout.reset().unwrap();

                stdout.set_color(&color).unwrap(); // Establecer el color para el archivo
                write!(stdout, "{:<20} {:>10} bytes", file_name, file_size).unwrap();
                stdout.reset().unwrap();
                println!(); // Nueva línea después de cada archivo
            }
            Err(_) => continue,
        }
    }
}

// Comando "now" que muestra la fecha y hora actual
fn show_current_time() {
    let now = Local::now(); // Obtiene la fecha y hora actual
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string(); // Formato: "2024-11-15 14:30:00"
    
    // Mostrar la flecha roja antes de la fecha y hora
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut arrow_color = ColorSpec::new();
    arrow_color.set_fg(Some(Color::Red));

    stdout.set_color(&arrow_color).unwrap();
    write!(stdout, "↪ ").unwrap();
    stdout.reset().unwrap();

    println!("{}", formatted_time); // Muestra la fecha y hora
}

fn main() {
    let mut command_history = CommandHistory::new();

    loop {
        // Obtener el directorio de trabajo actual
        let current_dir = env::current_dir().unwrap();
        let dir_str = current_dir.to_str().unwrap_or_default();

        // Crear un objeto de salida para escribir en la terminal
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        // Establecer el color verde para el directorio
        let mut green = ColorSpec::new();
        green.set_fg(Some(Color::Green)).set_bold(true);
        stdout.set_color(&green).unwrap();

        // Establecer el color rojo para la flecha del prompt
        let mut arrow_color = ColorSpec::new();
        arrow_color.set_fg(Some(Color::Red));

        // Imprimir la flecha roja antes del directorio y el prompt
        stdout.set_color(&arrow_color).unwrap();
        write!(stdout, "").unwrap();
        stdout.reset().unwrap();

        // Imprimir el directorio actual en color verde
        stdout.set_color(&green).unwrap();
        write!(stdout, "{:?} $ ", dir_str).unwrap();
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

        // Comando "now" que muestra la fecha y hora
        if command_input == "now" {
            show_current_time();
            continue;
        }

        // Intentar ejecutar el comando
        let mut parts = command_input.split_whitespace();
        let command = parts.next().unwrap_or_default();

        let output = Command::new(command).args(parts).output();

        match output {
            Ok(output) => {
                if !output.stdout.is_empty() {
                    // Agregar la flecha a la salida estándar
                    print!("↪ ");
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }
                if !output.stderr.is_empty() {
                    // Agregar la flecha a la salida de error
                    eprint!("→ ");
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
