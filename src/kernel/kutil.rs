pub fn range(lo: uint, hi: uint, it: &fn(uint)) {
    let mut iter = lo;
    while iter < hi {
        it(iter);
        iter += 1;
    }
}