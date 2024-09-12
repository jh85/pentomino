use std::{
    cmp::Ordering,
    ops::Range,
    usize,
    collections::HashSet,
    time::Instant,
};

fn solve(mut m: Matrix) -> Vec<Vec<usize>> {
    let mut answers = Vec::new();
    let mut answer = Vec::new();
    go(&mut m, &mut answer, &mut answers);
    answers
}

fn go(m: &mut Matrix, partial_answer: &mut Vec<usize>, answers: &mut Vec<Vec<usize>>) {
    let c = {
        let mut i = m.x.cursor(0);
        let mut c = match i.next(&m.x) {
            Some(it) => it,
            None => {
                let mut answer: Vec<usize> = partial_answer.iter().map(|&cell| m.row_of(cell)).collect();
                answer.sort();
                answers.push(answer);
                return;
            }
        };
        while let Some(next_c) = i.next(&m.x) {
            if m.size[next_c] < m.size[c] {
                c = next_c;
            }
        }
        c
    };

    m.cover(c);
    let mut r = m.y.cursor(c);
    while let Some(r) = r.next(&m.y) {
        partial_answer.push(r);
        let mut j = m.x.cursor(r);
        while let Some(j) = j.next(&m.x) {
            m.cover(m.c[j]);
        }
        go(m, partial_answer, answers);
        let mut j = m.x.cursor(r);
        while let Some(j) = j.prev(&m.x) {
            m.uncover(m.c[j]);
        }
        partial_answer.pop();
    }
    m.uncover(c);
}

struct Link {
    prev: usize,
    next: usize,
}

struct LinkedList {
    data: Vec<Link>,
}

impl LinkedList {
    fn with_capacity(cap: usize) -> LinkedList {
        LinkedList { data: Vec::with_capacity(cap) }
    }
    fn alloc(&mut self) -> usize {
        let length = self.data.len();
        self.data.push(Link { prev: length, next: length });
        length
    }
    // Inserts b into a-c to get a-b-c
    fn insert(&mut self, a: usize, b: usize) {
        let c = self.data[a].next;
        self.data[b].prev = a;
        self.data[b].next = c;
        self.data[a].next = b;
        self.data[c].prev = b;
    }
    // Removes b from a-b-c to get a-c
    fn remove(&mut self, b: usize) {
        let a = self.data[b].prev;
        let c = self.data[b].next;
        self.data[a].next = self.data[b].next;
        self.data[c].prev = self.data[b].prev;
    }
    // Restore previously removed b to get a-b-c
    fn restore(&mut self, b: usize) {
        let a = self.data[b].prev;
        let c = self.data[b].next;
        self.data[a].next = b;
        self.data[c].prev = b;
    }

    fn cursor(&self, head: usize) -> Cursor {
        Cursor { head: head, curr: head }
    }
}

struct Cursor {
    head: usize,
    curr: usize,
}

impl Cursor {
    fn next(&mut self, list: &LinkedList) -> Option<usize> {
        self.curr = list.data[self.curr].next;
        if self.curr == self.head {
            None
        } else {
            Some(self.curr)
        }
    }
    fn prev(&mut self, list: &LinkedList) -> Option<usize> {
        self.curr = list.data[self.curr].prev;
        if self.curr == self.head {
            None
        } else {
            Some(self.curr)
        }
    }
}

struct Matrix {
    // Auxilary map to get from cell to row, could be encoded more efficiently.
    row_ranges: Vec<Range<usize>>,

    // SoA fields
    // Links along the horizontal dimension
    x: LinkedList,
    // Links along the vertical dimension
    y: LinkedList,
    // Pointer to column headers
    c: Vec<usize>,
    // For column headers, the size of the column
    size: Vec<usize>,
}

