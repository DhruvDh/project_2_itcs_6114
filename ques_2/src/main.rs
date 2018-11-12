mod lib;
use std::io::stdin;

fn main() {
    // structs to store information on each piece
    let mut _Pieces = lib::Pieces::new();

    println!("\n\nThe chess board used in this game is mapped as follows -");
    println!("1. All the 8 rows are refered to using the numbers [1-8] respectively");
    println!("2. Similarly, all the 8 columns are refered to using the alphabets [a-h]");
    println!("\n\nThus, the possible positions are -");
    draw_empty();
    println!("Let's start the game now -");

    let names = vec![
        "R1", "K1", "B1", "R2", "K2", "B2", "Q", "K", "P1", "P2", "P3", "P4", "P5", "P6", "P7",
        "P8",
    ];

    loop {
        let selection = which_piece();
        let name = String::from(names[selection as usize]);
        _Pieces._move(name);
    }
}

// function that draws a blank board with position names
fn draw_empty() {
    print!("\n\n");

    for row in vec![1, 2, 3, 4, 5, 6, 7, 8].iter().rev() {
        println!("\t- - - - - - - - - - - - - - - - - - - - -");
        print!("\t");
        for column in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
            print!("| {}{} ", row, column);
        }
        print!("|\n");
    }

    println!(" \t- - - - - - - - - - - - - - - - - - - - -\n");
}

// self explanatory
fn which_piece() -> usize {
    println!("");
    println!("Which piece would you like to move?");
    println!("1. Rook (R1, R2)\t 4. Queen (Q)");
    println!("2. Knight (K1, K2)\t 5. King (K)");
    println!("3. Bishop (B1, B2)\t 6. Pawn (P1, P2, P3, P4, P5, P6, P7, P8)");

    println!("\nEnter piece name (R1, K2, B1, K and Q for King, Queen, etc): (press n to exit)");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("I couldn't read that :(");

    let input = input.trim();
    let input = input.to_lowercase();

    let names = vec![
        "r1", "k1", "b1", "r2", "k2", "b2", "q", "k", "p1", "p2", "p3", "p4", "p5", "p6", "p7",
        "p8",
    ];

    let index = names.iter().position(|&r| r == input).unwrap_or(99);

    if index < 16 {
        index
    } else {
        if input.parse::<char>().unwrap_or('y') == 'n'
            || input.parse::<char>().unwrap_or('Y') == 'N'
        {
            std::process::exit(0);
        }
        println!("This doesn't feel like one of the options I told you about. Are you sure you typed correctly?");
        which_piece()
    }
}
