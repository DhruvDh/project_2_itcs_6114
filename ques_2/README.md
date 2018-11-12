# ITCS 6114 - Project 2 - Question 1

> Note - I tried providing executables for Mac too, but the program won't compile on Mac for some reason. As such, I hope the Windows binaries can be used instead. It runs perfectly fine on my machine

## Introduction

We're required to compute legal moves for chess pieces after telling the computer where the piece was located. I did not like this way of interacting with the program, and instead chose to simulate a chess board with only one team playing, which requires the same computation of possible legal moves, but provides a much better way of interacting with the program.

## Notation used for position names.

The name of positions for each square is as follows -

```
        - - - - - - - - - - - - - - - - - - - - -
        | 8a | 8b | 8c | 8d | 8e | 8f | 8g | 8h |
        - - - - - - - - - - - - - - - - - - - - -
        | 7a | 7b | 7c | 7d | 7e | 7f | 7g | 7h |
        - - - - - - - - - - - - - - - - - - - - -
        | 6a | 6b | 6c | 6d | 6e | 6f | 6g | 6h |
        - - - - - - - - - - - - - - - - - - - - -
        | 5a | 5b | 5c | 5d | 5e | 5f | 5g | 5h |
        - - - - - - - - - - - - - - - - - - - - -
        | 4a | 4b | 4c | 4d | 4e | 4f | 4g | 4h |
        - - - - - - - - - - - - - - - - - - - - -
        | 3a | 3b | 3c | 3d | 3e | 3f | 3g | 3h |
        - - - - - - - - - - - - - - - - - - - - -
        | 2a | 2b | 2c | 2d | 2e | 2f | 2g | 2h |
        - - - - - - - - - - - - - - - - - - - - -
        | 1a | 1b | 1c | 1d | 1e | 1f | 1g | 1h |
        - - - - - - - - - - - - - - - - - - - - -
```
## Interface

To move a piece, type it's code when prompted, `R1` for Rook, `R2` for the other rook, `K` for king, `Q` for Queen, etc.

```
          - - - - - - - - - - - - - - - - - - - - -
        8 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        7 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        6 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        5 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        4 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        3 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        2 | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
          - - - - - - - - - - - - - - - - - - - - -
        1 | R1 | K1 | B1 | 𝕂  | Ｑ  | B2 | K2 | R2 |
          - - - - - - - - - - - - - - - - - - - - -
            a    b    c    d    e    f    g    h

         * -> indicates possition that can be moved into
R, K.. etc -> indicates initial of the piece placed there
(I totally deserve extra credit for this.)

Which piece would you like to move?
1. Rook (R1, R2)         4. Queen (Q)
2. Knight (K1, K2)       5. King (K)
3. Bishop (B1, B2)       6. Pawn (P1, P2, P3, P4, P5, P6, P7, P8)
```

If there are no legal moves for a piece, the program will let you know by displaying -

```
!=== There are no legal places for that piece to move. Try some other piece. ===!
```

Once a piece is selected, and legal moves for it are found, just type the position option number (`1` for `1*`, `2` for `2*`, and so on) as shown on the board.

```
Currently your K2 is at 1g, as shown below.


          - - - - - - - - - - - - - - - - - - - - -
        8 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        7 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        6 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        5 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        4 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        3 |    |    |    |    |    |  2*|    |  1*|
          - - - - - - - - - - - - - - - - - - - - -
        2 | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
          - - - - - - - - - - - - - - - - - - - - -
        1 | R1 | K1 | B1 | 𝕂  | Ｑ  | B2 | K2 | R2 |
          - - - - - - - - - - - - - - - - - - - - -
            a    b    c    d    e    f    g    h

         * -> indicates possition that can be moved into
R, K.. etc -> indicates initial of the piece placed there
(I totally deserve extra credit for this.)

Where would you like to move it to?
Enter the index of postion you like from the map above -
2
```

And press enter to move the piece.

```
The board is now as follows -


          - - - - - - - - - - - - - - - - - - - - -
        8 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        7 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        6 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        5 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        4 |    |    |    |    |    |    |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        3 |    |    |    |    |    | K2 |    |    |
          - - - - - - - - - - - - - - - - - - - - -
        2 | P1 | P2 | P3 | P4 | P5 | P6 | P7 | P8 |
          - - - - - - - - - - - - - - - - - - - - -
        1 | R1 | K1 | B1 | 𝕂  | Ｑ  | B2 |    | R2 |
          - - - - - - - - - - - - - - - - - - - - -
            a    b    c    d    e    f    g    h

         * -> indicates possition that can be moved into
R, K.. etc -> indicates initial of the piece placed there
(I totally deserve extra credit for this.)
```

## Implementation

This was implemented by making structs for each chess piece - `Piece`, and a struct for an entire white or black set of pieces - `Pieces`.

```
struct Piece {
    name: &'static str,
    _type: &'static str,
    position: (usize, usize),
}
```
`name` stores the name of the piece, `R1`, `K2`, etc. `_type` stores the type of piece, `Pawn`, `Rook`, etc. while `position` stores the current position of the piece.

A set of all the white or black pieces is stored in `Pieces`. Which is defined as follows -

```
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
```

`Box` is a smart pointer type, and `Box<Piece>` implies that is a smart pointer that is pointing to an instance of type `Piece`.

Method to find legal positions for each piece is implemented in `Piece::get_possible()`, while actually moving is implemented in `Pieces::_move()`.

The implementation is pretty straight forward actually, just long.