use std::io::stdin;

fn main() {
    // structs to store information on each piece
    let Rook = Piece {
        name: "Rook",
        position: (10, 10),
        has_moved: false,
    };

    let Knight = Piece {
        name: "Knight",
        position: (10, 10),
        has_moved: false,
    };

    let Bishop = Piece {
        name: "Bishop",
        position: (10, 10),
        has_moved: false,
    };

    let Queen = Piece {
        name: "Queen",
        position: (10, 10),
        has_moved: false,
    };

    let King = Piece {
        name: "King",
        position: (10, 10),
        has_moved: false,
    };

    let Pawn = Piece {
        name: "Pawn",
        position: (10, 10),
        has_moved: false,
    };

    // struct to store pointers to all pieces
    // Box is basically a smart pointer. I don't understand it entirely, but it works.
    let mut _Pieces = Pieces {
        Rook: Box::from(Rook),
        Knight: Box::from(Knight),
        Bishop: Box::from(Bishop),
        Queen: Box::from(Queen),
        King: Box::from(King),
        Pawn: Box::from(Pawn),
    };

    println!("\n\nThe chess board used in this game is mapped as follows -");
    println!("1. All the 8 rows are refered to using the numbers [1-8] respectively");
    println!("2. Similarly, all the 8 columns are refered to using the alphabets [a-h]");
    println!("\n\nThus, the possible positions are -");
    draw_board();
    println!("Let's start the game now -");

    let names = vec!["Rook", "Knight", "Bishop", "Queen", "King", "Pawn"];

    loop {
        let selection = which_piece() - 1;
        let name = String::from(names[selection as usize]);
        _Pieces._move(name);
    }
}

// function that draws a blank board with position names
fn draw_board() {
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

// function to draw possible locations for a piece to move on a board
fn draw_possibles(occupied: Vec<(usize, usize)>, can_go: Vec<(usize, usize)>) {
    // a macro to check whether position is in the provided vector
    macro_rules! is_in {
        ($vec: ident, $x: expr, $y: expr) => {{
            let mut found = false;
            for pos in $vec.iter() {
                if $x as usize == pos.0 && $y as usize == pos.1 {
                    found = true;
                    break;
                }
            }
            found
        }};
    }

    print!("\n\n");
    // list of first letters of all pieces, in order.
    let piece_initials = vec!['R', 'K', 'B', 'Q', 'K', 'P'];

    for row in vec![1, 2, 3, 4, 5, 6, 7, 8].iter().rev() {
        println!("\t  - - - - - - - - - - - - - - - - - - - - -");

        print!("\t{} ", row);
        for column in 0..8 {
            // check if postion is occupied
            if is_in!(occupied, (row - 1), column) {
                // find which piece is occupying the position
                let index = occupied
                    .iter()
                    .position(|&r| r.0 == (row - 1 as usize) && r.1 == column)
                    .unwrap();

                print!("|  {} ", piece_initials[index]);
            }
            // check if position is a possible location
            else if is_in!(can_go, (row - 1), column) {
                // find option number for that location
                let index = can_go
                    .iter()
                    .position(|&r| r.0 == (row - 1 as usize) && r.1 == column)
                    .unwrap();
                // just for printing uniformly
                if index < 9 {
                    print!("|  {}*", index + 1);
                } else {
                    print!("| {}*", index + 1);
                }
            } else {
                print!("|    ");
            }
        }
        print!("|\n");
    }
    println!(" \t  - - - - - - - - - - - - - - - - - - - - -");
    print!("\t ");
    for column in vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'] {
        print!("   {} ", column);
    }

    println!("\n\n         * -> indicates possition that can be moved into");
    println!("R, K.. etc -> indicates initial of the piece placed there",);
    println!("(I totally deserve extra credit for this.)");
}

// self explanatory
fn which_piece() -> isize {
    println!("");
    println!("Which piece would you like to move?");
    println!("1. Rook \t 4. Queen");
    println!("2. Knight\t 5. King");
    println!("3. Bishop\t 6. Pawn");

    println!("\nEnter piece number: (press n to exit)");
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("I couldn't read that :(");

    let input = input.trim();

    // parse error handling
    match input.parse::<isize>() {
        Ok(i) => {
            if i > 0 && i < 7 {
                i
            } else {
                println!("You sure you entered a number between 1-6? You're supposed to enter a number between 1-6");
                which_piece()
            }
        }
        Err(e) => {
            if input.parse::<char>().unwrap_or('y') == 'n'
                || input.parse::<char>().unwrap_or('Y') == 'N'
            {
                std::process::exit(0);
            }
            println!("This doesn't feel like one of the options I told you about. Are you sure you typed correctly? \n{}", e);
            which_piece()
        }
    }
}

