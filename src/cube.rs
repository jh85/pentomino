pub mod cube {
    use crate::polycubelist::polycubelist::*;
    use crate::solutionset::solutionset::Transformable;
    
    #[derive(Clone,Eq,PartialEq,Hash,Debug)]
    pub struct Cube(pub Vec<Vec<Vec<usize>>>);

    impl Cube {
        pub fn new(dim0: usize, dim1: usize, dim2: usize) -> Self {
            Cube(vec![vec![vec![0;dim2]; dim1]; dim0])
        }

        pub fn get_mut(&mut self, i: usize, j: usize, k: usize) -> &mut usize {
            &mut self.0[i][j][k]
        }
        
        fn rotate_k(&self, k: usize) -> Cube {
            let dim0 = self.0.len();
            let dim1 = self.0[0].len();
            let dim2 = self.0[0][0].len();
            
            let mut rotated = vec![vec![vec![0; dim2]; dim1]; dim0];
            for x in 0..dim0 {
                for y in 0..dim1 {
                    for z in 0..dim2 {
                        match k {
                            0  => rotated[       x][       y][       z] = self.0[x][y][z],
                            1  => rotated[dim0-1-x][       z][       y] = self.0[x][y][z],
                            2  => rotated[       x][dim1-1-z][       y] = self.0[x][y][z],
                            3  => rotated[       x][       z][dim2-1-y] = self.0[x][y][z],
                            4  => rotated[dim0-1-y][       x][       z] = self.0[x][y][z],
                            5  => rotated[       y][dim1-1-x][       z] = self.0[x][y][z],
                            6  => rotated[       y][       x][dim2-1-z] = self.0[x][y][z],
                            7  => rotated[       y][       z][       x] = self.0[x][y][z],
                            8  => rotated[       z][       x][       y] = self.0[x][y][z],
                            9  => rotated[dim0-1-z][       y][       x] = self.0[x][y][z],
                            10 => rotated[       z][dim1-1-y][       x] = self.0[x][y][z],
                            11 => rotated[       z][       y][dim2-1-x] = self.0[x][y][z],
                            12 => rotated[       x][dim1-1-y][dim2-1-z] = self.0[x][y][z],
                            13 => rotated[dim0-1-x][       y][dim2-1-z] = self.0[x][y][z],
                            14 => rotated[dim0-1-x][dim1-1-y][       z] = self.0[x][y][z],
                            15 => rotated[dim0-1-x][dim1-1-z][dim2-1-y] = self.0[x][y][z],
                            16 => rotated[dim0-1-y][dim1-1-x][dim2-1-z] = self.0[x][y][z],
                            17 => rotated[       y][dim1-1-z][dim2-1-x] = self.0[x][y][z],
                            18 => rotated[dim0-1-y][       z][dim2-1-x] = self.0[x][y][z],
                            19 => rotated[dim0-1-y][dim1-1-z][       x] = self.0[x][y][z],
                            20 => rotated[       z][dim1-1-x][dim2-1-y] = self.0[x][y][z],
                            21 => rotated[dim0-1-z][       x][dim2-1-y] = self.0[x][y][z],
                            22 => rotated[dim0-1-z][dim1-1-x][       y] = self.0[x][y][z],
                            23 => rotated[dim0-1-z][dim1-1-y][dim2-1-x] = self.0[x][y][z],
                            _ => (),
                        }
                    }
                }
            }
            Cube(rotated)
        }

        fn generate_congruent_shapes(&self) -> Vec<Cube> {
            let dim0 = self.0.len();
            let dim1 = self.0[0].len();
            let dim2 = self.0[0][0].len();

            let mut cshapes = Vec::new();

            // x,y,z
            if true {
                cshapes.push(self.rotate_k(0));
                cshapes.push(self.rotate_k(12));
                cshapes.push(self.rotate_k(13));
                cshapes.push(self.rotate_k(14));
            }
            // x,z,y
            if dim1 == dim2 {
                cshapes.push(self.rotate_k(1));
                cshapes.push(self.rotate_k(2));
                cshapes.push(self.rotate_k(3));
                cshapes.push(self.rotate_k(15));
                
            }
            // y,x,z
            if dim0 == dim1 {
                cshapes.push(self.rotate_k(4));
                cshapes.push(self.rotate_k(5));
                cshapes.push(self.rotate_k(6));
                cshapes.push(self.rotate_k(16));
            }
            // z,y,x
            if dim0 == dim2 {
                cshapes.push(self.rotate_k(9));
                cshapes.push(self.rotate_k(10));
                cshapes.push(self.rotate_k(11));
                cshapes.push(self.rotate_k(23));
            }

            if dim0 == dim1 && dim0 == dim2 {
                // y,z,x
                cshapes.push(self.rotate_k(7));
                cshapes.push(self.rotate_k(17));
                cshapes.push(self.rotate_k(18));
                cshapes.push(self.rotate_k(19));

                // z,x,y
                cshapes.push(self.rotate_k(8));
                cshapes.push(self.rotate_k(20));
                cshapes.push(self.rotate_k(21));
                cshapes.push(self.rotate_k(22));
            }
            cshapes
        }

        pub fn normalize_coordinates(&self) -> Vec<(usize,usize,usize)> {
            let mut ret = Vec::new();
            for (i,plane) in self.0.iter().enumerate() {
                for (j,row) in plane.iter().enumerate() {
                    for (k,value) in row.iter().enumerate() {
                        if *value > 0 {
                            ret.push((i,j,k));
                        }
                    }
                }
            }
            let (min_i,min_j,min_k) = ret.iter()
                .fold((usize::MAX,usize::MAX,usize::MAX),
                      |(min_i,min_j,min_k),&(i,j,k)| (min_i.min(i), min_j.min(j),min_k.min(k)));
            ret.iter_mut().for_each(|(i,j,k)| {
                *i -= min_i;
                *j -= min_j;
                *k -= min_k;
            });
            ret
        }
    }

    impl Transformable for Cube {
        fn get_all_transformations(&self) -> Vec<Self> {
            self.generate_congruent_shapes()
        }
    }
    
    fn conv2usize(cube: Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<usize>>> {
        cube.into_iter()
            .map(|plane| {
                plane.into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|value| if value { 1 } else { 0 })
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }
    
    pub fn congruent_figures_for_each_piece_3d(n: usize) -> Vec<Vec<Vec<(usize,usize,usize)>>> {
        let all_pieces = free_polycubes(n);
        let mut ret = Vec::new();
        for piece_b in all_pieces {
            let piece = conv2usize(piece_b);
            let mut congruent_figures = Vec::new();
            for figure in Cube(piece).generate_congruent_shapes() {
                let normalized_figure = figure.normalize_coordinates();
                if !congruent_figures.contains(&normalized_figure) {
                    congruent_figures.push(normalized_figure);
                }
            }
            ret.push(congruent_figures);
        }
        ret
    }
}
