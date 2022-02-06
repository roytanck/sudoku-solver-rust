# sudoku-solver-rust

A port of the algorithm used in my javascript brute force sudoku solver to Rust. Unlike it's web counterpart, this is optimized for speed, and does not provide any visual output while solving.

## Usage
Clone this repository and run `cargo build --release` to build the executable. Run it from the command line, providing a filename as command line argument.

`sudoku-solver --input sudoku.txt`

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

A number of example files are provided.
* `extreme.txt` (a very difficult puzzle from an app)
* `unsolvable.txt` (the most difficult sudoku puzzle, according to sudokuwiki.org)
* `protected.txt` (a puzzle that was designed to frustrate brute force sudoku solvers)

### Options

`--verbose` displays the input puzzle, the solution and soem statistics.

`--benchmark [number]` solves the same puzzle multiple times and provides some statistics.

`--threads [number]` sets the number of CPU threads to use when benchmarking.

### Examples

Solve the Sudoku in extreme.txt and display input, solution and statisitics:

`sudoku-solver --input extreme.txt --verbose`

Solve unsolvable.txt 100 times and show statistics:

`sudoku-solver --input unsolvable.txt --benchmark 100`

Solve 50 times, show only the total numberof milliseconds needed:

`sudoku-solver --input extreme.txt --benchmark 50`

Solve extreme.txt and output the solution to a new file called output.txt:

`sudoku-solver --input extreme.txt > output.txt`

Solve unsolvable.txt 100 times using 3 CPU threads:

`sudoku-solver --input unsolvable.txt --benchmark 100 --threads 3`

## Javascript original
* [tanck.nl/sudoku](https://tanck.nl/sudoku/)
* [github.com/roytanck/sudoku-solver](https://github.com/roytanck/sudoku-solver)
