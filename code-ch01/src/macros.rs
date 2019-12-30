macro_rules! assert_true {
    ($cond: expr) => {
        assert!($cond);
    }
}

macro_rules! assert_false {
    ($cond: expr) => {
        assert!(!$cond);
    }
}

