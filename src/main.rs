use std::process::Command;

#[cfg(test)]
mod test;
mod linked_list;

fn main() {
    let output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to run LinkedList tests");

    let log = output.stdout;
    println!("{}", String::from_utf8_lossy(&log));
}

