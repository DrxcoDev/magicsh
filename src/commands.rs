use std::env;

pub fn change_directory(command: &str) {
    let dir: &str = command.trim();
    if dir.is_empty() {
        eprintln!("Error: No se especificó un directorio.");
        return;
    }

    if let Err(e) = env::set_current_dir(dir) {
        eprintln!("Error al cambiar de directorio: {}", e);
    }
}
