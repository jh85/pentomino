use std::{
    array,
    usize,
    collections::HashSet,
    time::Instant,
};

const T: bool = true;
const F: bool = false;
type Bitmap = u128;
type Solution = [Bitmap; Piece::NUM_PIECES];

#[derive(Copy,Clone)]
struct Piece([[bool;5];5]);

impl Default for Piece {
    fn default() -> Self {
        Piece([[false;5];5])
    }
}

impl Piece {
    const NUM_PIECES: usize = 12;

    fn flip(&self) -> Self {
        let mut ret = Piece::default();
        for (y,row) in self.0.iter().enumerate() {
            for (x,col) in row.iter().enumerate() {
                ret.0[y][4-x] = *col;
            }
        }
        ret
    }

    fn rot90(&self) -> Self {
        let mut ret = Piece::default();
        for (y,row) in self.0.iter().enumerate() {
            for (x,col) in row.iter().enumerate() {
                ret.0[x][4-y] = *col;
            }
        }
        ret
    }

    fn transform(&self, is_flip: bool, n_rotations: usize) -> Self {
        let mut ret = if is_flip {
            self.flip()
        } else {
            *self
        };

        for _ in 0..n_rotations {
            ret = ret.rot90();
        }
        ret
    }

    fn true_locations(&self) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        for (y,row) in self.0.iter().enumerate() {
            for (x,col) in row.iter().enumerate() {
                if *col == true {
                    ret.push((x,y));
                }
            }
        }
        let (xmin,ymin) = ret.iter().fold((usize::MAX,usize::MAX),
                                          |(xmin,ymin),&(x,y)| (xmin.min(x),ymin.min(y)));
        ret.iter_mut().for_each(|(x,y)| {
            *x -= xmin;
            *y -= ymin;
        });
        ret
    }

    const PIECES: [Piece; Piece::NUM_PIECES] = [
        Piece([[T,F,F,F,F],[T,F,F,F,F],[T,F,F,F,F],[T,F,F,F,F],[T,F,F,F,F]]), // I
        Piece([[T,T,F,F,F],[T,T,F,F,F],[T,F,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // P
        Piece([[T,T,F,F,F],[F,T,F,F,F],[F,T,F,F,F],[F,T,F,F,F],[F,F,F,F,F]]), // L
        Piece([[F,T,T,F,F],[T,T,F,F,F],[F,T,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // F
        Piece([[F,F,T,T,F],[T,T,T,F,F],[F,F,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // N
        Piece([[T,T,T,F,F],[F,T,F,F,F],[F,T,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // T
        Piece([[T,F,T,F,F],[T,T,T,F,F],[F,F,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // U
        Piece([[T,F,F,F,F],[T,F,F,F,F],[T,T,T,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // V
        Piece([[T,F,F,F,F],[T,T,F,F,F],[F,T,T,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // W
        Piece([[F,T,F,F,F],[T,T,T,F,F],[F,T,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // X
        Piece([[F,F,T,F,F],[T,T,T,T,F],[F,F,F,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // Y
        Piece([[T,T,F,F,F],[F,T,F,F,F],[F,T,T,F,F],[F,F,F,F,F],[F,F,F,F,F]]), // Z
    ];

    fn variations_for_each_piece() -> Vec<Vec<Vec<(usize,usize)>>> {
        let mut ret = Vec::new();
        for piece in Self::PIECES {
            let mut variations = Vec::new();
            for f in [true,false] {
                for r in [0,1,2,3] {
                    let p = piece.transform(f,r).true_locations();
                    if !variations.contains(&p) {
                        variations.push(p);
                    }
                }
            }
            ret.push(variations);
        }
        ret
    }
}

#[derive(Clone,PartialEq,Eq,Hash)]
enum Piecename {
    I = 0,
    P,
    L,
    F,
    N,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

impl Piecename {
    fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Piecename::I),
            1 => Some(Piecename::P),
            2 => Some(Piecename::L),
            3 => Some(Piecename::F),
            4 => Some(Piecename::N),
            5 => Some(Piecename::T),
            6 => Some(Piecename::U),
            7 => Some(Piecename::V),
            8 => Some(Piecename::W),
            9 => Some(Piecename::X),
            10 => Some(Piecename::Y),
            11 => Some(Piecename::Z),
            _ => None,
        }
    }
}

struct Solver {
    rows: usize,
    cols: usize,
    table: [[Vec<Bitmap>; Piece::NUM_PIECES]; 128],
}

impl Solver {
    fn new(rows: usize, cols: usize) -> Self {
        let mut table = array::from_fn(|_| array::from_fn(|_| Vec::new()));
        for (k,kth_pieces) in Piece::variations_for_each_piece().iter().enumerate() {
            for variation in kth_pieces {
                if variation.iter().any(|&(x,y)| x >= cols || y >= rows) {
                    continue;
                }
                let bitmap = variation.iter().map(|&(x,y)| (1u128 << (x+y*cols))).sum::<u128>();
                let (xlen,ylen) = variation.iter().fold((usize::MIN,usize::MIN),
                                                        |(xmax,ymax), &(x,y)| (xmax.max(x),ymax.max(y)));
                for y in 0..rows - ylen {
                    for x in 0..cols - xlen {
                        let offset = x+y*cols;
                        let top_left_true = variation[0].0 + offset;
                        table[top_left_true][k].push(bitmap << offset);
                    }
                }
            }
        }
        Self {rows,cols,table}
    }

    fn backtrack(&self, bitmap: Bitmap, solution: &mut Solution, solutions: &mut SolutionSet) {
        if solution.iter().all(|p| *p != 0) {
            let b = self.solution2board(solution);
            solutions.add_solution(b, solution);
            return;
        }
        let first_false_bit = bitmap.trailing_ones() as usize;
        for i in 0..Piece::NUM_PIECES {
            if solution[i] == 0 {
                for &p in self.table[first_false_bit][i].iter() {
                    if bitmap & p == 0 {
                        solution[i] = p;
                        self.backtrack(bitmap|p, solution, solutions);
                        solution[i] = Bitmap::default();
                    }
                }
            }
        }
    }

    fn solve(&self, initial_bits: Bitmap) -> Vec<Solution> {
        let mut solution = [Bitmap::default(); Piece::NUM_PIECES];
        let mut solutions = SolutionSet::new();
        self.backtrack(initial_bits.clone(), &mut solution, &mut solutions);
        solutions.get_solutions()
    }

    fn solution2board(&self, solution: &Solution) -> Board {
        let mut ret = vec![vec![None;self.cols];self.rows];
        for (i,b) in solution.iter().enumerate() {
            let p = Piecename::from_usize(i);
            for (y,row) in ret.iter_mut().enumerate() {
                for (x,col) in row.iter_mut().enumerate() {
                    if b & (1 << (x+y*self.cols)) != 0 {
                        *col = p.clone();
                    }
                }
            }
        }
        Board(ret)
    }
}

#[derive(Clone,PartialEq,Eq,Hash)]
struct Board(Vec<Vec<Option<Piecename>>>);

impl Board {
    fn is_square(&self) -> bool {
        self.0.len() == self.0[0].len()
    }

    fn flip_h(&self) -> Self {
        let mut ret = self.clone();            
        for (y,row) in self.0.iter().enumerate() {
            for (x,col) in row.iter().enumerate() {
                ret.0[y][row.len()-1-x] = col.clone();
            }
        }
        ret
    }

    fn flip_v(&self) -> Self {
        let mut ret = self.clone();
        ret.0.reverse();
        ret
    }

    fn transpose(&self) -> Self {
        let mut ret = self.clone();
        for (y,row) in self.0.iter().enumerate() {
            for (x,col) in row.iter().enumerate() {
                ret.0[x][y] = col.clone();
            }
        }
        ret
    }

    fn transform(&self, is_flip_h: bool, is_flip_v: bool, is_transpose: bool) -> Self {
        let mut ret = if is_flip_h {
            self.flip_h()
        } else {
            self.clone()
        };
        if is_flip_v {
            ret = ret.flip_v();
        }
        if is_transpose {
            ret = ret.transpose();
        }
        ret
    }
}

struct SolutionSet {
    solutions: Vec<Solution>,
    set: HashSet<Board>,
}

impl SolutionSet {
    fn new() -> Self {
        SolutionSet {
            solutions: Vec::new(),
            set: HashSet::new(),
        }
    }

    fn add_solution(&mut self, board: Board, solution: &Solution) {
        if !self.set.contains(&board) {
            self.solutions.push(*solution);
            let transpose_opt = if board.is_square() {
                vec![true,false]
            } else {
                vec![false]
            };
            for is_flip_h in [true,false] {
                for is_flip_v in [true,false] {
                    for is_transpose in &transpose_opt {
                        let b = board.transform(is_flip_h, is_flip_v, *is_transpose);
                        self.set.insert(b);
                    }
                }
            }
        }
    }

    fn get_solutions(&self) -> Vec<Solution> {
        self.solutions.clone()
    }
}

fn board2bitmap(board: &Vec<Vec<bool>>) -> Bitmap {
    let cols = board[0].len();
    let mut true_locations = Vec::new();
    for (y,row) in board.iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            if *col {
                true_locations.push((x,y));
            }
        }
    }
    let v = true_locations.iter().map(|&(x,y)| 1u128 << (x+y*cols)).sum::<u128>();
    v
}

fn solve_pentomino(board: &Vec<Vec<bool>>) -> usize {
    let solver = Solver::new(board.len(), board[0].len());
    let s = solver.solve(board2bitmap(&board));
    s.len()
}

fn main() {
    // 8x8 - 4(center)
    let board1 = vec![
        vec![F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F],
        vec![F,F,F,T,T,F,F,F],
        vec![F,F,F,T,T,F,F,F],
        vec![F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F],
    ];
    let n_sol1: usize = 65;

    // 5x12
    let board2 = vec![
        vec![F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F],
    ];
    let n_sol2: usize = 1010;

    // 6x10
    let board3 = vec![
        vec![F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F],
    ];
    let n_sol3: usize = 2339;

    // 3x21b
    let board4 = vec![
        vec![F,F,F,F,F,F,F,F,T,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,T,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,T,F,F,F,F,F,F,F,F],
    ];
    let n_sol4: usize = 3;
    
    // 3x20
    let board5 = vec![
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
    ];
    let n_sol5: usize = 2;

    // 7x9 - 3
    let board6 = vec![
        vec![F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F],
        vec![F,T,F,F,T,F,F,T,F],
        vec![F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F],
    ];
    let n_sol6: usize = 143;

    // 10x10 - 40
    let board7 = vec![
        vec![T,T,T,T,T,T,T,T,T,T],
        vec![T,T,F,F,F,F,F,F,T,T],
        vec![T,F,F,F,F,F,F,F,F,T],
        vec![T,F,F,F,F,F,F,F,F,T],
        vec![F,F,F,F,T,T,F,F,F,F],
        vec![F,F,F,F,T,T,F,F,F,F],
        vec![T,F,F,F,F,F,F,F,F,T],
        vec![T,F,F,F,F,F,F,F,F,T],
        vec![T,T,F,F,F,F,F,F,T,T],
        vec![T,T,T,T,T,T,T,T,T,T],
    ];
    let n_sol7: usize = 65;

    let board8 = vec![
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
        vec![F,F,F,F,F,F,F,F,F,F,F,F,F,F,F],
    ];
    let n_sol8: usize = 368;
    
    let problems  = vec![board1,board2,board3,board4,board5,board6,board7,board8];
    let n_solutions = vec![n_sol1,n_sol2,n_sol3,n_sol4,n_sol5,n_sol6,n_sol7,n_sol8];

    for i in 0..problems.len() {
        let start = Instant::now();
        let ans = solve_pentomino(&problems[i]);
        let duration = start.elapsed();
        println!("n_solution={} correct_number={} time={:?}",
                 ans, n_solutions[i], duration);
    }
}
