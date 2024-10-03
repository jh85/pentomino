mod pieces;
mod dancinglinks;
mod board;
mod backtracking;
mod solutionset;
mod polyominolist;
mod polycubelist;

use std::time::Instant;
use pieces::piece::*;
use dancinglinks::dlx::*;
use board::board::*;
use solutionset::solutionset::*;
use backtracking::backtracking::solve_polyomino_bt;
use polyominolist::polyominolist::*;
use polycubelist::polycubelist::*;

fn have_common_position(positions_a: &[usize], positions_b: &[usize]) -> bool {
    positions_b.iter().any(|pos| positions_a.contains(pos))
}

fn pieces2positions(board: &Vec<Vec<usize>>, n: usize) -> (Vec<Vec<bool>>,Vec<usize>) {
    let num_pieces: usize = get_num_pieces(n);
    let mut positions: Vec<Vec<usize>> = Vec::new();
    let mut kinds: Vec<usize> = Vec::new();

    let board_h: usize = board.len();
    let board_w: usize = board[0].len();
    let mut hole_positions = Vec::new();
    for (i,row) in board.iter().enumerate() {
        for (j,value) in row.iter().enumerate() {
            if *value == 1 {
                hole_positions.push(i*board_w+j);
            }
        }
    }
    for (k,congruent_figures) in congruent_figures_for_each_piece(n).iter().enumerate() {
        for figure in congruent_figures {
            let (figure_h,figure_w) = figure.iter().fold((usize::MIN,usize::MIN),
                                                         |(max_i,max_j),&(i,j)| (max_i.max(i),max_j.max(j)));
            for offset_i in 0..board_h.saturating_sub(figure_h) {
                for offset_j in 0..board_w.saturating_sub(figure_w) {
                    let mut figure_positions: Vec<_> = figure.into_iter()
                        .map(|(i,j)| (i+offset_i)*board_w + (j+offset_j))
                        .collect();
                    if have_common_position(&figure_positions, &hole_positions) {
                        continue;
                    } else {
                        figure_positions.push(k + board_h*board_w);
                        positions.push(figure_positions);
                        kinds.push(k);
                    }
                }
            }
        }
    }

    hole_positions.push(board_h*board_w + num_pieces);
    positions.push(hole_positions);
    kinds.push(num_pieces);

    let length: usize = board_h*board_w + num_pieces + 1;
    let onehot_vectors: Vec<Vec<bool>> = positions.iter().map(|inner_vec| {
        let mut bool_vec = vec![false; length];
        inner_vec.iter().for_each(|&index| bool_vec[index] = true);
        bool_vec
    }).collect();
    (onehot_vectors,kinds)
}

fn solution2board(solution: &Vec<usize>,
                  kinds: &Vec<usize>,
                  positions: &Vec<Vec<bool>>,
                  board: &Vec<Vec<usize>>) -> Board {
    let mut ret: Board = Board::new(board.len(), board[0].len());
    for &k in solution.iter() {
        let kind_value = kinds[k];
        let position_bit_pattern = &positions[k];

        for i in 0..ret.height() {
            for j in 0..ret.width() {
                if position_bit_pattern[i*ret.width() + j] {
                    *ret.get_mut(i,j) = kind_value;
                }
            }
        }
    }
    ret
}

fn solve_polyomino_dlx(board: &Vec<Vec<usize>>, n: usize) -> Vec<Board> {
    let num_pieces: usize = get_num_pieces(n);
    let (positions,kinds) = pieces2positions(&board, n);
    let num_cells = board.iter().map(|row| row.len()).sum::<usize>();
    let mut m = Matrix::new(num_cells + num_pieces + 1);
    for pos_1hvec in &positions {
        m.add_row(&pos_1hvec);
    }

    let mut solutions = SolutionSet::new();
    let num_solutions: usize = if n >= 6 { 1 } else { 0 };
    for solution in solve(m, num_solutions).iter() {
        let solved_board = solution2board(&solution, &kinds, &positions, &board);
        solutions.add_solution(solved_board);
    }
    solutions.get_solutions()
}

