use crate::Command;

pub fn arrows_library() {
    let output = Command::new("bash")
     .arg("./arrows.sh")
     .output()
     .expect("Error");

    println!("\n{}", String::from_utf8_lossy(&output.stdout));
}