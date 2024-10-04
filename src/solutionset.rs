pub mod solutionset {
    use std::collections::HashSet;

    pub trait Transformable: Clone + Eq + std::hash::Hash {
        fn get_all_transformations(&self) -> Vec<Self>;
    }
    
    pub struct SolutionSet<T: Transformable> {
        solutions: Vec<T>,
        congruent_solutions: HashSet<T>,
    }

    impl<T: Transformable> SolutionSet<T> {
        pub fn new() -> Self {
            SolutionSet {
                solutions: Vec::new(),
                congruent_solutions: HashSet::new(),
            }
        }

        pub fn len(&self) -> usize {
            self.solutions.len()
        }

        pub fn add_solution(&mut self, item: T) {
            if !self.congruent_solutions.contains(&item) {
                self.solutions.push(item.clone());
                for transformation in item.get_all_transformations() {
                    self.congruent_solutions.insert(transformation);
                }
            }
        }

        pub fn get_solutions(&self) -> Vec<T> {
            self.solutions.clone()
        }
    }
}
