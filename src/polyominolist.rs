pub mod polyominolist {
    use std::collections::HashSet;

    fn dfs(i: usize,
           j: usize,
           last: usize,
           next_order: &mut usize,
           max_visited: &mut usize,
           ominos: &mut Vec<Vec<Vec<usize>>>,
           order: &mut Vec<Vec<Option<usize>>>,
           border: &mut HashSet<(usize, usize)>,
           ordered_queue: &mut Vec<(usize, usize, usize, usize)>,
           visited: &mut Vec<Vec<usize>>,
           visited_queue: &mut Vec<(usize, usize)>) {
        let height = visited.len();
        let width = visited[0].len();

        if last == 0 {
            ominos.push(visited.clone());
            return;
        }

        // Neighboring cells of (i,j)
        let directions = vec![(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];
        for (di, dj) in directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if (ni == 0 && nj < height as isize) || !(0 <= ni && ni < height as isize &&
                                                      0 <= nj && nj < width as isize) {
                continue;
            }
            let ni: usize = ni as usize;
            let nj: usize = nj as usize;
            if order[ni][nj].is_none() {
                order[ni][nj] = Some(*next_order);
                *next_order += 1;
                border.insert((ni, nj));
                ordered_queue.push((i, j, ni, nj));
            }
        }

        // Places where you can insert cells
        let border_list: Vec<(usize, usize)> = border.iter().cloned().collect();
        for (ni, nj) in border_list {
            if let Some(order_value) = order[ni][nj] {
                if order_value > *max_visited {
                    // depth + 1
                    visited[ni][nj] = 1;
                    visited_queue.push((ni, nj));
                    let pre_max_visited = *max_visited;
                    *max_visited = order_value;
                    border.remove(&(ni, nj));

                    dfs(ni,
                        nj,
                        last - 1,
                        next_order,
                        max_visited,
                        ominos,
                        order,
                        border,
                        ordered_queue,
                        visited,
                        visited_queue);

                    // depth -1
                    while !ordered_queue.is_empty() && (ni, nj) == (ordered_queue.last().unwrap().0,
                                                                    ordered_queue.last().unwrap().1) {
                        let (_, _, pi, pj) = ordered_queue.pop().unwrap();
                        order[pi][pj] = None;
                    }
                    border.insert((ni, nj));
                    *max_visited = pre_max_visited;
                    visited_queue.pop();
                    visited[ni][nj] = 0;
                }
            }
        }
    }

    fn enumerate_n_omino(n: usize) -> Vec<Vec<Vec<usize>>> {
        let height = n;
        let width = 2 * n - 1;
        let center = n - 1;

        let mut ominos: Vec<Vec<Vec<usize>>> = Vec::new();
        let mut visited = vec![vec![0; width]; height];
        let mut visited_queue: Vec<(usize, usize)> = Vec::new();
        let mut order = vec![vec![None; width]; height];
        let mut ordered_queue: Vec<(usize, usize, usize, usize)> = Vec::new();
        let mut border: HashSet<(usize, usize)> = HashSet::new();

        // Search starts at (0, center_w)
        visited_queue.push((0, center));
        visited[0][center] = 1;
        order[0][center] = Some(0);

        let mut max_visited = 0usize;
        let mut next_order = 1usize;

        dfs(0,
            center,
            center,
            &mut next_order,
            &mut max_visited,
            &mut ominos,
            &mut order,
            &mut border,
            &mut ordered_queue,
            &mut visited,
            &mut visited_queue);
        ominos
    }

    fn normalize(omino: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let height = omino.len();
        let width = omino[0].len();
        let mut min_i = usize::MAX;
        let mut max_i = 0;
        let mut min_j = usize::MAX;
        let mut max_j = 0;

        for i in 0..height {
            for j in 0..width {
                if omino[i][j] > 0 {
                    min_i = min_i.min(i);
                    max_i = max_i.max(i);
                    min_j = min_j.min(j);
                    max_j = max_j.max(j);
                }
            }
        }

        let new_height = max_i - min_i + 1;
        let new_width = max_j - min_j + 1;

        let mut omino2 = vec![vec![0; height]; height]; // height == n
        for i in 0..new_height {
            for j in 0..new_width {
                omino2[i][j] = omino[i + min_i][j + min_j];
            }
        }
        omino2
    }

    fn congruent_forms(omino: &Vec<Vec<usize>>) -> Vec<Vec<Vec<usize>>> {
        let mut cforms = Vec::new();

        for &horizontally in &[true, false] {
            for &vertically in &[true, false] {
                for &diagonally in &[true, false] {
                    let mut o = omino.clone();
                    if diagonally {
                        // Transpose the matrix
                        for i in 0..o.len() {
                            for j in 0..o[0].len() {
                                o[j][i] = omino[i][j];
                            }
                        }
                    }
                    if horizontally {
                        for row in &mut o {
                            row.reverse();
                        }
                    }
                    if vertically {
                        o.reverse();
                    }
                    let o = normalize(&o);
                    cforms.push(o);
                }
            }
        }
        cforms
    }

    pub fn free_polyominos(n: usize) -> Vec<Vec<Vec<usize>>> {
        let ominos = enumerate_n_omino(n);
        let mut ominos_all: HashSet<Vec<Vec<usize>>> = HashSet::new();
        let mut ominos2 = Vec::new();
        for o in &ominos {
            let norm_o = normalize(o);
            if !ominos_all.contains(&norm_o) {
                ominos2.push(norm_o.clone());
                for norm_o2 in congruent_forms(&norm_o) {
                    ominos_all.insert(norm_o2);
                }
            }
        }
        ominos2
    }
}
