use std::io::{stdin,BufRead,Error};
use std::collections::HashMap;

type Disjoint_Loops = Vec<&HashMap<(u32,u32),HashSet<Direction>>>; 

use std::collections::HashMap;
use lazy_static::lazy_static;

enum Direction {
    N,
    S,
    E,
    W,
}

// Map of which directions connect to each other
lazy_static! {
    static ref DIRECTIONS_MATCH: HashMap<Direction, Direction> = {
    let mut map = HashMap::new();
        map.insert(Direction::N, Direction::S);
        map.insert(Direction::S, Direction::N);
        map.insert(Direction::E, Direction::W);
        map.insert(Direction::W, Direction::E);
    };
}

// Return whether a desired merged direction from a new cell 
// matches the merge directions of an existing cell
fn directions_compat (
  pair: (Direction,Direction),
  new_dir: Direction,
) -> bool {
  for dir in pair {
      if DIRECTIONS_MATCH.get(dir) == new_dir {
          true
      }
  }
  false
}

fn merge_start(
    disjoints: Disjoint_Loops,
    start_coord: (u32,u32),
    ) -> Result<u32,Error> {
    // Cycle through all the possibilities
    let mut neighbors_who_want_to_merge =  
    // Cell to the North wants to merge S 

}

// We need a disjoint set data structure
// the elements are coordinates (x,y) starting at (0,0) in the upper-left
// two elements are equivalent if they are connected

// We can fake it with a Vec of HashMaps,
// with the equivalence classes named by the index in the Vec 

// What operations do we want for this disjoint set?
// Extending / adding a new tuple to an existing set, keep a length with each disjoint 
// starting with any disjoint set, measuring its length

// Insert a new coord which merges two loops together
// Return value:
//   Some(usize)
//   Error() it's not an error to merge to a coord that doesn't exist yet, 
//      but it *is* an error to insert a new coord with incompatible merge directions 
//      to an existing insertion. later insertions always overwrite previous insertions, 
//      and incompatible merge directions will just start a new loop.
fn merge_into_disjoints(
    disjoints: Disjoint_Loops,
    new_coord: (u32,u32),
    new_merge_directions: (Direction,Direction),
    existing_coord1: (u32, u32), 
    existing_coord2: (u32, u32), 
) -> Result<u32,Error> {
    let mut found1: bool = false;
    let mut found2: bool = false;
    let mut index1: usize = 0;
    let mut index2: usize = 0;
    for (i, map) in disjoints.iter().enumerate() {
      if map.contains(existing_coord1) {
          found1 = true;
          index1 = i;
      }
      if map.contains(existing_coord2) {
          found2 = true;
          index2 = i;
      }
   } 
    if !found1 {
      Err("{existing_coord1} not found in any map.")
    }
    if !found2 {
      Err("{existing_coord2} not found in any map.")
    }
    // At this point 
    if index1 != index2 {
        // we are potentially closing a loop, if the merge directions match
        // we merge the two maps 
        // the new coord is the missing link 
        // we return the sum of the two previous existing loops
        let disjoint1 = disjoints.get(index1);
        let disjoint2 = disjoints.get(index2);
        for (key, val) in disjoint2.iter() {
            disjoint1.insert(key, val);
        }
        let new_len = disjoint1.len() + disjoint2.len();
        disjoints.remove(disjoint2);
        Some(new_len)
    } else {
        // index1 == index2
        // we already know these loops are the same, just increasing the loop length by one 
        // for the new_coord
        Some(disjoints.get(index1).len() + 1)
    }
}

// Insert a new coord which only extends an existing loop one way
fn insert_into_disjoints(
    disjoints: &Vec<&HashSet<(u32,u32)>>,
    new_coord: (u32,u32),
    existing_coord: (u32, u32), 
) -> Result<u32,Error> {
   for (i, map) in disjoints.iter().enumerate() {
     if map.contains(existing_coord) {
         map.insert(new_coord);
         return Some(map.len());
     }
   } 
   Err("{existing_coord} not found in any map.")
}

fn get_half_loop_length(
    disjoints: &Vec<&HashSet<(u32,u32)>>,
    existing_coord: (u32, u32), 
) -> Result<usize,Error> {
   for (i, map) in disjoints.iter().enumerate() {
     if map.contains(existing_coord) {
         return Some(map.len() / 2);
     }
   } 
   Err("{existing_coord} not found in any map.")
}

fn main() {

    let mut disjoint_sets : Vec<&HashMap<(u32,u32),HashSet<Direction>>> = Vec::new();

    let input = stdin().lock();
    let lines = input.lines();

    let mut sum : i32 = 0;
    let mut j : u32 = 0;
    let mut start = (0,0); // initially invalid 
    let mut start_found : bool = false;

    for line in lines {
       
        let line_str = line.unwrap().to_string();

        for (i,c) in line_str.chars().enumerate() {
            let result_len = match c {
                '|' => merge_into_disjoints(disjoints, (i,j), (i,j-1), (i,j+1)),
                '-' => merge_into_disjoints(disjoints, (i,j), (i-1,j), (i+1,j)),
                'L' => merge_into_disjoints(disjoints, (i,j), (i,j-1), (i+1,j)),
                'J' => merge_into_disjoints(disjoints, (i,j), (i,j-1), (i-1,j)),
                '7' => merge_into_disjoints(disjoints, (i,j), (i-1,j), (i,j+1)),
                'F' => merge_into_disjoints(disjoints, (i,j), (i+1,j), (i,j+1)),
                'S' => merge_start(disjoints, (i,j)),
                    // when we find S, we want to infer which two adjacent
                       // cells want to merge with us consistently
                       // (what if there are more than two?)
                       // I think the puzzle promises us there will only be two 
                       // There are 6 possibilities
            }
        }
        
        j += 1;
    }

}