#[allow(dead_code)]
fn test_board(n: usize) -> Vec<Vec<usize>> {
    let num_pieces = get_num_pieces(n);
    let height: usize = match n {
        4 => 5, // 5 = 1x5
        5 => 4, // 12 = 3x4
        6 => 5, // 35 = 5x7
        7 => 12, // 108 = 9x12
        8 => 41, // 369 = 9x41
        9 => 257, // 1285 = 5x257
        10 => 95, // 4655 = 49x95
        11 => 271, // 17073 = 63x271
        _ => panic!("not supported number"),
    };        
    let width: usize = num_pieces / height;
    
    let mut b = vec![vec![0;n*width];n*height];
    let polyominoes = free_polyominos(n);
    for (k,p) in polyominoes.iter().enumerate() {
        let i = k / width;
        let j = k % width;
        for ii in 0..n {
            for jj in 0..n {
                b[i*n + ii][j*n + jj] = if p[ii][jj] == 1 { 0 } else { 1 };
            }
        }
    }
    b
}

fn main() {
    let b5 = test_board(5);
    let b6 = test_board(6);
    let b7 = test_board(7);
    let b8 = test_board(8);
    let problems = vec![&b5, &b6, &b7, &b8];
    let sizes: Vec<usize> = vec![5,6,7,8,9,10];

    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_dlx(&problems[i], sizes[i]);
        let duration = start.elapsed();
        println!("DancingLinks: n={} solution={} time={:?}", sizes[i], solutions.len(), duration);
    }

    // 7x3 - 1
    let board_401 = vec![
        vec![0,0,0,0,0,0,0],
        vec![0,0,1,0,0,0,0],
        vec![0,0,0,0,0,0,0],
    ];
    let solsz_401: usize = 4;

    // 7x3 - 1
    let board_402 = vec![
        vec![0,0,0,0,0,1,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
    ];
    let solsz_402: usize = 3;

    // 7x3 - 1
    let board_403 = vec![
        vec![0,0,0,0,0,0,0],
        vec![1,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
    ];
    let solsz_403: usize = 4;

    // 7x3 - 1
    let board_404 = vec![
        vec![0,0,0,1,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
    ];
    let solsz_404: usize = 2;

    // 9x3 - 7
    let board_405 = vec![
        vec![1,1,0,0,0,0,0,0,1],
        vec![0,0,0,0,0,0,0,0,0],
        vec![1,1,0,0,1,0,0,0,1],
    ];
    let solsz_405: usize = 1;
    
    let problems = vec![board_401,board_402,board_403,board_404,board_405];
    let solsizes = vec![solsz_401,solsz_402,solsz_403,solsz_404,solsz_405];

    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_dlx(&problems[i], 4);
        let duration = start.elapsed();
//        solutions[0].pprint();
        println!("DancingLinks: solution={} correct={} time={:?}", solutions.len(), solsizes[i], duration);
    }

    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_bt(&problems[i], 4);
        let duration = start.elapsed();
//        solutions[0].pprint();
        println!("Backtracking: solution={} correct={} time={:?}", solutions.len(), solsizes[i], duration);
    }
    
    // 8x8 - 4(center)
    let board_501 = vec![
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,1,1,0,0,0],
        vec![0,0,0,1,1,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
    ];
    let solsz_501: usize = 65;

    // 5x12
    let board_502 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let solsz_502: usize = 1010;

    // 6x10
    let board_503 = vec![
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
    ];
    let solsz_503: usize = 2339;

    // 3x21b
    let board_504 = vec![
        vec![0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],
    ];
    let solsz_504: usize = 3;
    
    // 3x20
    let board_505 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let solsz_505: usize = 2;

    // 7x9 - 3
    let board_506 = vec![
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,1,0,0,1,0,0,1,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
    ];
    let solsz_506: usize = 143;

    // 10x10 - 40
    let board_507 = vec![
        vec![1,1,1,1,1,1,1,1,1,1],
        vec![1,1,0,0,0,0,0,0,1,1],
        vec![1,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,1],
        vec![0,0,0,0,1,1,0,0,0,0],
        vec![0,0,0,0,1,1,0,0,0,0],
        vec![1,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,1],
        vec![1,1,0,0,0,0,0,0,1,1],
        vec![1,1,1,1,1,1,1,1,1,1],
    ];
    let solsz_507: usize = 42;

    let board_508 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let solsz_508: usize = 368;

    let problems = vec![board_501,board_502,board_503,board_504,board_505,board_506,board_507,board_508];
    let solsizes = vec![solsz_501,solsz_502,solsz_503,solsz_504,solsz_505,solsz_506,solsz_507,solsz_508];

    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_dlx(&problems[i], 5);
        let duration = start.elapsed();
//        solutions[0].pprint();        
        println!("DancingLinks: solution={} correct={} time={:?}", solutions.len(), solsizes[i], duration);
    }

    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_bt(&problems[i], 5);
        let duration = start.elapsed();
