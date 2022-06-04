pub fn trunc_towards_neg_inf(n: i32, chunk_size: i32) -> i32 {
    if n >= 0 {
        n / chunk_size
    } else {
        (n + 1) / chunk_size - 1
    }
}

pub fn trunc_towards_neg_inf_f(x: f32) -> f32 {
    f32::floor(x)
}

#[cfg(test)]
mod tests {
    use super::*;
    /// half and full chunk sizes
    const SIZE_X: usize = 16;
    const SIZE_Y: usize = 4;
    const SIZE_Z: usize = 16;
    const H_X: i32 = SIZE_X as i32 / 2;
    const H_Y: i32 = SIZE_Y as i32 / 2;
    const H_Z: i32 = SIZE_Z as i32 / 2;
    const F_X: i32 = SIZE_X as i32;
    const F_Y: i32 = SIZE_Y as i32;
    const F_Z: i32 = SIZE_Z as i32;
    #[test]
    fn trunc_i() {
        assert_eq!(trunc_towards_neg_inf(0, 5), 0);
        assert_eq!(trunc_towards_neg_inf(1, 5), 0);
        assert_eq!(trunc_towards_neg_inf(2, 5), 0);
        assert_eq!(trunc_towards_neg_inf(3, 5), 0);
        assert_eq!(trunc_towards_neg_inf(4, 5), 0);
        assert_eq!(trunc_towards_neg_inf(5, 5), 1);
        assert_eq!(trunc_towards_neg_inf(-1, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-2, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-3, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-4, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-5, 5), -1);
        assert_eq!(trunc_towards_neg_inf(-6, 5), -2);
    }

    #[test]
    fn trunc_f() {
        assert_eq!(trunc_towards_neg_inf_f(0.0), 0.0);
        assert_eq!(trunc_towards_neg_inf_f(0.4), 0.0);
        assert_eq!(trunc_towards_neg_inf_f(0.8), 0.0);
        assert_eq!(trunc_towards_neg_inf_f(1.0), 1.0);
        assert_eq!(trunc_towards_neg_inf_f(1.2), 1.0);
        assert_eq!(trunc_towards_neg_inf_f(-0.2), -1.0);
        assert_eq!(trunc_towards_neg_inf_f(-0.8), -1.0);
        assert_eq!(trunc_towards_neg_inf_f(-1.0), -1.0);
        assert_eq!(trunc_towards_neg_inf_f(-1.2), -2.0);
    }
}