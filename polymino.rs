mod pieces;
mod dancinglinks;

use std::{
    collections::HashSet,
    time::Instant,
};
use pieces::piece;
use dancinglinks::dlx::*;

#[derive(Clone,Eq,PartialEq,Hash)]
struct Board(Vec<Vec<usize>>);

impl Board {
    fn new(h: usize, w: usize) -> Self {
        Board(vec![vec![0;w]; h])
    }
    
    fn height(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    fn get_mut(&mut self, i: usize, j: usize) -> &mut usize {
        &mut self.0[i][j]
    }
    
    fn flip_horizontally(&self) -> Self {
        let ret: Vec<Vec<usize>> = self.0
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect();
        Board(ret)
    }

    fn flip_vertically(&self) -> Self {
        let ret: Vec<Vec<usize>> = self.0
            .iter()
            .rev()
            .cloned()
            .collect();
        Board(ret)
    }

    fn flip_diagonally(&self) -> Self {
        let size = self.width();
        let mut ret = vec![vec![0;size]; size];
        for i in 0..size {
            for j in 0..size {
                ret[i][j] = self.0[j][i];
            }
        }
        Board(ret)
    }

    fn is_square(&self) -> bool {
        self.width() == self.height()
    }

    fn transform(&self, vertically: bool, horizontally: bool, diagonally: bool) -> Self {
        let mut ret = if vertically {
            self.flip_vertically()
        } else {
            self.clone()
        };

        ret = if horizontally {
            ret.flip_horizontally()
        } else {
            ret
        };

        ret = if diagonally {
            ret.flip_diagonally()
        } else {
            ret
        };
        ret
    }

    fn normalize_coordinates(&self) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        for (i,row) in self.0.iter().enumerate() {
            for (j,value) in row.iter().enumerate() {
                if *value > 0 {
                    ret.push((i,j));
                }
            }
        }
        let (min_i,min_j) = ret.iter().fold((usize::MAX,usize::MAX),
                                            |(min_i,min_j),&(i,j)| (min_i.min(i), min_j.min(j)));
        ret.iter_mut().for_each(|(i,j)| {
            *i -= min_i;
            *j -= min_j;
        });
        ret
    }
}

impl<const N: usize> From<[[usize;N];N]> for Board {
    fn from(array: [[usize;N];N]) -> Self {
        let vec = array.iter()
            .map(|row| row.to_vec())
            .collect::<Vec<Vec<usize>>>();
        Board(vec)
    }
}

