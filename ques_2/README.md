# ITCS 6114 - Project 2 - Question 1

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

Once a piece is selected, and legal moves for it are found