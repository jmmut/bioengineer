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
