use hash_table::{HashTable, SlotStatus, parse_operations};
use std::io::{self, Write};

fn main() {
    print!("Commands: ");
    let _ = io::stdout().flush();

    let mut operations_string = String::new();
    io::stdin().read_line(&mut operations_string)
        .expect("Error reading input.");
    let operations = parse_operations(&operations_string);

    let mut ht = HashTable::new();
    ht.execute_operations(operations);

    let mut values: Vec<&str> = Vec::new();
    for slot in &ht.inner {
        match slot {
            SlotStatus::Occupied(value) => {
                values.push(value)
            },
            _ => continue
        }
    }
    println!("{}", values.join(" "));
}
