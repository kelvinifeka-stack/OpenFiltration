use crate::LinearSystem;

pub struct ConjugateGradient;

impl ConjugateGradient {
    pub fn solve(
        system: &LinearSystem,
        tolerance: f64,
        max_iterations: usize,
    ) -> Vec<f64> {
        let n = system.rhs().len();

        let mut x = vec![0.0; n];

        let mut r = system.rhs().to_vec();

        let mut p = r.clone();

        let mut rs_old = dot(&r, &r);

        for _ in 0..max_iterations {

            let ap = system.matrix().multiply(&p);

            let alpha = rs_old / dot(&p, &ap);

            for i in 0..n {
                x[i] += alpha * p[i];
            }

            for i in 0..n {
                r[i] -= alpha * ap[i];
            }

            let rs_new = dot(&r, &r);

            if rs_new.sqrt() < tolerance {
                break;
            }

            let beta = rs_new / rs_old;

            for i in 0..n {
                p[i] = r[i] + beta * p[i];
            }

            rs_old = rs_new;
        }

        x
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| x * y)
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::LinearSystem;

    #[test]
    fn solve_identity_system() {

        let mut system = LinearSystem::new(3);

        system.matrix_mut().add(0,0,1.0);
        system.matrix_mut().add(1,1,1.0);
        system.matrix_mut().add(2,2,1.0);

        system.matrix_mut().finalize();

        system.rhs_mut()[0] = 4.0;
        system.rhs_mut()[1] = 5.0;
        system.rhs_mut()[2] = 6.0;

        let x = ConjugateGradient::solve(
            &system,
            1e-12,
            20,
        );

        assert!((x[0]-4.0).abs() < 1e-10);
        assert!((x[1]-5.0).abs() < 1e-10);
        assert!((x[2]-6.0).abs() < 1e-10);
    }
}