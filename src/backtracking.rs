pub mod backtracking {
    use ethnum::u256;
    use crate::pieces::piece::*;
    use crate::board::board::*;
    use crate::solutionset::solutionset::*;
    
    type Bitmap = u256;

    #[derive(Clone)]
    struct Solution(Vec<Bitmap>);

    impl Solution {
        fn new(num_pieces: usize) -> Self {
            Solution(vec![Bitmap::from(0u8); num_pieces])
        }
    }

    struct Solver {
        height: usize,
        width: usize,
        num_pieces: usize,
        num_solutions: usize,
        table: Vec<Vec<Vec<Bitmap>>>,
    }

    impl Solver {
        fn new(board_h: usize, board_w: usize, n: usize, num_solutions: usize) -> Self {
            let bitmap_size: usize = std::mem::size_of::<Bitmap>()*8;
            let num_pieces: usize = get_num_pieces(n);
            let mut table = (0..bitmap_size).map(|_| {
                (0..num_pieces).map(|_| Vec::<Bitmap>::new()).collect::<Vec<_>>()
            }).collect::<Vec<_>>();

            for (k,congruent_figures) in congruent_figures_for_each_piece(n).iter().enumerate() {
                for figure in congruent_figures {
                    let (figure_h,figure_w) = figure.iter().fold((usize::MIN,usize::MIN),
                                                                 |(max_i,max_j),&(i,j)| (max_i.max(i),max_j.max(j)));
                    let bitmap = figure.iter().map(|&(i,j)| Bitmap::from(1u8) << (i*board_w+j)).sum::<Bitmap>();
                    for offset_i in 0..board_h.saturating_sub(figure_h) {
                        for offset_j in 0..board_w.saturating_sub(figure_w) {
                            let offset = offset_i*board_w + offset_j;
                            let lowest_1 = figure[0].1 + offset;
                            table[lowest_1][k].push(bitmap << offset);
                        }
                    }
                }
            }
            Solver { height: board_h,
                     width: board_w,
                     num_pieces: num_pieces,
                     num_solutions: num_solutions,
                     table: table }
        }

        fn solution2board(&self, solution: &Solution) -> Board {
            let mut ret = Board::new(self.height, self.width);
            for (k,bitmap) in solution.0.iter().enumerate() {
                for (i,row) in ret.0.iter_mut().enumerate() {
                    for (j,col) in row.iter_mut().enumerate() {
                        if *bitmap & (Bitmap::from(1u8) << (i*self.width+j)) != Bitmap::from(0u8) {
                            *col = k;
                        }
                    }
                }
            }
            ret
        }

        fn backtrack(&self, bitmap: Bitmap, partial_solution: &mut Solution, solutions: &mut SolutionSet) {
            if self.num_solutions > 0 && solutions.len() >= self.num_solutions {
                return;
            }
            if partial_solution.0.iter().all(|p| *p != Bitmap::from(0u8)) {
                let solved_board = self.solution2board(partial_solution);
                solutions.add_solution(solved_board);
            } else {
                let lowest_0 = bitmap.trailing_ones() as usize;
                for i in 0..self.num_pieces {
                    if partial_solution.0[i] == Bitmap::from(0u8) {
                        for &p in self.table[lowest_0][i].iter() {
                            if bitmap & p == Bitmap::from(0u8) {
                                partial_solution.0[i] = p;
                                self.backtrack(bitmap|p, partial_solution, solutions);
                                partial_solution.0[i] = Bitmap::from(0u8);
                            }
                        }
                    }
                }
            }
        }

        fn solve(&self, initial_bits: Bitmap) -> Vec<Board> {
            let mut solution = Solution::new(self.num_pieces);
            let mut solutions = SolutionSet::new();
            self.backtrack(initial_bits.clone(), &mut solution, &mut solutions);
            solutions.get_solutions()
        }
    }

    fn board2bitmap(board: &Vec<Vec<usize>>) -> Bitmap {
        let width = board[0].len();
        let mut hole_coordinates = Vec::new();
        for (i,row) in board.iter().enumerate() {
            for (j,col) in row.iter().enumerate() {
                if *col != 0 {
                    hole_coordinates.push((i,j));
                }
            }
        }
        let b = hole_coordinates.iter().map(|&(i,j)| Bitmap::from(1u8) << (i*width+j)).sum::<Bitmap>();
        b
    }

    pub fn solve_polyomino_bt(board: &Vec<Vec<usize>>, n: usize) -> Vec<Board> {
        let num_solutions: usize = if n >= 6 { 1 } else { 0 };
        let solver = Solver::new(board.len(), board[0].len(), n, num_solutions);
        let solutions = solver.solve(board2bitmap(&board));
        solutions
    }
}
