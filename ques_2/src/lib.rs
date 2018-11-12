use std::io::stdin;

struct Piece {
    name: &'static str,
    _type: &'static str,
    position: (usize, usize),
}

pub struct Pieces {
    R1: Box<Piece>,
    K1: Box<Piece>,
    B1: Box<Piece>,
    R2: Box<Piece>,
    K2: Box<Piece>,
    B2: Box<Piece>,
    Queen: Box<Piece>,
    King: Box<Piece>,
    P1: Box<Piece>,
    P2: Box<Piece>,
    P3: Box<Piece>,
    P4: Box<Piece>,
    P5: Box<Piece>,
    P6: Box<Piece>,
    P7: Box<Piece>,
    P8: Box<Piece>,
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
    let names = vec![
        "R1", "K1", "B1", "R2", "K2", "B2", "Ｑ ", "𝕂 ", "P1", "P2", "P3", "P4", "P5", "P6",
        "P7", "P8",
    ];

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

                print!("| {} ", names[index]);
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

        println!(
            "\nCurrently your {} is at {}{}, as shown below.",
            self.name,
            self.position_name().0,
            self.position_name().1
        );

        // vector to store possible locations
        let mut can_go: Vec<(usize, usize)> = Vec::new();

        // a switch like statement to execute logic differently for each piece.
        match self._type {
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

                        if (x < 0 || x > 7) || (y < 0 || y > 7) {
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

                        if (x < 0 || x > 7) || (y < 0 || y > 7) {
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
                    let _x = x + 1;
                    let __x = x + 2;
                    if not_occupied!(_x, y) && not_occupied!(__x, y) {
                        can_go.push((_x, y));
                        can_go.push((__x, y));
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

    // a method to translate indices into proper names on the map
    fn position_name(&self) -> (usize, char) {
        let x = self.position.0 + 1;
        let y = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'][self.position.1];
        (x, y)
    }
}

// helper functions
impl Pieces {
    pub fn new() -> Pieces {
        // structs to store information on each piece
        let R1 = Piece {
            name: "R1",
            _type: "Rook",
            position: (0, 0),
        };

        let R2 = Piece {
            name: "R2",
            _type: "Rook",
            position: (0, 7),
        };

        let K1 = Piece {
            name: "K1",
            _type: "Knight",
            position: (0, 1),
        };

        let K2 = Piece {
            name: "K2",
            _type: "Knight",
            position: (0, 6),
        };

        let B1 = Piece {
            name: "B1",
            _type: "Bishop",
            position: (0, 2),
        };

        let B2 = Piece {
            name: "B2",
            _type: "Bishop",
            position: (0, 5),
        };

        let King = Piece {
            name: "𝕂 ",
            _type: "King",
            position: (0, 3),
        };

        let Queen = Piece {
            name: "Ｑ ",
            _type: "Queen",
            position: (0, 4),
        };

        let P1 = Piece {
            name: "P1",
            _type: "Pawn",
            position: (1, 0),
        };

        let P2 = Piece {
            name: "P2",
            _type: "Pawn",
            position: (1, 1),
        };

        let P3 = Piece {
            name: "P3",
            _type: "Pawn",
            position: (1, 2),
        };

        let P4 = Piece {
            name: "P4",
            _type: "Pawn",
            position: (1, 3),
        };

        let P5 = Piece {
            name: "P5",
            _type: "Pawn",
            position: (1, 4),
        };

        let P6 = Piece {
            name: "P6",
            _type: "Pawn",
            position: (1, 5),
        };

        let P7 = Piece {
            name: "P7",
            _type: "Pawn",
            position: (1, 6),
        };

        let P8 = Piece {
            name: "P8",
            _type: "Pawn",
            position: (1, 7),
        };

        // struct to store pointers to all pieces
        // Box is basically a smart pointer. I don't understand it entirely, but it works.

        Pieces {
            R1: Box::from(R1),
            K1: Box::from(K1),
            B1: Box::from(B1),
            R2: Box::from(R2),
            K2: Box::from(K2),
            B2: Box::from(B2),
            Queen: Box::from(Queen),
            King: Box::from(King),
            P1: Box::from(P1),
            P2: Box::from(P2),
            P3: Box::from(P3),
            P4: Box::from(P4),
            P5: Box::from(P5),
            P6: Box::from(P6),
            P7: Box::from(P7),
            P8: Box::from(P8),
        }
    }
    // function that moves pieces
    pub fn _move(&mut self, piece: String) {
        // get list of positions that are occupied
        let occupied = self.list_positions();

        // match name of piece with a pointer to the respective piece's object
        let piece = match piece.as_str() {
            "R1" => &mut self.R1,
            "K1" => &mut self.K1,
            "B1" => &mut self.B1,
            "R2" => &mut self.R2,
            "K2" => &mut self.K2,
            "B2" => &mut self.B2,
            "Q" => &mut self.Queen,
            "K" => &mut self.King,
            "P1" => &mut self.P1,
            "P2" => &mut self.P2,
            "P3" => &mut self.P3,
            "P4" => &mut self.P4,
            "P5" => &mut self.P5,
            "P6" => &mut self.P6,
            "P7" => &mut self.P7,
            "P8" => &mut self.P8,
            _ => std::process::exit(1),
        };

        // get possible locations for that object
        let can_go = piece.get_possible(occupied);

        // if the piece has moved for the first time
        if can_go.len() == 1 && can_go[0] == (11, 11) {
            println!("\nThe board is now as follows -");
            draw_possibles(self.list_positions(), vec![]);
            return;
        }
        // if there were no possible locatiosn found
        else if can_go.len() == 1 && can_go[0] == (10, 10) {
            println!("\n!=== There are no legal places for that piece to move. Try some other piece. ===!");
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
        let mut list: Vec<(usize, usize)> = Vec::with_capacity(16);
        list.push(self.R1.position);
        list.push(self.K1.position);
        list.push(self.B1.position);
        list.push(self.R2.position);
        list.push(self.K2.position);
        list.push(self.B2.position);
        list.push(self.Queen.position);
        list.push(self.King.position);
        list.push(self.P1.position);
        list.push(self.P2.position);
        list.push(self.P3.position);
        list.push(self.P4.position);
        list.push(self.P5.position);
        list.push(self.P6.position);
        list.push(self.P7.position);
        list.push(self.P8.position);
        list
    }
}
