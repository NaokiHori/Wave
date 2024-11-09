pub fn get_coordinate(length: f64, nitems: usize, index: usize) -> f64 {
    length * ((index as f64 + 0.5f64) / nitems as f64)
}

#[cfg(test)]
mod test {
    use super::get_coordinate;
    #[test]
    fn check() {
        let length = 1f64;
        let nitems = 4usize;
        assert!(0.125 == get_coordinate(length, nitems, 0));
        assert!(0.375 == get_coordinate(length, nitems, 1));
        assert!(0.625 == get_coordinate(length, nitems, 2));
        assert!(0.875 == get_coordinate(length, nitems, 3));
    }
}