fn congruent_figures_for_each_piece(n: usize) -> Vec<Vec<Vec<(usize,usize)>>> {
    let piece_list = if n == 5 {
        piece::PIECES_5.iter()
            .map(|mat| {
                mat.iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<Vec<usize>>>>()
    } else {
        piece::PIECES_6.iter()
            .map(|mat| {
                mat.iter()
                    .map(|row| row.to_vec())
                    .collect::<Vec<Vec<usize>>>()
            })
            .collect::<Vec<Vec<Vec<usize>>>>()
    };
    
    let mut ret = Vec::new();
    for piece in piece_list {
        let mut congruent_figures = Vec::new();
        // let figure = Board::from(piece);
        let figure = Board(piece);
        for vertically in [true,false] {
            for horizontally in [true,false] {
                for diagonally in [true,false] {
                    let normalized_figure = figure
                        .transform(vertically,horizontally,diagonally)
                        .normalize_coordinates();
                    if !congruent_figures.contains(&normalized_figure) {
                        congruent_figures.push(normalized_figure);
                    }
                }
            }
        }
        ret.push(congruent_figures);
    }
    ret
}

fn have_common_position(positions_a: &[usize], positions_b: &[usize]) -> bool {
    positions_b.iter().any(|pos| positions_a.contains(pos))
}

fn congruent_pieces(board: &Vec<Vec<usize>>, n: usize) -> (Vec<Vec<bool>>,Vec<usize>) {
    let num_pieces: usize = if n == 5 {
        piece::NUM_PIECES_5
    } else {
        piece::NUM_PIECES_6
    };

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

    // to one hot vector
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
                  pieces: &Vec<Vec<bool>>,
                  board: &Vec<Vec<usize>>) -> Board {
    // create a copy of the board object
    let mut ret: Board = Board::new(board.len(), board[0].len());
    // iterate through solution objects
    for &k in solution.iter() {
        let kind_value = kinds[k];
        let piece_pattern = &pieces[k];

        for i in 0..ret.height() {
            for j in 0..ret.width() {
                // Check if the corresponding piece_pattern is true
                if piece_pattern[i*ret.width() + j] {
                    *ret.get_mut(i,j) = kind_value;
                }
            }
        }
    }
    ret
}

fn solve_polymino(board: &Vec<Vec<usize>>, n: usize) -> usize {
    let num_pieces: usize = if n == 5 {
        piece::NUM_PIECES_5
    } else {
        piece::NUM_PIECES_6
    };

    let (pieces,kinds) = congruent_pieces(&board, n);
    let n_cells = board.iter().map(|row| row.len()).sum::<usize>();
    let mut m = Matrix::new(n_cells + num_pieces + 1);
    for onehot in &pieces {
        m.add_row(&onehot);
    }

    let mut congruent_solutions = HashSet::new();
    let mut solutions = Vec::new();
    for solution in solve(m).iter() {
        let solved_board = solution2board(&solution, &kinds, &pieces, &board);
        if !congruent_solutions.contains(&solved_board) {
            solutions.push(solution.clone());
            let diagonal_opt = if solved_board.is_square() {
                vec![true,false]
            } else {
                vec![false]
            };

            for vertically in [true,false] {
                for horizontally in [true,false] {
                    for &diagonally in &diagonal_opt {
                        let b = solved_board.transform(vertically,horizontally,diagonally);
                        congruent_solutions.insert(b.clone());
                    }
                }
            }
        }
    }
    solutions.len()
}
    

fn main() {
    // 8x8 - 4(center)
    let board1 = vec![
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,1,1,0,0,0],
        vec![0,0,0,1,1,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0],
    ];
    let n_sol1: usize = 65;

    // 5x12
    let board2 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let n_sol2: usize = 1010;

    // 6x10
    let board3 = vec![
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0],
    ];
    let n_sol3: usize = 2339;

    // 3x21b
    let board4 = vec![
        vec![0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,1,0,0,0,0,0,0,0,0],
    ];
    let n_sol4: usize = 3;
    
    // 3x20
    let board5 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let n_sol5: usize = 2;

    // 7x9 - 3
    let board6 = vec![
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,1,0,0,1,0,0,1,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0],
    ];
    let n_sol6: usize = 143;

    // 10x10 - 40
    let board7 = vec![
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
    let n_sol7: usize = 42;

    let board8 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let n_sol8: usize = 368;

    let problems  = vec![board1,board2,board3,board4,board5,board6,board7,board8];
    let n_solutions = vec![n_sol1,n_sol2,n_sol3,n_sol4,n_sol5,n_sol6,n_sol7,n_sol8];

    for i in 0..problems.len() {
        let start = Instant::now();
        let ans = solve_polymino(&problems[i], 5);
        let duration = start.elapsed();
        println!("n_solution={} correct_number={} time={:?}",
                 ans, n_solutions[i], duration);
    }

    let board6_1 = vec![
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
    let n_sol6_1: usize = 10;

    let board6_2 = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];
    let n_sol6_2 = 10;

    let board6_3 = vec![
        vec![0,1,1, 0,0,1, 0,1,1, 0,1,1, 1,0,1, 0,0,1, 0,0,1],
        vec![0,1,1, 0,1,1, 0,0,1, 0,1,1, 0,0,1, 0,0,1, 0,1,1],
        vec![0,1,1, 0,1,1, 0,1,1, 0,0,1, 0,1,1, 0,1,1, 0,0,1],
        vec![0,1,1, 0,1,1, 0,1,1, 0,1,1, 0,1,1, 0,1,1, 0,1,1],
        vec![0,1,1, 0,1,1, 0,1,1, 0,1,1, 0,1,1, 1,1,1, 1,1,1],
        vec![0,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1],

        vec![0,0,1, 0,1,1, 0,0,0, 0,1,1, 0,0,0, 1,0,0, 1,0,0],
        vec![0,1,1, 0,0,1, 0,1,1, 0,0,0, 1,0,1, 0,0,1, 1,0,1],
        vec![0,1,1, 0,0,1, 0,1,1, 0,1,1, 1,0,1, 1,0,1, 0,0,1],
        vec![0,0,1, 0,1,1, 0,1,1, 0,1,1, 1,0,1, 1,0,1, 1,0,1],
        vec![1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1],

        vec![1,0,0, 1,0,1, 1,0,1, 1,0,1, 1,0,1, 1,0,1, 1,0,1],
        vec![1,0,1, 1,0,0, 0,0,0, 0,0,0, 0,0,1, 1,0,1, 0,0,1],
        vec![1,0,1, 0,0,1, 1,0,1, 0,1,1, 0,1,1, 0,0,1, 0,0,1],
        vec![0,0,1, 1,0,1, 1,0,1, 0,1,1, 0,0,1, 0,1,1, 0,1,1],
        vec![1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 0,1,1, 1,1,1],

        vec![0,0,1, 1,1,0, 0,0,0, 1,1,0, 1,1,0, 1,0,0, 0,0,0],
        vec![0,0,1, 0,0,0, 1,0,0, 1,0,0, 0,0,0, 0,0,1, 0,1,0],
        vec![0,0,1, 1,0,1, 1,0,1, 0,0,1, 0,1,1, 0,1,1, 0,1,1],
        vec![1,1,1, 1,0,1, 1,1,1, 1,0,1, 0,1,1, 0,1,1, 1,1,1],
        vec![1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1],

        vec![0,1,0, 0,1,0, 1,0,0, 0,1,1, 1,0,1, 1,1,0, 1,1,0],
        vec![0,0,0, 0,0,0, 1,0,1, 0,0,1, 0,0,0, 0,0,0, 1,0,0],
        vec![0,1,1, 1,0,1, 0,0,1, 0,0,0, 0,0,1, 0,0,1, 0,0,1],
        vec![1,1,1, 1,1,1, 0,1,1, 1,1,1, 1,1,1, 1,1,1, 0,1,1],
        vec![1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1, 1,1,1],
    ];
    let n_sol6_3: usize = 1;
    
    let hproblems  = vec![board6_3];
    let hn_solutions = vec![n_sol6_3];

    for i in 0..hproblems.len() {
        let start = Instant::now();
        let ans = solve_polymino(&hproblems[i], 6);
        let duration = start.elapsed();
        println!("n_solution={} correct_number={} time={:?}",
                 ans, hn_solutions[i], duration);
    }
    
}
