# sudoku-solver-rust

A port of the algorithm used in my javascript sudoku solver to Rust. Unlike it's web counterpart, this is optimized for speed, and does not provide any visual output while solving.

## Usage
Clone this repository and run `cargo build --release` to build the executable. Run it from the command line, providing a filename as command line argument.

`sudoku-solver sudoku.txt`

The text file should contain exactly nine lines, with nine numbers each. Use `0` for empty positions in the sudoku.

```
600008940
900006100
070040000
200610000
000000200
089002000
000060005
000000030
800001600
```

## Javascript original
[tanck.nl/sudoku](https://tanck.nl/sudoku/)
[github.com/roytanck/sudoku-solver](https://github.com/roytanck/sudoku-solver)
