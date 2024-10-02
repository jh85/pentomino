pub mod polyocubelist {
    use std::collections::HashSet;

    macro_rules! set_value_3d {
        ($array:expr, $loc:expr, $value:expr) => {{
            $array[$loc[0]][$loc[1]][$loc[2]] = $value;
        }};
    }

    macro_rules! get_value_3d {
        ($array:expr, $loc:expr) => {{
            $array[$loc[0]][$loc[1]][$loc[2]]
        }};
    }

    macro_rules! add_locations_3d {
        ($loc1:expr, $loc2:expr) => {{
            [
                $loc1[0] + $loc2[0],
                $loc1[1] + $loc2[1],
                $loc1[2] + $loc2[2],
            ]
        }};
    }

    fn dfs(loc: &[usize; 3],
           required_cells: u32,
           mut next_number: u32,
           numbered: &mut Vec<Vec<Vec<Option<u32>>>>,
           mut max_visited: u32,
           visited: &mut Vec<Vec<Vec<u32>>>,
           border: &mut HashSet<[usize; 3]>,
           solutions: &mut Vec<Vec<Vec<Vec<u32>>>>) {
        let n = visited.len();
        let dim: [usize; 3] = [n, 2*n-1, 2*n-1];

        if required_cells == 0 {
            solutions.push(visited.clone());
            return;
        }

        let moves: [[i32; 3]; 6] = [
            [ 1,  0,  0],
            [ 0,  1,  0],
            [ 0,  0,  1],
            [-1,  0,  0],
            [ 0, -1,  0],
            [ 0,  0, -1],
        ];

        for mv in &moves {
            let loc_i32: [i32; 3] = [loc[0] as i32, loc[1] as i32, loc[2] as i32];
            let new_loc_temp = add_locations_3d!(loc_i32, *mv);
            if new_loc_temp[0] < 0 || new_loc_temp[1] < 0 || new_loc_temp[2] < 0 {
                continue;
            }
            let new_loc: [usize; 3] = [
                new_loc_temp[0] as usize,
                new_loc_temp[1] as usize,
                new_loc_temp[2] as usize,
            ];

            if new_loc[0] == 0 && (n <= new_loc[1] || n <= new_loc[2]) {
                continue;
            }

            if dim[0] <= new_loc[0] || dim[1] <= new_loc[1] || dim[2] <= new_loc[2] {
                continue;
            }

            let value_numbered = get_value_3d!(numbered, &new_loc);
            if value_numbered.is_none() {
                set_value_3d!(numbered, &new_loc, Some(next_number));
                next_number += 1;
                border.insert(new_loc);
            }
        }

        let border_list: Vec<_> = border.clone().into_iter().collect();
        for new_loc in border_list {
            let value_numbered = get_value_3d!(numbered, &new_loc);
            if let Some(v) = value_numbered {
                if v > max_visited {
                    set_value_3d!(visited, &new_loc, 1);

                    let pre_max_visited = max_visited;
                    max_visited = v;
                    border.remove(&new_loc);

                    dfs(&new_loc,
                        required_cells - 1,
                        next_number,
                        &mut numbered.clone(),
                        max_visited,
                        &mut visited.clone(),
                        &mut border.clone(),
                        solutions);

                    border.insert(new_loc);
                    max_visited = pre_max_visited;

                    set_value_3d!(visited, &new_loc, 0);
                }
            }
        }
    }

    fn list_3d_ominoes(n: usize) -> Vec<Vec<Vec<Vec<u32>>>> {
        let size = 2 * n - 1;
        let mut visited = vec![vec![vec![0; size]; size]; n];
        let mut numbered = vec![vec![vec![None; size]; size]; n];
        let border: HashSet<[usize; 3]> = HashSet::new();

        let start_loc: [usize; 3] = [0, n - 1, n - 1];
        set_value_3d!(visited, &start_loc, 1);
        set_value_3d!(numbered, &start_loc, Some(0));

        let max_visited = 0;
        let next_number = 1;
        let mut solutions = Vec::new();
        let required_cells = n - 1;

        dfs(&start_loc,
            required_cells as u32,
            next_number,
            &mut numbered.clone(),
            max_visited,
            &mut visited.clone(),
            &mut border.clone(),
            &mut solutions);

        solutions
    }

    fn normalize(omino: &Vec<Vec<Vec<u32>>>, n: usize) -> Vec<Vec<Vec<u32>>> {
        let dim0 = omino.len();
        let dim1 = omino[0].len();
        let dim2 = omino[0][0].len();
        let mut min_i = usize::MAX;
        let mut max_i = 0;
        let mut min_j = usize::MAX;
        let mut max_j = 0;
        let mut min_k = usize::MAX;
        let mut max_k = 0;

        for i in 0..dim0 {
            for j in 0..dim1 {
                for k in 0..dim2 {
                    if omino[i][j][k] > 0 {
                        min_i = min_i.min(i);
                        max_i = max_i.max(i);
                        min_j = min_j.min(j);
                        max_j = max_j.max(j);
                        min_k = min_k.min(k);
                        max_k = max_k.max(k);
                    }
                }
            }
        }
        let mut omino2 = vec![vec![vec![0; n]; n]; n];
        for i in 0..(max_i - min_i + 1) {
            for j in 0..(max_j - min_j + 1) {
                for k in 0..(max_k - min_k + 1) {
                    omino2[i][j][k] = omino[i + min_i][j + min_j][k + min_k];
                }
            }
        }
        omino2
    }

    fn rotate_k(cube: &Vec<Vec<Vec<u32>>>, k: usize) -> Vec<Vec<Vec<u32>>> {
        let n = cube.len();
        let mut rotated = vec![vec![vec![0; n]; n]; n];
        for x in 0..n {
            for y in 0..n {
                for z in 0..n {
                    match k {
                        0  => rotated[        x][        y][        z] = cube[x][y][z],
                        1  => rotated[n - 1 - x][        z][        y] = cube[x][y][z],
                        2  => rotated[        x][n - 1 - z][        y] = cube[x][y][z],
                        3  => rotated[        x][        z][n - 1 - y] = cube[x][y][z],
                        4  => rotated[n - 1 - y][        x][        z] = cube[x][y][z],
                        5  => rotated[        y][n - 1 - x][        z] = cube[x][y][z],
                        6  => rotated[        y][        x][n - 1 - z] = cube[x][y][z],
                        7  => rotated[        y][        z][        x] = cube[x][y][z],
                        8  => rotated[        z][        x][        y] = cube[x][y][z],
                        9  => rotated[n - 1 - z][        y][        x] = cube[x][y][z],
                        10 => rotated[        z][n - 1 - y][        x] = cube[x][y][z],
                        11 => rotated[        z][        y][n - 1 - x] = cube[x][y][z],
                        12 => rotated[        x][n - 1 - y][n - 1 - z] = cube[x][y][z],
                        13 => rotated[n - 1 - x][        y][n - 1 - z] = cube[x][y][z],
                        14 => rotated[n - 1 - x][n - 1 - y][        z] = cube[x][y][z],
                        15 => rotated[n - 1 - x][n - 1 - z][n - 1 - y] = cube[x][y][z],
                        16 => rotated[n - 1 - y][n - 1 - x][n - 1 - z] = cube[x][y][z],
                        17 => rotated[        y][n - 1 - z][n - 1 - x] = cube[x][y][z],
                        18 => rotated[n - 1 - y][        z][n - 1 - x] = cube[x][y][z],
                        19 => rotated[n - 1 - y][n - 1 - z][        x] = cube[x][y][z],
                        20 => rotated[        z][n - 1 - x][n - 1 - y] = cube[x][y][z],
                        21 => rotated[n - 1 - z][        x][n - 1 - y] = cube[x][y][z],
                        22 => rotated[n - 1 - z][n - 1 - x][        y] = cube[x][y][z],
                        23 => rotated[n - 1 - z][n - 1 - y][n - 1 - x] = cube[x][y][z],
                        _ => (),
                    }
                }
            }
        }
        rotated
    }

    fn congruent_forms(omino: &Vec<Vec<Vec<u32>>>) -> Vec<Vec<Vec<Vec<u32>>>> {
        let mut cforms = Vec::new();
        let n = omino.len();
        for k in 0..24 {
            let o = rotate_k(omino, k);
            let o_norm = normalize(&o, n);
            cforms.push(o_norm);
        }
        cforms
    }

    pub fn free_polyocubes(n: usize) -> Vec<Vec<Vec<Vec<u32>>>> {
        let ominoes = list_3d_ominoes(n);
        let mut ominoes_all: HashSet<Vec<Vec<Vec<u32>>>> = HashSet::new();
        let mut ominoes_nodup = Vec::new();
        
        for o in ominoes {
            let norm_o = normalize(&o, n);
            if !ominoes_all.contains(&norm_o) {
                let cforms = congruent_forms(&norm_o);
                for c in cforms {
                    ominoes_all.insert(c);
                }
                ominoes_nodup.push(norm_o);
            }
        }
        ominoes_nodup
    }
}
