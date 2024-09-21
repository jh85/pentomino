pub mod dlx {
    use std::{
        cmp::Ordering,
        ops::Range,
        usize,
    };

    pub fn solve(mut m: Matrix, num_solutions: usize) -> Vec<Vec<usize>> {
        let mut answers = Vec::new();
        let mut answer = Vec::new();
        go(&mut m, &mut answer, &mut answers, num_solutions);
        answers
    }

    fn go(m: &mut Matrix, partial_answer: &mut Vec<usize>, answers: &mut Vec<Vec<usize>>, num_solutions: usize) {
        if num_solutions > 0 && answers.len() >= num_solutions {
            return;
        }
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
            go(m, partial_answer, answers, num_solutions);
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

    pub struct Matrix {
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
        pub fn new(n_cols: usize) -> Matrix {
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

        pub fn add_row(&mut self, row: &[bool]) {
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
}
