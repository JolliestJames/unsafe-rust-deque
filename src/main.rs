use std::process::Command;

mod linked_list;
#[cfg(test)]
mod test;

fn main() {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to run LinkedList tests");

    let log = output.stdout;
    println!("{}", String::from_utf8_lossy(&log));
}
