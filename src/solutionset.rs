pub mod solutionset {
    use std::collections::HashSet;
    use crate::board::board::*;
    
    pub struct SolutionSet {
        solutions: Vec<Board>,
        congruent_solutions: HashSet<Board>,
    }

    impl SolutionSet {
        pub fn new() -> Self {
            SolutionSet {
                solutions: Vec::new(),
                congruent_solutions: HashSet::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.solutions.len()
        }

        pub fn add_solution(&mut self, board: Board) {
            if !self.congruent_solutions.contains(&board) {
                self.solutions.push(board.clone());
                let diagonal_opt = if board.is_square() {
                    vec![true,false]
                } else {
                    vec![false]
                };
                for vertically in [true,false] {
                    for horizontally in [true,false] {
                        for &diagonally in &diagonal_opt {
                            let board = board.transform(vertically,horizontally,diagonally);
                            self.congruent_solutions.insert(board.clone());
                        }
                    }
                }
            }
        }

        pub fn get_solutions(&self) -> Vec<Board> {
            self.solutions.clone()
        }
    }
}