impl Matrix {
    fn new(n_cols: usize) -> Matrix {
        let mut ret = Matrix {
            row_ranges: Vec::new(),
            x: LinkedList::with_capacity(n_cols + 1),
            y: LinkedList::with_capacity(n_cols + 1),
            c: Vec::with_capacity(n_cols + 1),
            size: Vec::with_capacity(n_cols + 1),
        };
        ret.alloc_column();
        for _ in 0..n_cols {
            ret.add_column();
        }
        ret
    }
    fn alloc(&mut self, c: usize) -> usize {
        self.c.push(c);
        let cell_idx = self.x.alloc();
        self.y.alloc();
        cell_idx
    }
    fn alloc_column(&mut self) -> usize {
        let cell_idx = self.alloc(0);
        self.c[cell_idx] = cell_idx;
        self.size.push(0);
        cell_idx
    }
    fn add_column(&mut self) {
        let new_col = self.alloc_column();
        self.x.insert(self.x.data[0].prev, new_col);
    }

    fn add_row(&mut self, row: &[bool]) {
        assert_eq!(row.len(), self.size.len() - 1);
        let row_start = self.x.data.len();
        let mut c = 0;
        let mut prev = None;
        for &is_filled in row {
            c = self.x.data[c].next;
            if is_filled {
                self.size[c] += 1;
                let new_cell = self.alloc(c);
                self.y.insert(self.y.data[c].prev, new_cell);
                if let Some(prev) = prev {
                    self.x.insert(prev, new_cell);
                }
                prev = Some(new_cell);
            }
        }
        let row_end = self.x.data.len();
        self.row_ranges.push(row_start..row_end);
    }

    fn row_of(&self, cell: usize) -> usize {
        self.row_ranges.binary_search_by(|range| {
            if cell < range.start {
                Ordering::Greater
            } else if range.start <= cell && cell < range.end {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        }).unwrap()
    }

    fn cover(&mut self, c: usize) {
        self.x.remove(c);
        let mut i = self.y.cursor(c);
        while let Some(i) = i.next(&self.y) {
            let mut j = self.x.cursor(i);
            while let Some(j) = j.next(&self.x) {
                self.y.remove(j);
                self.size[self.c[j]] -= 1;
            }
        }
    }
    fn uncover(&mut self, c: usize) {
        let mut i = self.y.cursor(c);
        while let Some(i) = i.prev(&self.y) {
            let mut j = self.x.cursor(i);
            while let Some(j) = j.prev(&self.x) {
                self.size[self.c[j]] += 1;
                self.y.restore(j);
            }
        }
        self.x.restore(c);
    }
}

const T: bool = true;
const F: bool = false;

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

    fn congruents_for_each_piece() -> Vec<Vec<Vec<(usize,usize)>>> {
        let mut ret = Vec::new();
        for piece in Self::PIECES {
            let mut congruents = Vec::new();
            for f in [true,false] {
                for r in [0,1,2,3] {
                    let p = piece.transform(f,r).true_locations();
                    if !congruents.contains(&p) {
                        congruents.push(p);
                    }
                }
            }
            ret.push(congruents);
        }
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
}

fn have_common_location(locations_a: &[(usize,usize)], locations_b: &[(usize,usize)]) -> bool {
    // let set_a: HashSet<_> = locations_a.iter().cloned().collect();
    // locations_b.iter().any(|loc| set_a.contains(loc))
    locations_b.iter().any(|loc| locations_a.contains(loc))
}

fn congruent_pieces(board: &Vec<Vec<bool>>) -> (Vec<Vec<bool>>,Vec<usize>) {
    let mut locations: Vec<Vec<usize>> = Vec::new();
    let mut kinds: Vec<usize> = Vec::new();

    let rows: usize = board.len();
    let cols: usize = board[0].len();
    let mut bad_locations = Vec::new();
    for (y,row) in board.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value {
                bad_locations.push((x,y));
            }
        }
    }
    for (k,congruents) in Piece::congruents_for_each_piece().iter().enumerate() {
        for congruent in congruents {
            let (xlen,ylen) = congruent.iter().fold((usize::MIN,usize::MIN),|(xmax,ymax),&(x,y)| (xmax.max(x),ymax.max(y)));
            for y_offset in 0..rows.saturating_sub(ylen) {
                for x_offset in 0..cols.saturating_sub(xlen) {
                    let new_congruent: Vec<_> = congruent.into_iter().map(|(x,y)| (x+x_offset,y+y_offset)).collect();
                    if have_common_location(&new_congruent, &bad_locations) {
                        continue;
                    } else {
                        let mut loc: Vec<_> = new_congruent.iter().map(|&(x,y)| x+y*cols).collect();
                        loc.push(k + rows*cols);
                        locations.push(loc);
                        kinds.push(k);
                    }
                }
            }
        }
    }
    let mut bad_loc: Vec<_> = bad_locations.iter().map(|&(x,y)| x+y*cols).collect();
    bad_loc.push(12 + rows*cols);
    locations.push(bad_loc);

    let sz: usize = 12 + rows*cols + 1;
    let locations2: Vec<Vec<bool>> = locations.iter().map(|inner_vec| {
        let mut bool_vec = vec![false; sz];
        inner_vec.iter().for_each(|&index| bool_vec[index] = true);
        bool_vec
    }).collect();
    kinds.push(12);
    (locations2,kinds)
}

