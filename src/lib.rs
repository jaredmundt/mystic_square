use std::fmt::Formatter;


pub struct Board {
    heuristic: i32,
    // slots: Vec<Slot>,
    
}

enum Slot {
    Tile(i32),
    Empty
}

impl Board {
    pub fn new() -> Board {
        let heuristic = 0;

        Board {heuristic}
    }

}

impl std::fmt::Display for Board {
    fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
