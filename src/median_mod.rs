pub mod median_mod {
    /**
     * Find Median of a Vector of f64 numbers
     */
    pub fn median(v: &mut Vec<f64>) -> Option<f64> {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = v.len() / 2;

        match mid > 0 {
            true => match mid % 2 {
                0 => Some((v[mid - 1] + v[mid]) / 2.0),
                1 => Some(v[mid]),
                _ => None,
            },
            false => None,
        }
    }
}

#[test]
fn empty_test() {
    let mut input = vec![];
    let res = median_mod::median(&mut input);
    assert_eq!(res, None)
}

#[test]
fn unsorted_even_collection() {
    let mut input = vec![23.0, 2.0, 34.22, 98.8];
    let res = median_mod::median(&mut input).unwrap();
    assert_eq!(res, 28.61);
}

#[test]
fn unsorted_odd_collection() {
    let mut input = vec![1.0, 23.8, 2.3, 23.0, 2.0, 34.22, 98.8];
    let res = median_mod::median(&mut input).unwrap();
    assert_eq!(res, 23.0);
}
