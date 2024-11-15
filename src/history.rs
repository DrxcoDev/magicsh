pub struct CommandHistory {
    history: Vec<String>,
}

impl CommandHistory {
    pub fn new() -> Self {
        CommandHistory {
            history: Vec::new(),
        }
    }

    pub fn add_command(&mut self, command: &str) {
        if !command.trim().is_empty() {
            self.history.push(command.to_string());
        }
    }

    pub fn show_history(&self) {
        if self.history.is_empty() {
            println!("No hay comandos en el historial.");
        } else {
            for (index, cmd) in self.history.iter().enumerate() {
                println!("{}: {}", index + 1, cmd);
            }
        }
    }
}