//        solutions[0].pprint();
        println!("Backtracking: solution={} correct={} time={:?}", solutions.len(), solsizes[i], duration);
    }

    let board_601 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,1,1,1,1,1,0,0,0,0,0],
        vec![0,0,0,0,0,1,1,1,1,1,0,0,0,0,0],
        vec![0,0,0,0,0,1,1,1,1,1,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    let board_602 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    let board_603 = vec![
        vec![0,0,0,0,0],

        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],

        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],

        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],

        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],

        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
        vec![0,0,0,0,0],
        vec![0,0,1,0,0],
        vec![0,0,0,0,0],
    ];

    #[allow(unused_variables)]
    let board_604 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    let board_605 = vec![
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],

        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        vec![0,0,1,1,1,0,0],
        
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0],
    ];        

    let board_606 = vec![
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],

        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        
        vec![0,0,0,0, 1,1,1, 1,1,1, 1,1,1, 0,0,0,0],
        vec![0,0,0,0, 1,1,1, 1,1,1, 1,1,1, 0,0,0,0],
        vec![0,0,0,0, 1,1,1, 1,1,1, 1,1,1, 0,0,0,0],
        
        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 1,1,1, 0,0,0, 0,0,0,0],
        
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],
        vec![0,0,0,0, 0,0,0, 0,0,0, 0,0,0, 0,0,0,0],
    ];

    let board_607 = vec![
        vec![1,1,1,1,1, 1,1,1,1,0, 1,1,1,1,1, 1,1,1,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0],
    ];

    let board_608 = vec![
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,1],
    ];

    // board_602 and board_604 take too long
    let problems = vec![&board_601,&board_603,&board_605,&board_606,&board_607,&board_608];
    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_bt(&problems[i], 6);
        let duration = start.elapsed();
        solutions[0].pprint();
        println!("Backtracking: solution={} time={:?}", solutions.len(), duration);
    }
    
    let problems = vec![&board_602,&board_603,&board_605,&board_606,&board_607,&board_608];
    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_dlx(&problems[i], 6);
        let duration = start.elapsed();
        solutions[0].pprint();
        println!("DancingLinks: solution={} time={:?}", solutions.len(), duration);
    }

    let board_701 = vec![
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],

        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,1,0,1,1,0,1,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,1,1,1,1,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,1,1,1,1,1,1,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,1,1,1,1,1,1,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,1,1,1,1,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,1,0,1,1,0,1,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],

        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
        vec![0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0, 0,0,0,0,0],
    ];

    let problems = vec![&board_701];
    for i in 0..problems.len() {
        let start = Instant::now();
        let solutions = solve_polyomino_dlx(&problems[i], 7);
        let duration = start.elapsed();
        solutions[0].pprint();
        println!("DancingLinks: solution={} time={:?}", solutions.len(), duration);
    }
}
