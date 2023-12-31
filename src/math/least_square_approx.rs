/// Least Square Approximation <p>
/// Function that returns a polynomial which very closely passes through the given points (in 2D)
///
/// The result is made of coeficients, in descending order (from x^degree to free term)
///
/// Parameters:
///
/// points -> coordinates of given points
///
/// degree -> degree of the polynomial
///
pub fn least_square_approx(points: &[(f64, f64)], degree: i32) -> Vec<f64> {
    use nalgebra::{DMatrix, DVector};

    /* Used for rounding floating numbers */
    fn round_to_decimals(value: f64, decimals: u32) -> f64 {
        let multiplier = 10f64.powi(decimals as i32);
        (value * multiplier).round() / multiplier
    }

    /* Computes the sums in the system of equations */
    let mut sums = Vec::<f64>::new();
    for i in 1..=(2 * degree + 1) {
        sums.push(points.iter().map(|(x, _)| x.powi(i - 1)).sum());
    }

    let mut free_col = Vec::<f64>::new();
    /* Compute the free terms column vector */
    for i in 1..=(degree + 1) {
        free_col.push(points.iter().map(|(x, y)| y * (x.powi(i - 1))).sum());
    }
    let b = DVector::from_row_slice(&free_col);

    let size = (degree + 1) as usize;
    /* Create and fill the system's matrix */
    let a = DMatrix::from_fn(size, size, |i, j| sums[i + j]);

    /* Solve the system of equations: A * x = b */
    match a.qr().solve(&b) {
        Some(x) => {
            let mut rez: Vec<f64> = x.iter().map(|x| round_to_decimals(*x, 5)).collect();
            rez.reverse();
            rez
        }
        None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_points_1st_degree() {
        let points = vec![
            (5.3, 7.8),
            (4.9, 8.1),
            (6.1, 6.9),
            (4.7, 8.3),
            (6.5, 7.7),
            (5.6, 7.0),
            (5.8, 8.2),
            (4.5, 8.0),
            (6.3, 7.2),
            (5.1, 8.4),
        ];

        assert_eq!(least_square_approx(&points, 1), [-0.49069, 10.44898]);
    }

    #[test]
    fn eight_points_5th_degree() {
        let points = vec![
            (4f64, 8f64),
            (8f64, 2f64),
            (1f64, 7f64),
            (10f64, 3f64),
            (11.0, 0.0),
            (7.0, 3.0),
            (10.0, 1.0),
            (13.0, 13.0),
        ];

        assert_eq!(
            least_square_approx(&points, 5),
            [0.00603, -0.21304, 2.79929, -16.53468, 40.29473, -19.35771]
        );
    }

    #[test]
    fn four_points_2nd_degree() {
        let points = vec![
            (2.312, 8.345344),
            (-2.312, 8.345344),
            (-0.7051, 3.49716601),
            (0.7051, 3.49716601),
        ];

        assert_eq!(least_square_approx(&points, 2), [1.0, 0.0, 3.0]);
    }
}
