/// Given a set of integers, generate its power set
pub fn power_set_of(set: Vec<i32>) -> Vec<Vec<i32>> {
    // Space complexity is also TwoToN because 2^N subsets + max N clones at a time == 2^N + N.
    fn power_set_recr(ordered_set: &Vec<i32>, index: usize, parent: &Vec<i32>) -> Vec<Vec<i32>> {
        if index == ordered_set.len() {
            return vec![parent.clone()];
        }

        let left = parent.clone();
        let left_subsets = power_set_recr(ordered_set, index + 1, &left);

        let right = {
            let mut clone = parent.clone();
            clone.push(ordered_set[index]);
            clone
        };
        let right_subsets = power_set_recr(ordered_set, index + 1, &right);

        return left_subsets
            .into_iter()
            .chain(right_subsets.into_iter())
            .collect();
    }

    power_set_recr(&set.into_iter().collect(), 0, &vec![])
}

#[test]
fn simple() {
    assert_eq!(power_set_of(vec![]), vec![vec![]]);
    assert_eq!(power_set_of(vec![-1]), vec![vec![], vec![-1]]);
    assert_eq!(power_set_of(vec![3]), vec![vec![], vec![3]]);
    assert_eq!(
        power_set_of(vec![1, 2]),
        vec![vec![], vec![2], vec![1], vec![1, 2],]
    );
    assert_eq!(
        power_set_of(vec![1, 2, 3]),
        vec![
            vec![],
            vec![3],
            vec![2],
            vec![2, 3],
            vec![1],
            vec![1, 3],
            vec![1, 2],
            vec![1, 2, 3]
        ]
    );
}
