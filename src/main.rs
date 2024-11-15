mod commands;
mod history;

use std::env;
use std::io::{self, Write};
use std::process::{Command, exit};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use history::CommandHistory;

fn main() {
    let mut command_history = CommandHistory::new();

    loop {
        // Obtener el directorio de trabajo actual
        let current_dir = env::current_dir().unwrap();
        let dir_str = current_dir.to_str().unwrap_or_default();

        // Crear un objeto de salida para escribir en la terminal
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

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