struct Piece {
    name: &'static str,
    position: (usize, usize),
    has_moved: bool,
}

struct Pieces {
    Rook: Box<Piece>,
    Knight: Box<Piece>,
    Bishop: Box<Piece>,
    Queen: Box<Piece>,
    King: Box<Piece>,
    Pawn: Box<Piece>,
}

impl Piece {
    fn get_possible(&mut self, occupied: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        // a macro to check whether position is occupied
        macro_rules! not_occupied {
            ($x: ident, $y: ident) => {{
                let mut found = true;
                for pos in occupied.iter() {
                    if $x as usize == pos.0 && $y as usize == pos.1 {
                        found = false;
                        break;
                    }
                }
                found
            }};
        }

        // check whether this piece has been placed somewhere by the user yet
        if !self.has_moved {
            println!(
                "\nI don't have a position for this piece yet. Where would you like to place {}?",
                self.name
            );
            println!("(Just for reference positions look like 1a, 4b, 7h and so on..)");

            // some code to take input
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("I couldn't read that :(");

            let input = input.trim();

            // just some code to validate input
            if input.len() == 2 {
                let mut chars = input.chars();

                // checking if the first char is a valid position
                let first = match chars.next().unwrap().to_digit(10) {
                    Some(number) => number,
                    None => {
                        println!("\nIdk dude that didn't look like a position to me. Try again?");
                        return self.get_possible(occupied);
                    }
                };

                // checking the second
                let second = match chars.next().unwrap().to_digit(18) {
                    Some(number) => {
                        if number >= 10 {
                            number - 10
                        } else {
                            println!(
                                "\nIdk dude that didn't look like a position to me. Try again?"
                            );
                            return self.get_possible(occupied);
                        }
                    }
                    None => {
                        println!("\nIdk dude that didn't look like a position to me. Try again?");
                        return self.get_possible(occupied);
                    }
                };

                let first = first - 1;
                // check if entered position is not already occupied
                if not_occupied!(first, second) {
                    self.position = (first as usize, second as usize);
                } else {
                    println!("That position is taken, try another..");
                    return self.get_possible(occupied);
                }

                // set the has moved flag to true
                self.has_moved = true;

                // return an invalid value since piece can't move this turn
                return vec![(11, 11)];
            } else {
                println!("\nIdk dude that didn't look like a position to me. Try again?");
                return self.get_possible(occupied);
            }
        } else {
            // else clause - if the piece has been placed by the user somewhere before
            println!(
                "\nCurrently your {} is at {}{}, as shown below.`",
                self.name.chars().next().unwrap(),
                self.position_name().0,
                self.position_name().1
            );

            // vector to store possible locations
            let mut can_go: Vec<(usize, usize)> = Vec::new();

            // a switch like statement to execute logic differently for each piece.
            match self.name {
                "Rook" => {
                    let mut x = self.position.0;
                    let y = self.position.1;

                    // going straight up top
                    while x != 7 {
                        x = x + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let y = self.position.1;

                    // going straight down bottom
                    while x != 0 {
                        x = x - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let x = self.position.0;
                    let mut y = self.position.1;

                    // going straight to right
                    while y != 7 {
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let x = self.position.0;
                    let mut y = self.position.1;

                    // going straight to left
                    while y != 0 {
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go // returning possible locations
                    }
                }

                "Knight" => {
                    // first possibility
                    for x_offset in vec![2, -2] {
                        for y_offset in vec![1, -1] {
                            let x = self.position.0 as isize + x_offset;
                            let y = self.position.1 as isize + y_offset;

                            if (x < 0 || x >= 7) || (y < 0 || y >= 7) {
                                continue;
                            }

                            if not_occupied!(x, y) {
                                can_go.push((x as usize, y as usize));
                            }
                        }
                    }

                    // sencond possibility
                    for x_offset in vec![1, -1] {
                        for y_offset in vec![2, -2] {
                            let x = self.position.0 as isize + x_offset;
                            let y = self.position.1 as isize + y_offset;

                            if (x < 0 || x >= 7) || (y < 0 || y >= 7) {
                                continue;
                            }

                            if not_occupied!(x, y) {
                                can_go.push((x as usize, y as usize));
                            }
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go
                    }
                }

                "Bishop" => {
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight top-right
                    while x != 7 && y != 7 {
                        x = x + 1;
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight top-left
                    while x != 7 && y != 0 {
                        x = x + 1;
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight bottom-right
                    while x != 0 && y != 0 {
                        x = x - 1;
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight bottom-right
                    while x != 0 && y != 7 {
                        x = x - 1;
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go
                    }
                }

                "Queen" => {
                    let mut x = self.position.0;
                    let y = self.position.1;

                    // going straight up top
                    while x != 7 {
                        x = x + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let y = self.position.1;

                    // going straight down bottom
                    while x != 0 {
                        x = x - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let x = self.position.0;
                    let mut y = self.position.1;

                    // going straight to right
                    while y != 7 {
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let x = self.position.0;
                    let mut y = self.position.1;

                    // going straight to left
                    while y != 0 {
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }

                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight top-right
                    while x != 7 && y != 7 {
                        x = x + 1;
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight top-left
                    while x != 7 && y != 0 {
                        x = x + 1;
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight bottom-right
                    while x != 0 && y != 0 {
                        x = x - 1;
                        y = y - 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }
                    let mut x = self.position.0;
                    let mut y = self.position.1;

                    // going straight bottom-right
                    while x != 0 && y != 7 {
                        x = x - 1;
                        y = y + 1;
                        if not_occupied!(x, y) {
                            can_go.push((x, y));
                        } else {
                            break;
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go
                    }
                }
                "King" => {
                    let x = self.position.0 as isize;
                    let y = self.position.1 as isize;

                    // checking in a grid around the king for legal locations to move
                    for x_offset in vec![-1, 0, 1] {
                        let _x = x + x_offset;
                        for y_offset in vec![-1, 0, 1] {
                            let _y = y + y_offset;
                            if not_occupied!(_x, _y) {
                                if _x >= 0 && _x <= 7 {
                                    if y >= 0 && y <= 7 {
                                        can_go.push((_x as usize, _y as usize))
                                    }
                                }
                            }
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go
                    }
                }

                "Pawn" => {
                    let x = self.position.0;
                    let y = self.position.1;

                    // checking if pawn is in the second row, if so moving two steps
                    if x == 1 {
                        let _x = x + 2;
                        if not_occupied!(_x, y) {
                            can_go.push((_x, y));
                        }
                    } else {
                        // else just moving one
                        let _x = x + 1;
                        if not_occupied!(_x, y) {
                            if _x != 8 {
                                can_go.push((_x, y));
                            }
                        }
                    }

                    // drawing the board to give options
                    draw_possibles(occupied, can_go.clone());
                    if can_go.len() == 0 {
                        vec![(10, 10)]
                    } else {
                        can_go
                    }
                }
                _ => std::process::exit(1), // if the piece has an invalid name, which will never happen.
            }
        }
    }

    // a method to translate indices into proper names on the map
    fn position_name(&self) -> (usize, char) {
        let x = self.position.0 + 1;
        let y = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.position.1];
        (x, y)
    }
}

// helper functions
impl Pieces {
    // function that moves pieces
    pub fn _move(&mut self, piece: String) {
        // get list of positions that are occupied
        let occupied = self.list_positions();

        // match name of piece with a pointer to the respective piece's object
        let piece = match piece.as_str() {
            "Rook" => &mut self.Rook,
            "Knight" => &mut self.Knight,
            "Bishop" => &mut self.Bishop,
            "Queen" => &mut self.Queen,
            "King" => &mut self.King,
            "Pawn" => &mut self.Pawn,
            _ => std::process::exit(1),
        };

        // get possible locations for that object
        let can_go = piece.get_possible(occupied);

        // if the piece has moved for the first time
        if can_go.len() == 1 && can_go[0] == (11, 11) {
            println!("The board is now as follows -");
            draw_possibles(self.list_positions(), vec![]);
            return;
        }
        // if there were no possible locatiosn found
        else if can_go.len() == 1 && can_go[0] == (10, 10) {
            println!("There is no where for the piece to move. Try some other piece");
            draw_possibles(self.list_positions(), vec![]);
        }
        // if some locations were found
        else {
            println!("\nWhere would you like to move it to?");
            println!("Enter the index of postion you like from the map above - ");

            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("I couldn't read that :(");

            let input = input.trim();
            let input = input.parse::<usize>().unwrap_or(542335374); // just a random number that user would never enter, used for error handling

            // input is = 542335374 is parsing to unsigned integer failed.
            if input == 542335374 || input > can_go.len() {
                println!("That doesn't seem like one of the options presented to you.");
                println!("Stop being naughty and try again.");
            } else {
                piece.position = can_go[input - 1];
            }

            println!("The board is now as follows -");
            draw_possibles(self.list_positions(), vec![])
        }
    }

    // function that returns the list of positions of all pieces in order.
    pub fn list_positions(&self) -> Vec<(usize, usize)> {
        let mut list: Vec<(usize, usize)> = Vec::with_capacity(6);
        list.push(self.Rook.position);
        list.push(self.Knight.position);
        list.push(self.Bishop.position);
        list.push(self.Queen.position);
        list.push(self.King.position);
        list.push(self.Pawn.position);
        list
    }
}
