// Booleans (`bool`)

fn main() {
    let is_morning = true;
    
    if is_morning {
        println!("Good morning!");
    }

    // Define a boolean variable with the name `is_evening`
    let is_evening = !is_morning; // Negation of `is_morning`

    if is_evening {
        println!("Good evening!");
    }
}
