use std::io::{self, Write};
static NAMES: [&str; 20] = [
    "alanine",
    "agrinine",
    "asparagine",
    "aspartic acid",
    "cysteine",
    "glutamic acid",
    "glutamine",
    "glycine",
    "histidine",
    "isoleucine",
    "leucine",
    "lysine",
    "methionine",
    "phenylalanine",
    "proline",
    "serine",
    "threonine",
    "tryptophan",
    "tyrosine",
    "valine",
];
static THREE_LETTER_SYMBOLS: [&str; 20] = [
    "ala", "arg", "asn", "asp", "cys", "glu", "gln", "gly", "his", "ile", "leu", "lys", "met",
    "phe", "pro", "ser", "thr", "trp", "tyr", "val",
];
static ONE_LETTER_SYMBOLS: [&str; 20] = [
    "a", "r", "n", "d", "c", "e", "q", "g", "h", "i", "l", "k", "m", "f", "p", "s", "t", "w", "y",
    "v",
];
fn main() {
    let operation_choice: i32;

    print!("1. Name of amino acid to it's three and one letter symbols.\n2. Three letter symbol to it's name and one letter symbol.\n3. One letter symbol to it's name and three letter symbol.\n");
    print!("Choose a number: ");
    clean_input();
    operation_choice = get_num_input();
    match operation_choice {
        1 => {
            print!("Enter the name of your amino acid: ");
            clean_input();
            let mut name: String = get_str_input();
            name = name.to_lowercase();
            let i = find_name_index(&name);
            if i != 21 {
                println!(
                    "\nName: {} \nThree Letter Symbol: {} \nOne Letter Symbol: {}",
                    name,
                    THREE_LETTER_SYMBOLS[i].to_uppercase(),
                    ONE_LETTER_SYMBOLS[i].to_uppercase()
                );
            } else {
                println!("\nAmino acid not found, try again.");
            }
        }
        2 => {
            print!("Enter the three letter symbol of your amino acid: ");
            clean_input();
            let mut three_letter_symbol: String = get_str_input();
            three_letter_symbol = three_letter_symbol.to_lowercase();
            let i = find_three_letter_symbol_index(&three_letter_symbol);
            if i != 21 {
                println!(
                    "\nThree Letter Symbol: {} \nName: {} \nOne Letter Symbol: {}",
                    three_letter_symbol.to_uppercase(),
                    NAMES[i],
                    ONE_LETTER_SYMBOLS[i].to_uppercase()
                );
            } else {
                println!("\nAmino acid not found, try again.");
            }
        }
        3 => {
            print!("Enter the one letter symbol of your amino acid: ");
            clean_input();
            let mut one_letter_symbol: String = get_str_input();
            one_letter_symbol = one_letter_symbol.to_lowercase();
            let i = find_one_letter_symbol_index(&one_letter_symbol);
            if i != 21 {
                println!(
                    "\nOne Letter Symbol: {} \nName: {} \nThree Letter Symbol: {}",
                    one_letter_symbol.to_uppercase(),
                    NAMES[i],
                    THREE_LETTER_SYMBOLS[i].to_uppercase()
                );
            } else {
                println!("\nAmino acid not found, try again.");
            }
        }
        _ => {
            println!("Invalid choice.");
            println!("Terminating...");
        }
    }
}

fn get_str_input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    return s.trim().parse().unwrap();
}
fn get_num_input() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    return s.trim().parse().unwrap();
}
fn clean_input() {
    let _ = io::stdout().flush();
}
fn find_name_index(x: &str) -> usize {
    if let Some(index) = NAMES.iter().position(|&r| r == x) {
        return index;
    }
    return 21;
}
fn find_three_letter_symbol_index(x: &str) -> usize {
    if let Some(index) = THREE_LETTER_SYMBOLS.iter().position(|&r| r == x) {
        return index;
    }
    return 21;
}
fn find_one_letter_symbol_index(x: &str) -> usize {
    if let Some(index) = ONE_LETTER_SYMBOLS.iter().position(|&r| r == x) {
        return index;
    }
    return 21;
}