#[derive(Clone,Hash,Eq,PartialEq)]
struct NumBoard(Vec<Vec<usize>>);

impl NumBoard {
    fn new(v: usize, h: usize) -> Self {
        NumBoard(vec![vec![0; h]; v])
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn row_len(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    fn get_mut(&mut self, v: usize, h: usize) -> &mut usize {
        &mut self.0[v][h]
    }
    
    fn flip_h(&self) -> Self {
        let ret: Vec<Vec<usize>> = self.0
            .iter()
            .map(|row| row.iter().rev().cloned().collect())
            .collect();
        NumBoard(ret)
    }

    fn flip_v(&self) -> Self {
        let ret: Vec<Vec<usize>> = self.0.iter().rev().cloned().collect();
        NumBoard(ret)            
    }

    fn flip_d(&self) -> Self {
        let size = self.0.len();
        let mut ret = vec![vec![0; size]; size];
        for i in 0..size {
            for j in 0..size {
                ret[i][j] = self.0[j][i];
            }
        }
        NumBoard(ret)
    }

    fn is_square(&self) -> bool {
        self.0.len() == self.0[0].len()
    }

    fn transform(&self, is_flip_h: bool, is_flip_v: bool, is_flip_d: bool) -> Self {
        let mut ret = if is_flip_h {
            self.flip_h()
        } else {
            self.clone()
        };

        ret = if is_flip_v {
            ret.flip_v()
        } else {
            ret
        };

        ret = if is_flip_d {
            ret.flip_d()
        } else {
            ret
        };
        ret
    }
}

fn solution2board(solution: &Vec<usize>,
                  kinds: &Vec<usize>,
                  pieces: &Vec<Vec<bool>>,
                  board: &Vec<Vec<bool>>) -> NumBoard {
    // create a copy of the board object
    let mut ret: NumBoard = NumBoard::new(board.len(), board[0].len());
    // iterate through solution objects
    for &k in solution.iter() {
        let kind_value = kinds[k];
        let piece_pattern = &pieces[k];

        for i in 0..ret.len() {
            for j in 0..ret.row_len() {
                // Check if the corresponding piece_pattern is true
                if piece_pattern[i*ret.row_len() + j] {
                    *ret.get_mut(i,j) = kind_value;
                }
            }
        }
    }
    ret
}

fn solve_pentomino(board: &Vec<Vec<bool>>) -> usize {
    let (pieces,kinds) = congruent_pieces(&board);
    let n_cells = board.iter().map(|row| row.len()).sum::<usize>();
    let mut m = Matrix::new(12+n_cells+1);
    for loc in &pieces {
        m.add_row(&loc);
    }

    let mut all_solutions = HashSet::new();
    let mut sol = Vec::new();
    let solutions = solve(m);
    for solution in solutions.iter() {
        let board = solution2board(&solution, &kinds, &pieces, &board);
        if !all_solutions.contains(&board) {
            sol.push(solution.clone());
            let flip_diagonal = if board.is_square() {
                vec![true,false]
            } else {
                vec![false]
            };
                
            for flip_x in [true,false] {
                for flip_y in [true,false] {
                    for &flip_d in &flip_diagonal {
                        let b = board.transform(flip_x, flip_y, flip_d);
                        all_solutions.insert(b.clone());
                    }
                }
            }
        }
    }
    sol.len()
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
