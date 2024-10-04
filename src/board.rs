pub mod board {
    use colored::Colorize;
    use crate::solutionset::solutionset::Transformable;
    
    #[derive(Clone,Eq,PartialEq,Hash)]
    pub struct Board(pub Vec<Vec<usize>>);

    impl Board {
        pub fn new(h: usize, w: usize) -> Self {
            Board(vec![vec![0;w]; h])
        }
    
        pub fn height(&self) -> usize {
            self.0.len()
        }

        pub fn width(&self) -> usize {
            if self.0.is_empty() {
                0
            } else {
                self.0[0].len()
            }
        }

        pub fn get_mut(&mut self, i: usize, j: usize) -> &mut usize {
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

        pub fn is_square(&self) -> bool {
            self.width() == self.height()
        }

        pub fn transform(&self, vertically: bool, horizontally: bool, diagonally: bool) -> Self {
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

        pub fn normalize_coordinates(&self) -> Vec<(usize,usize)> {
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

        fn num2color(cell: usize) -> (u8,u8,u8) {
            let (r,g,b) = match cell {
                0 => (255, 0, 0), // Red
                1 => (0, 255, 0), // Green
                2 => (0, 0, 255), // Blue
                3 => (255, 255, 0), // Yellow
                4 => (255, 165, 0), // Orange
                5 => (128, 0, 128), // Purple
                6 => (0, 255, 255), // Cyan
                7 => (255, 20, 147), // Deep Pink
                8 => (0, 128, 0), // Dark Green
                9 => (75, 0, 130), // Indigo
                10 => (255, 192, 203), // Pink
                11 => (139, 69, 19), // Saddle Brown
                12 => (169, 169, 169), // Dark Gray
                13 => (255, 69, 0), // Red-Orange
                14 => (0, 0, 128), // Navy
                15 => (255, 105, 180), // Hot Pink
                16 => (255, 255, 224), // Light Yellow
                17 => (0, 128, 128), // Teal
                18 => (165, 42, 42), // Brown
                19 => (210, 105, 30), // Chocolate
                20 => (124, 252, 0), // Lawn Green
                21 => (0, 255, 127), // Spring Green
                22 => (0, 100, 0), // Dark Green
                23 => (127, 255, 212), // Aquamarine
                24 => (72, 61, 139), // Dark Slate Blue
                25 => (220, 20, 60), // Crimson
                26 => (173, 255, 47), // Green-Yellow
                27 => (255, 140, 0), // Dark Orange
                28 => (255, 99, 71), // Tomato
                29 => (144, 238, 144), // Light Green
                30 => (70, 130, 180), // Steel Blue
                31 => (135, 206, 250), // Light Sky Blue
                32 => (46, 139, 87), // Sea Green
                33 => (255, 215, 0), // Gold
                34 => (0, 191, 255), // Deep Sky Blue
                35 => (238, 130, 238), // Violet
                _  => (128, 0, 0), // Maroon
            };
            (r,g,b)
        }
        
        pub fn pprint(&self) {
            for row in self.0.iter() {
                for value in row.iter() {
                    let cell = "●";
                    let (r,g,b) = Board::num2color(*value);
                    let color_cell = cell.truecolor(r,g,b);
                    print!("{}", color_cell);
                }
                println!();
            }
        }
    }

    impl Transformable for Board {
        fn get_all_transformations(&self) -> Vec<Self> {
            let mut transformations = Vec::new();
            let diagonal_opt = if self.is_square() {
                vec![true, false]
            } else {
                vec![false]
            };

            for vertically in [true, false] {
                for horizontally in [true, false] {
                    for &diagonally in &diagonal_opt {
                        transformations.push(self.transform(vertically, horizontally, diagonally));
                    }
                }
            }

            transformations
        }
    }
}
