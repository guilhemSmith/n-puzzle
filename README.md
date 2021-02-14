# n-puzzle

n-puzzle is a simple solver for the eponyme game using a custom implementation of the a* algorithm.  

## Compilation

This project compiles with rust.  
To install the rust toolchain: https://www.rust-lang.org/tools/install  
Once the toolchain is ready, use `cargo build --release`  
The executable should be located at `target/release/n-puzzle`  

## puzzle format

### Input Example
```
# a comment
3 # the dimension for this puzzle
8 1 3
0 2 4 # the empty space is the zero
5 6 7
```

### Output
The ouput follow the snail format, the following example is a solved dimension 3 puzzle:  
```
1 2 3
8 0 4
7 6 5
```

## Usage

    n-puzzle [FLAGS] [OPTIONS]

### FLAGS  
| short | long               | description                                         |
| ----- | ------------------ | --------------------------------------------------- |
|       | --help             | Prints help information                             |
| -V    | --version          | Prints version information                          |
| -W    | --without_solution | Specify the generated puzzle to not have a solution |

### OPTIONS
| short | long          | value           | description                                                                                                                                                                      |
| ----- | ------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| -d    | --dimension   | NUMBER          | The dimension of the puzzle to generate, it will have dimension x dimension cells (possible values: 3, 4, 5, 6, 7)                                                               |
| -f    | --file        | FILENAME        | The file to read the puzzle from                                                                                                                                                 |
| -h    | --heuristic   | HEURISTIC       | The heuristic used to estimate the distance of each state to the goal (default: linear_conflict+manhattan) (possible values: hamming, manhattan,linear_conflict+manhattan)       |
| -i    | --iterations  | NUMBER          | The number of iterations to do when generating a puzzle                                                                                                                          |
| -s    | --search_type | SEARCH_TYPE     | The search type used to set the score of each state (default: best_first) (possible values: uniform_cost, greedy, best_first)                                                    |
| -w    | --weight      | FLOATING_NUMBER | The weight applied to the distance when computing the score (best first search type only), the cost has always a weight of 1. Leave this parameter unset to use a dynamic weight |
