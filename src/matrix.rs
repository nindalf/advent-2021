pub struct Matrix {
    storage: Vec<u32>,
    pub max_x: usize,
    pub max_y: usize,
}

impl Matrix {
    #[allow(dead_code)]
    pub fn new(input: &str) -> Option<Matrix> {
        let storage = input
            .split("")
            .filter_map(|d| d.parse::<u32>().ok())
            .collect();
        let max_x = input
            .split_ascii_whitespace()
            .next()?
            .split("")
            .filter_map(|d| d.parse::<u32>().ok())
            .count();
        let max_y = input.split_ascii_whitespace().count();
        Some(Matrix {
            storage,
            max_x,
            max_y,
        })
    }

    pub fn all_points(&self) -> Vec<(usize, usize)> {
        (0..self.max_x)
            .flat_map(|x| std::iter::repeat(x).zip(0..self.max_y))
            .collect()
    }

    pub fn value(&self, x: usize, y: usize) -> Option<&u32> {
        self.storage.get(y * self.max_x + x)
    }
}
