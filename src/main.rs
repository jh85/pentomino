mod pieces;
mod dancinglinks;
mod board;
mod backtracking;
mod solutionset;
mod polyominolist;
mod polycubelist;
mod cube;
mod testset;

use std::time::Instant;
use pieces::piece::*;
use dancinglinks::dlx::*;
use board::board::*;
use solutionset::solutionset::*;
use backtracking::backtracking::solve_polyomino_bt;
use polyominolist::polyominolist::*;
use cube::cube::*;
use testset::testset::*;

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

fn pieces2positions3d(cube: &Vec<Vec<Vec<usize>>>, n: usize) -> (Vec<Vec<bool>>,Vec<usize>) {
    let dim0 = cube.len();
    let dim1 = cube[0].len();
    let dim2 = cube[0][0].len();

    let num_pieces: usize = get_num_pieces_3d(n);
    let mut positions: Vec<Vec<usize>> = Vec::new();
    let mut kinds: Vec<usize> = Vec::new();
    let mut hole_positions = Vec::new();
    for i in 0..dim0 {
        for j in 0..dim1 {
            for k in 0..dim2 {
                if cube[i][j][k] == 1 {
                    hole_positions.push(i*dim1*dim2 + j*dim2 + k);
                }
            }
        }
    }
    for (kind,congruent_figures) in congruent_figures_for_each_piece_3d(n).iter().enumerate() {
        for figure in congruent_figures {
            let (figure_dim0,figure_dim1,figure_dim2) = figure.iter()
                .fold((usize::MIN,usize::MIN,usize::MIN),
                      |(max_i,max_j,max_k),&(i,j,k)| (max_i.max(i),max_j.max(j),max_k.max(k)));
            for offset_i in 0..dim0.saturating_sub(figure_dim0) {
                for offset_j in 0..dim1.saturating_sub(figure_dim1) {
                    for offset_k in 0..dim2.saturating_sub(figure_dim2) {
                        let mut figure_positions: Vec<_> = figure.into_iter()
                            .map(|(i,j,k)| (i+offset_i)*dim1*dim2 + (j+offset_j)*dim2 + (k+offset_k))
                            .collect();
                        if have_common_position(&figure_positions, &hole_positions) {
                            continue;
                        } else {
                            figure_positions.push(kind + dim0*dim1*dim2);
                            positions.push(figure_positions);
                            kinds.push(kind);
                        }
                    }
                }
            }
        }
    }
    hole_positions.push(dim0*dim1*dim2 + num_pieces);
    positions.push(hole_positions);
    kinds.push(num_pieces);

    let length: usize = dim0*dim1*dim2 + num_pieces + 1;
    let onehot_vectors: Vec<Vec<bool>> = positions.iter().map(|inner_vec| {
        let mut bool_vec = vec![false; length];
        inner_vec.iter().for_each(|&index| bool_vec[index] = true);
        bool_vec
    }).collect();
    (onehot_vectors,kinds)
}

fn solution2cube(solution: &Vec<usize>,
                 kinds: &Vec<usize>,
                 positions: &Vec<Vec<bool>>,
                 cube: &Vec<Vec<Vec<usize>>>) -> Cube {
    let dim0 = cube.len();
    let dim1 = cube[0].len();
    let dim2 = cube[0][0].len();
    let mut ret: Cube = Cube::new(dim0,dim1,dim2);
    for &k in solution.iter() {
        let kind_value = kinds[k];
        let position_bit_pattern = &positions[k];

        for i in 0..dim0 {
            for j in 0..dim1 {
                for k in 0..dim2 {
                    if position_bit_pattern[i*dim1*dim2 + j*dim2 + k] {
                        *ret.get_mut(i,j,k) = kind_value;
                    }
                }
            }
        }
    }
    ret
}

fn conv2num(v: Vec<bool>) -> Vec<usize> {
    v.into_iter()
        .map(|value| if value {1} else {0})
        .collect()
}

fn solve_polycube_dlx(cube: &Vec<Vec<Vec<usize>>>, n: usize) -> Vec<Cube> {
    let num_pieces: usize = get_num_pieces_3d(n);
    let (positions,kinds) = pieces2positions3d(&cube, n);
    let num_cells = cube.len() * cube[0].len() * cube[0][0].len();
    let mut m = Matrix::new(num_cells + num_pieces + 1);
    for pos_1hvec in &positions {
        m.add_row(&pos_1hvec);
    }

    let mut solutions = SolutionSet::new();
    let num_solutions: usize = 0;
    for solution in solve(m, num_solutions).iter() {
        let solved_cube = solution2cube(&solution, &kinds, &positions, &cube);
        solutions.add_solution(solved_cube);
    }
    solutions.get_solutions()
}

fn main() {
    let start = Instant::now();
    let solutions = solve_polycube_dlx(&test_cube("404"), 5);
    let duration = start.elapsed();
    println!("DancingLinks: problem=B # of solutions={} elapsed time={:?}", solutions.len(), duration);

    return;
    
    let start = Instant::now();
    let solutions = solve_polycube_dlx(&test_cube("401"), 4);
    let duration = start.elapsed();
    println!("DancingLinks: problem=CUBE_401 # of solutions={} elapsed time={:?}", solutions.len(), duration);

    let start = Instant::now();
    let solutions = solve_polyomino_bt(&test_board("501"), 5);
    let duration = start.elapsed();
    println!("Backtracking: problem=BOARD_501 # of solutions={} elapsed time={:?}", solutions.len(), duration);

    let start = Instant::now();
    let solutions = solve_polyomino_dlx(&test_board("401"), 4);
    let duration = start.elapsed();
    println!("DancingLinks: problem=BOARD_601 # of solutions={} elapsed time={:?}", solutions.len(), duration);

}
