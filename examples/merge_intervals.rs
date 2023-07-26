/// https://leetcode.com/problems/merge-intervals/
/// T: Sorting intervals == O(n * log(n)), merging intervals == O(n)
/// S:  Merge sort == O(n), merge intervals == O(n)
pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if intervals.len() == 0 {
        return vec![];
    }
    let sorted_intervals = {
        let mut sorted_intervals = intervals;
        sorted_intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        sorted_intervals
    };
    // println!("{:?}", sorted_intervals);
    let mut merged_intervals = vec![];
    let mut n = 0usize;
    let mut k = 1usize;
    let mut start = sorted_intervals[n][0];
    let mut end = sorted_intervals[n][1];
    loop {
        if n + k >= sorted_intervals.len() {
            break;
        }
        if sorted_intervals[n + k][0] <= end {
            end = end.max(sorted_intervals[n + k][1]);
            k += 1;
        } else {
            merged_intervals.push(vec![start, end]);
            n += k;
            k = 1;
            start = sorted_intervals[n][0];
            end = sorted_intervals[n][1];
        }
    }
    merged_intervals.push(vec![start, end]);
    merged_intervals
}

#[test]
fn simple() {
    assert_eq!(merge(vec![]), Vec::<Vec<i32>>::new());
    assert_eq!(
        merge(vec![vec![1, 3], vec![3, 10], vec![10, 18]]),
        vec![[1, 18]]
    );
    assert_eq!(
        merge(vec![
            vec![1, 7],
            vec![3, 4],
            vec![4, 5],
            vec![9, 11],
            vec![-2, -1]
        ]),
        vec![vec![-2, -1], vec![1, 7], vec![9, 11]]
    );
    assert_eq!(
        merge(vec![
            vec![1, 3],
            vec![8, 10],
            vec![15, 18],
            vec![2, 6],
            vec![-2, -1]
        ]),
        vec![vec![-2, -1], vec![1, 6], vec![8, 10], vec![15, 18]]
    );
    assert_eq!(merge(vec![vec![-1, 4]]), vec![vec![-1, 4]]);
    assert_eq!(
        merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![vec![1, 6], vec![8, 10], vec![15, 18]]
    );
    assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), vec![vec![1, 5]]);
}

fn main() {}
