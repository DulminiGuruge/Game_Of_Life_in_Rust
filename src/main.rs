/*
 Game of Life -
    Input values should be read from the file and the generated output should be written to the file.
    Two vectors are used:  to hold the current generation, and to calculate its successor.
    input grid   vector representation
        "0"       0 - quiet cells
        "*"       1 - firing cells
        "."       2 -refractory cells.

 Rules
    - quiet cells with 2 firing neighbours start firing
    - firing cells enter the refractory state
    - refractory cells recover and become quiet

 Method
    A nested for loop considers each element of the current array in turn, counting the firing
    neighbours of each cell to decide whether the corresponding element of the successor array
    should be 0 or 1. Change 1 to 2. The successor array is displayed. For the next iteration,
    the arrays may swap roles so that the successor array in the last iteration becomes the
    current array in the next iteration.

 */


 mod lib;
 use crate::lib::Grid;
 
 
 fn main() {
 
     //input the vector size and the count of the generations needed as the arguments
     let mut post = unsafe { Grid::new(10000, 7) };
 
     //read the grid and store it in a vector
     post.read_from_file();
     // Change the input vector according to the number of generations and the rules
     post.update_grid();
     //convert the vector to a grid and write to a file
     post.write_to_file();
 }
 
 //create test cases
 #[cfg(test)]
 mod tests {
     use crate::lib::Grid;
 
     #[test]
     fn read_from_file() {
         let mut post = unsafe { Grid::new(10000, 1) };
 
         //read the grid and store it in a vector
         post.read_from_file();
 
         post.update_grid();
         //convert the vector to a grid and write to a file
         post.write_to_file();
 
         let mut firing_cells_count=0;
         let mut refractory_cells=0;
         let mut dead_cells_count=0;
 
         //count the different cells in the output file
         for i in &post.current_arr {
 
             match i {
                 1=> firing_cells_count += 1,
                 2=> refractory_cells += 1,
                 _ => {}
             }
         }
         //Test whether the firing_cells_count and the refractory_cells match
         assert_eq!(firing_cells_count, 35);
         assert_eq!(refractory_cells, 7);
 
     }
 }
 