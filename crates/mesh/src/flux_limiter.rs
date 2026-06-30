pub trait FluxLimiter {
    fn limiter(r: f64) -> f64;
}

#[derive(Debug, Clone, Copy)]
pub struct MinmodLimiter;

impl FluxLimiter for MinmodLimiter {
    fn limiter(r: f64) -> f64 {
        if r <= 0.0 {
            0.0
        } else {
            r.min(1.0)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VanLeerLimiter;

impl FluxLimiter for VanLeerLimiter {
    fn limiter(r: f64) -> f64 {
        if r <= 0.0 {
            0.0
        } else {
            (2.0 * r) / (1.0 + r)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SuperbeeLimiter;

impl FluxLimiter for SuperbeeLimiter {
    fn limiter(r: f64) -> f64 {
        if r <= 0.0 {
            0.0
        } else {
            (2.0 * r)
                .min(1.0)
                .max(r.min(2.0))
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MonotonizedCentralLimiter;

impl FluxLimiter for MonotonizedCentralLimiter {
    fn limiter(r: f64) -> f64 {
        if r <= 0.0 {
            0.0
        } else {
            ((1.0 + r) / 2.0)
                .min(2.0)
                .min(2.0 * r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minmod_limits_correctly() {
        assert_eq!(MinmodLimiter::limiter(-1.0), 0.0);
        assert_eq!(MinmodLimiter::limiter(0.5), 0.5);
        assert_eq!(MinmodLimiter::limiter(2.0), 1.0);
    }

    #[test]
    fn vanleer_limits_correctly() {
        assert_eq!(VanLeerLimiter::limiter(-1.0), 0.0);

        let value = VanLeerLimiter::limiter(1.0);

        assert!((value - 1.0).abs() < 1e-12);
    }

    #[test]
    fn superbee_limits_correctly() {
        assert_eq!(SuperbeeLimiter::limiter(-1.0), 0.0);

        let value = SuperbeeLimiter::limiter(2.0);

        assert!((value - 2.0).abs() < 1e-12);
    }

    #[test]
    fn mc_limits_correctly() {
        assert_eq!(MonotonizedCentralLimiter::limiter(-1.0), 0.0);

        let value = MonotonizedCentralLimiter::limiter(1.0);

        assert!((value - 1.0).abs() < 1e-12);
    }
}