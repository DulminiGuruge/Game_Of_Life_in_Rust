# Conway Game of Life in Rust programming

This document presents an implementation of Conway Game of Life in Rust, an open-source systems programming language known for its speed and memory safety. 

The Game of Life algorithm requires input values to be read from a file and the generated output to be written to a file. This implementation uses two vectors to hold the current generation and to calculate its successor. The input grid vector representation maps "0" to quiet cells, "*" to firing cells, and "." to refractory cells.

The rules of the algorithm specify that quiet cells with two firing neighbors start firing, firing cells enter the refractory state, and refractory cells recover and become quiet.

The method used in this implementation involves a nested for loop that considers each element of the current array in turn, counting the firing neighbors of each cell to decide whether the corresponding element of the successor array should be 0 or 1. The algorithm changes 1 to 2 and displays the successor array. For the next iteration, the arrays may swap roles so that the successor array in the last iteration becomes the current array in the next iteration.

This implementation offers an efficient and reliable solution for computing the Game of Life algorithm and could be of interest to a wide range of users in the field of systems programming.