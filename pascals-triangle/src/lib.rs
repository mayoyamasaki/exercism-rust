pub struct PascalsTriangle {
    height: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            height: row_count
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut triangle: Vec<Vec<u32>> = Vec::new();
        for i in 0..self.height {
            let mut stage: Vec<u32> = Vec::new();
            for j in 0..i+1 {
                let v = self.get_value(i as usize, j as usize, &triangle);
                stage.push(v);
            }
            triangle.push(stage);
        }
        triangle
    }

    fn get_value(&self, i: usize, j:usize, triangle: &Vec<Vec<u32>>) -> u32 {
        if j == 0 || j == i {
            1
        } else {
            triangle[i-1][j-1] + triangle[i-1][j]
        }
    }
}
