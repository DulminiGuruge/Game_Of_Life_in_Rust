use std::fs::File;

use std::io::{BufRead, BufReader, Write};

pub struct Grid {
    //vector to hold the current generation
    pub current_arr: Vec<i32>,
    //vector to hold the calculated successor
     successor_arr: Vec<i32>,
    //size of the vector which stores grid data
     vector_size: usize,
    //column or the row length of he grid
     col_size: usize,
    //number of generations
     generations: usize,
}

impl Grid {
    //initialize the variables and the data structures
    pub unsafe fn new(vec_size: usize, generation_no: usize) -> Self {
        Grid {
            current_arr: (vec![0; vec_size]), //vector for current generation
            successor_arr: (vec![0; vec_size]), // vector for calculate its successor.
            vector_size: vec_size,
            col_size: f32::sqrt(vec_size as f32) as usize,
            generations: generation_no
        }
    }

    /*
        @desc - Read the input file and convert to a vector
    */
    pub fn read_from_file(&mut self) -> () {
        let file = File::open("input.txt")
            .expect("file not found!");
        let buf_reader = BufReader::new(file);
        let mut index: usize = 0;

        for line in buf_reader.lines() {
            let a: String = line.unwrap();
            for token in a.split(" ") {
                if index != self.vector_size {
                    //Convert firing cells which are denoted by * to 1
                    if token.parse::<String>().unwrap() == "*" {
                        self.current_arr[index] = 1;
                    } //Convert the cells in refractory stage which are denoted by "." to 2
                    else if token.parse::<String>().unwrap() == "." {  
                        self.current_arr[index] = 2;
                    } else {
                        //if the cells are in quiet state convert " " to 0
                        self.current_arr[index] = 0;
                    }             
                    index = index + 1;
                }
            }

        }
       
        println!("data read from file");
    }

    /*
        @desc - update the grid according to the number of generations and the rules
                Rules
                - quiet cells with 2 firing neighbours start firing
                - firing cells enter the refractory state
                - refractory cells recover and become quiet
     */
    pub fn update_grid(&mut self) -> &Vec<i32> {

        //loop to create the number of generations needed
        for mut i in 1..self.generations {
          
            //loop through the vector
            for row in 0..self.col_size {
                for col in 0..self.col_size {

                    //firing neighbour count of the iteration
                    let mut neighbours = 0;
                    //logic to change the two vectors from one iteration to the other
                    let mut iteration:bool = (i % 2) == 0;

                    // let vec_index1: usize = (row * 100 + col) as usize;
                    let vec_index: usize = (row * 100 + col) as usize;

                    if iteration {
                        /*call the function to loop through each element and check for neighbouring
                         values and add neighbour count if there are firing neighbours*/
                        count_neighbours(row as i32, col as i32, &mut neighbours, &self.successor_arr);

                        // firing cells represented by 1 , enter the refractory state
                        if self.successor_arr[vec_index] == 1 {
                            self.current_arr[vec_index] = 2;

                        }  //refractory cells represented by 2, recover and become quiet
                        else if self.successor_arr[vec_index] == 2 {
                            self.current_arr[vec_index] = 0;
                        } else {
                            /*quiet cells represented by 0, with 2 firing neighbours represented 
                            by 1 start firing this returns bool as 1 if the above condition is satisfied*/
                            self.current_arr[vec_index] = i32::from(neighbours == 2);
                        }

                        
                    } 
                    else {

                        /*call the function to loop through each element and check for neighbouring
                        values and add neighbour count if there are firing neighbours*/
    
                        count_neighbours(row as i32, col as i32, &mut neighbours, &self.current_arr);

                        // firing cells represented by 1 , enter the refractory state
                        if self.current_arr[vec_index] == 1 {
                            self.successor_arr[vec_index] = 2;
                        //refractory cells represented by 2, recover and become quiet
                        } else if self.current_arr[vec_index] == 2 {
                            self.successor_arr[vec_index] = 0;
                        } else {
                            /*quiet cells represented by 0, with 2 firing neighbours represented 
                            by 1 start firing this returns bool as 1 if the above condition is satisfied */
                            self.successor_arr[vec_index] = i32::from(neighbours == 2);
                        }
                    }
                }
            }

        }
        return &self.current_arr;
    }

    /*
        @desc - write the generated vector as a grid to a file
        @param - pointer to the vector which is generated
    */
    pub fn write_to_file(&mut self) -> () {
        let mut file1 = std::fs::File::create("output.txt").expect("create failed");

        for vec_index in 0..self.vector_size
        {
            if self.current_arr[vec_index] == 1 {
                if vec_index > 0 && vec_index % self.col_size == 0 {
                    file1.write_all("\n".as_bytes()).expect("write failed");
                }
                file1.write_all("* ".as_bytes()).expect("write failed");
            } else if self.current_arr[vec_index] == 2 {
                if vec_index > 0 && vec_index % self.col_size == 0 {
                    file1.write_all("\n".as_bytes()).expect("write failed");
                }
                file1.write_all(". ".as_bytes()).expect("write failed");
            } else {
                if vec_index > 0 && vec_index % 100 == 0 {
                    file1.write_all("\n".as_bytes()).expect("write failed");
                }
                file1.write_all("  ".as_bytes()).expect("write failed");
            }
        }
        file1.write_all("\n".as_bytes()).expect("write failed");
        println!("data written to file");
    }
}


/*

    @desc - check for the status of the neighbours and add the number of firing 
            neighbours to the count
    @param - row, column, number of firing neighbours, current or successor array

*/
fn count_neighbours(mut row: i32, mut col: i32, neighbours: &mut usize, arr_choice: &Vec<i32>) {
    
    let constant_val: i32 = row * 100 + col;
    if row > 0
    {
        let north_west: usize = (constant_val - 101) as usize;//north west neighbour
        let north: usize = (constant_val - 100) as usize;//north neighbour
        let north_east: usize = (constant_val - 99) as usize;//north east neighbour
        //update the neighbour count
        if col > 0 { if arr_choice[north_west] == 1 { *neighbours += 1; } }
        if arr_choice[north] == 1 { *neighbours += 1; }
        if col < 99 { if arr_choice[north_east] == 1 { *neighbours += 1; } }
    }
    if col > 0 {
        let west: usize = (constant_val - 1) as usize; //west neighbour
        //update the neighbour count
        if arr_choice[west] == 1 { *neighbours += 1; }
    }
    if col < 99 {
        let east: usize = (constant_val + 1) as usize; //east neighbour
        //update the neighbour count
        if arr_choice[east] == 1 { *neighbours += 1; }
    }
    if row < 99
    {s
        let south_west: usize = (constant_val + 99) as usize; //south west neighbour
        let south: usize = (constant_val + 100) as usize;//south neighbour
        let south_east: usize = (constant_val + 101) as usize;//south east neighbour
        //update the neighbour counts
        if col > 0 { if arr_choice[south_west] == 1 { *neighbours += 1; } }
        if arr_choice[south] == 1 { *neighbours += 1; }
        if col < 99 { if arr_choice[south_east] == 1 { *neighbours += 1; } }
    }
}

