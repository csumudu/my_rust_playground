pub mod unique_mod {
    pub fn unique(col: &Vec<i32>) -> Vec<i32> {
        let mut unq: Vec<i32> = Vec::new();
        col.iter().fold(unq, |mut acc, cuu| {
            if acc.contains(cuu) {
                acc
            } else {
                acc.push(*cuu);
                acc
            }
        })
    }

    pub fn unique2<T: Ord + std::clone::Clone>(col: &Vec<T>) -> Vec<T> {
        let mut d = col.clone();
        d.sort();
        d.dedup();
        d
    }
}

#[test]
fn test_unique() {
    let input = vec![1, 1, 2];
    let res = unique_mod::unique(&input);
    assert_eq!(vec![1, 2], res)
}

#[test]
fn test_no_duplicates() {
    let input = vec![1, 4, 2];
    let res = unique_mod::unique(&input);
    assert_eq!(vec![1, 4, 2], res)
}
#[test]
fn test_unsorted_duplicates() {
    let input = vec![1, 4, 2, 1, 5, 6, 8, 3, 3, 3, 6, 7, 8, 8];
    let res = unique_mod::unique(&input);
    assert_eq!(vec![1, 4, 2, 5, 6, 8, 3, 7], res)
}
