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
           numbered_q: &mut Vec<[usize;3]>,
           mut max_visited: u32,
           visited: &mut Vec<Vec<Vec<bool>>>,
           border: &mut HashSet<[usize; 3]>,
           solutions: &mut Vec<Vec<Vec<Vec<bool>>>>) {
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

            if get_value_3d!(numbered, &new_loc).is_none() {
                set_value_3d!(numbered, &new_loc, Some(next_number));
                next_number += 1;
                border.insert(new_loc);
                numbered_q.push(new_loc);
            }
        }

        let border_list: Vec<_> = border.clone().into_iter().collect();
        for new_loc in border_list {
            if let Some(v) = get_value_3d!(numbered, &new_loc) {
                if v > max_visited {
                    set_value_3d!(visited, &new_loc, true);
                    let pre_max_visited = max_visited;
                    max_visited = v;
                    border.remove(&new_loc);

                    let mut numbered_q2: Vec<[usize;3]> = Vec::new();
                    dfs(&new_loc,
                        required_cells - 1,
                        next_number,
                        numbered,
                        &mut numbered_q2,
                        max_visited,
                        visited,
                        border,
                        solutions);

                    while numbered_q2.len() > 0 {
                        let numbered_loc = numbered_q2.pop().unwrap();
                        set_value_3d!(numbered, &numbered_loc, None);
                    }

                    border.insert(new_loc);
                    max_visited = pre_max_visited;

                    set_value_3d!(visited, &new_loc, false);
                }
            }
        }
    }

    fn generate_polyocube_candidates(n: usize) -> Vec<Vec<Vec<Vec<bool>>>> {
        let size = 2 * n - 1;
        let mut visited = vec![vec![vec![false; size]; size]; n];
        let mut numbered = vec![vec![vec![None; size]; size]; n];
        let mut numbered_q: Vec<[usize;3]> = Vec::new();
        let mut border: HashSet<[usize; 3]> = HashSet::new();

        let start_loc: [usize; 3] = [0, n - 1, n - 1];
        set_value_3d!(visited, &start_loc, true);
        set_value_3d!(numbered, &start_loc, Some(0));

        let max_visited = 0;
        let next_number = 1;
        let mut solutions = Vec::new();
        let required_cells = n - 1;

        dfs(&start_loc,
            required_cells as u32,
            next_number,
            &mut numbered,
            &mut numbered_q,
            max_visited,
            &mut visited,
            &mut border,
            &mut solutions);

        solutions
    }

    fn normalize(omino: &Vec<Vec<Vec<bool>>>, n: usize) -> Vec<Vec<Vec<bool>>> {
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
                    if omino[i][j][k] {
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
        let mut omino2 = vec![vec![vec![false; n]; n]; n];
        for i in 0..(max_i - min_i + 1) {
            for j in 0..(max_j - min_j + 1) {
                for k in 0..(max_k - min_k + 1) {
                    omino2[i][j][k] = omino[i + min_i][j + min_j][k + min_k];
                }
            }
        }
        omino2
    }

    fn rotate_k(cube: &Vec<Vec<Vec<bool>>>, k: usize) -> Vec<Vec<Vec<bool>>> {
        let n = cube.len();
        let mut rotated = vec![vec![vec![false; n]; n]; n];
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

    fn generate_congruent_shapes(omino: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<Vec<bool>>>> {
        let mut congruent_shapes = Vec::new();
        let n = omino.len();
        for k in 0..24 {
            let o = rotate_k(omino, k);
            let o_norm = normalize(&o, n);
            congruent_shapes.push(o_norm);
        }
        congruent_shapes
    }

    fn conv2vec(cube: &Vec<Vec<Vec<bool>>>) -> Vec<usize> {
        let mut v = Vec::new();
        let n = cube.len();
        'outer: for i in 0..n {
            for j in 0..n {
                for k in 0..n {
                    if cube[i][j][k] {
                        v.push(i*n*n + j*n + k);
                        if v.len() == n {
                            break 'outer
                        }
                    }
                }
            }
        }
        v
    }

    pub fn free_polyocubes_vec(n: usize) -> Vec<Vec<usize>> {
        let candidates = generate_polyocube_candidates(n);
        let mut polyocube_all: HashSet<Vec<usize>> = HashSet::new();
        let mut polyocube_nodup = Vec::new();
    
        for cuboid in candidates {
            let normalized_cube = normalize(&cuboid, n);
            let shape_vec = conv2vec(&normalized_cube);
            if !polyocube_all.contains(&shape_vec) {
                for congruent_normalized_cube in generate_congruent_shapes(&normalized_cube) {
                    let congruent_shape_vec = conv2vec(&congruent_normalized_cube);
                    polyocube_all.insert(congruent_shape_vec);
                }
                polyocube_nodup.push(shape_vec);
            }
        }
        polyocube_nodup
    }

    pub fn free_polyocubes(n: usize) -> Vec<Vec<Vec<Vec<bool>>>> {
        let candidates = generate_polyocube_candidates(n);
        let mut polyocube_all: HashSet<Vec<Vec<Vec<bool>>>> = HashSet::new();
        let mut polyocube_nodup = Vec::new();

        for cuboid in candidates {
            let normalized_cube = normalize(&cuboid, n);
            if !polyocube_all.contains(&normalized_cube) {
                for cshape in generate_congruent_shapes(&normalized_cube) {
                    polyocube_all.insert(cshape);
                }
                polyocube_nodup.push(normalized_cube);
            }
        }
        polyocube_nodup
    }
}
