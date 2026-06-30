pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

pub fn norm(v: &[f64]) -> f64 {
    dot(v, v).sqrt()
}

pub fn axpy(
    alpha: f64,
    x: &[f64],
    y: &mut [f64],
) {
    assert_eq!(x.len(), y.len());

    for (yi, xi) in y.iter_mut().zip(x.iter()) {
        *yi += alpha * xi;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];

        assert_eq!(dot(&a, &b), 32.0);
    }

    #[test]
    fn vector_norm() {
        let v = vec![3.0, 4.0];

        assert_eq!(norm(&v), 5.0);
    }

    #[test]
    fn axpy_operation() {
        let x = vec![1.0, 2.0];
        let mut y = vec![3.0, 4.0];

        axpy(2.0, &x, &mut y);

        assert_eq!(y, vec![5.0, 8.0]);
    }
}