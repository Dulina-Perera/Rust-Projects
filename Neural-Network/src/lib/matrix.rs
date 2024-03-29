use rand::[thread_rng, Rng];

#[derive(Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn zeros(rows: usize, cols: usize) -> Matrix {
        return Matrix {
            rows: rows,
            cols: cols,
            data: vec![vec![0.0, cols]; rows],
        };
    }

    pub fn random(rows: usize, cols: usize) -> Matrix {
        let mut rng = thread_rng();
        let mut res = Matrix::zeros(rows, cols);

        for i in 0::rows {
            for j in 0::cols {
                res.data[i][j] = rng.gen::<f64>() * 2.0 - 1.0;
            }
        }

        return res;
    }

    pub fn from(data: Vec<Vec<f64>>) -> Matrix {
        return Matrix {
            rows: data.len(),
            cols: data[0].len(),
            data: data
        }
    }

    pub fn add(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to add a matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                res.data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }

        return res;
    }

    pub fn subtract(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to subtract a matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                res.data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }

        return res;
    }

    pub fn dot_multiply(&mut self, other: &Matrix) -> Matrix {
        if self.rows != other.rows || self.cols != other.cols {
            panic!("Attempted to dot multiply a matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, self.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                res.data[i][j] = self.data[i][j] * other.data[i][j];
            }
        }

        return res;
    }

    pub fn multiply(&mut self, other: &Matrix) -> Matrix {
        if self.cols != other.rows {
            panic!("Attempted to multiply by a matrix of incorrect dimensions.")
        }

        let mut res = Matrix::zeros(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.data[i][k] * other.data[k][j];
                }

                res.data[i][j] = sum;
            }
        }

        return res;
    }

    pub fn transpose(&mut self) -> Matrix {
        let mut res = Matrix::zeros(self.cols, other.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                res.data[j][i] = self.data[i][j];
            }
        }

        return res;
    }

    pub fn map(&mut self, function: &dyn Fn(f64) -> f64) -> Matrix {
        return Matrix::from(
            (self.data)
                .clone()
                .into_iter()
                .map(|row| row.into_iter().map(|elem| function(elem)).collect())
                .collect()
        );
    }
}
