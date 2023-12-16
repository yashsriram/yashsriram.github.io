/// https://codeforces.com/problemset/problem/1374/C
/// - An input with given constrains shall have a zero final sum.
/// - An input that is a regular bracket sequence (rbs), shall have a non-negative sum-from-start at any index.
/// - This serves as a verification method.
/// - To make non-rbs an rbs, one way is to somehow make all negative sum-from-start be non-negative.
/// - With the permitted operations, this can be done by doing, for each bracket with negative sum-from-start
///     - `(` => move to the beggining
///     - or
///     - `)` => move to the end
/// - With each move, the min sum-from-start increases by one
/// - Min number of moves to make min(sum-from-start) non-negative is abs(min(sum-from-start)).
/// - Hence proved.
fn min_moves_for_regular_bracket_sequence(brackets: &str) -> Option<u32> {
    let mut lowest_sum = 0i32;
    let mut sum = 0i32;
    for bracket in brackets.chars() {
        if bracket == '(' {
            sum += 1;
        } else if bracket == ')' {
            sum -= 1;
        } else {
            return None;
        }

        if sum < lowest_sum {
            lowest_sum = sum;
        }
    }
    Some(lowest_sum.abs() as u32)
}

#[test]
fn simple2() {
    assert_eq!(min_moves_for_regular_bracket_sequence("()"), Some(0));
    assert_eq!(min_moves_for_regular_bracket_sequence("()"), Some(0));
    assert_eq!(min_moves_for_regular_bracket_sequence("()"), Some(0));
    assert_eq!(min_moves_for_regular_bracket_sequence(")("), Some(1));
    assert_eq!(min_moves_for_regular_bracket_sequence("()()"), Some(0));
    assert_eq!(min_moves_for_regular_bracket_sequence("(())"), Some(0));
    assert_eq!(min_moves_for_regular_bracket_sequence("))(("), Some(2));
    assert_eq!(min_moves_for_regular_bracket_sequence(")()("), Some(1));
    assert_eq!(min_moves_for_regular_bracket_sequence("())()()("), Some(1));
    assert_eq!(
        min_moves_for_regular_bracket_sequence(")))((((())"),
        Some(3),
    );
}

/// https://codeforces.com/problemset/problem/1375/B
mod neighbour_grid {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Grid {
        num_rows: usize,
        num_cols: usize,
        row_major_values: Vec<u32>,
    }

    impl Grid {
        fn new(num_rows: usize, num_cols: usize, row_major_values: &Vec<u32>) -> Option<Grid> {
            if num_rows * num_cols == row_major_values.len() {
                Some(Grid {
                    num_rows,
                    num_cols,
                    row_major_values: row_major_values.to_vec(),
                })
            } else {
                None
            }
        }

        // The loop statement in good_version() breaks <==> the grid is good
        // Why greedy method works?
        fn good_version(&self) -> Option<Grid> {
            fn situation_at(grid: &Grid, r: usize, c: usize) -> (u32, u32) {
                // If val is 0, no further check needed => early return with zeros
                if grid[(r, c)] == 0 {
                    return (0, 0);
                }
                let mut positive_neighbour_count = 0u32;
                if r > 0 && grid[(r - 1, c)] > 0 {
                    positive_neighbour_count += 1;
                }
                if r < grid.num_rows - 1 && grid[(r + 1, c)] > 0 {
                    positive_neighbour_count += 1;
                }
                if c > 0 && grid[(r, c - 1)] > 0 {
                    positive_neighbour_count += 1;
                }
                if c < grid.num_cols - 1 && grid[(r, c + 1)] > 0 {
                    positive_neighbour_count += 1;
                }
                (grid[(r, c)], positive_neighbour_count)
            }

            fn try_increment_neighbours(
                grid: &mut Grid,
                r: usize,
                c: usize,
                remaining: u32,
            ) -> Option<u32> {
                let mut remaining = remaining;
                if remaining > 0 && r > 0 && grid[(r - 1, c)] == 0 {
                    grid[(r - 1, c)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && r < grid.num_rows - 1 && grid[(r + 1, c)] == 0 {
                    grid[(r + 1, c)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && c > 0 && grid[(r, c - 1)] == 0 {
                    grid[(r, c - 1)] += 1;
                    remaining -= 1;
                }
                if remaining > 0 && c < grid.num_cols - 1 && grid[(r, c + 1)] == 0 {
                    grid[(r, c + 1)] += 1;
                    remaining -= 1;
                }
                match remaining {
                    0 => Some(0),
                    _ => None,
                }
            }

            let mut good_version = self.clone();

            loop {
                let mut still_bad_version = false;
                for r in 0..good_version.num_rows {
                    for c in 0..good_version.num_cols {
                        let (val, num_positive_neighbours) = situation_at(&good_version, r, c);
                        if val == 0 {
                            continue;
                        } else if val < num_positive_neighbours {
                            good_version[(r, c)] += num_positive_neighbours - val;
                        } else if val > num_positive_neighbours {
                            try_increment_neighbours(
                                &mut good_version,
                                r,
                                c,
                                val - num_positive_neighbours,
                            )?;
                            still_bad_version = true;
                        }
                    }
                }
                if !still_bad_version {
                    break;
                }
            }

            Some(good_version)
        }
    }

    use std::ops::{Index, IndexMut};
    impl Index<(usize, usize)> for Grid {
        type Output = u32;

        fn index(&self, coordinate: (usize, usize)) -> &Self::Output {
            let (r, c) = coordinate;
            &self.row_major_values[r * self.num_cols + c]
        }
    }

    impl IndexMut<(usize, usize)> for Grid {
        fn index_mut(&mut self, coordinate: (usize, usize)) -> &mut Self::Output {
            let (r, c) = coordinate;
            &mut self.row_major_values[r * self.num_cols + c]
        }
    }

    use std::fmt;
    impl fmt::Display for Grid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for i in 0..self.num_rows {
                for j in 0..self.num_cols {
                    if j == self.num_cols - 1 {
                        write!(f, "{}", self.row_major_values[i * self.num_cols + j])?;
                    } else {
                        write!(f, "{} ", self.row_major_values[i * self.num_cols + j])?;
                    }
                }
                if i < self.num_rows - 1 {
                    write!(f, "\n")?;
                }
            }
            write!(f, "")
        }
    }

    fn test_using_macro(m: usize, n: usize, values: Vec<u32>) -> Option<Vec<u32>> {
        let grid = Grid::new(m, n, &values).unwrap();
        let good_version = grid.good_version();
        good_version.and_then(|good_version| Some(good_version.row_major_values))
    }

    #[test]
    fn simple() {
        assert_eq!(
            test_using_macro(3, 4, vec![0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0])
        );
        assert_eq!(
            test_using_macro(2, 2, vec![0, 0, 0, 0]),
            Some(vec![0, 0, 0, 0])
        );
        assert_eq!(
            test_using_macro(4, 4, vec![0, 0, 0, 0, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0]),
            Some(vec![0, 1, 0, 1, 0, 2, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0])
        );
        assert_eq!(test_using_macro(1, 1, vec![0]), Some(vec![0]));
        assert_eq!(test_using_macro(1, 2, vec![1, 0]), Some(vec![1, 1]));
        assert_eq!(
            test_using_macro(4, 3, vec![0, 0, 0, 0, 4, 2, 0, 3, 0, 0, 0, 0]),
            Some(vec![0, 2, 2, 2, 4, 2, 2, 3, 0, 0, 1, 0])
        );
        assert_eq!(test_using_macro(2, 2, vec![3, 0, 0, 0]), None);
        assert_eq!(test_using_macro(2, 3, vec![0, 0, 0, 0, 4, 0]), None);
        assert_eq!(test_using_macro(1, 1, vec![1]), None);
        assert_eq!(test_using_macro(1, 2, vec![1, 2]), None);
    }
}

//! Given a list of integers, sort them in ascending order.
mod recursive {
    fn sort_recr(original: &[i32]) -> Vec<i32> {
        if original.len() == 1 {
            return vec![original[0]];
        }

        let middle = original.len() / 2;
        let sorted_left = sort_recr(&original[..middle]);
        let sorted_right = sort_recr(&original[middle..]);

        let mut l = 0usize;
        let mut r = 0usize;
        let mut local_merge = vec![];
        loop {
            if l == sorted_left.len() && r == sorted_right.len() {
                break;
            } else if l < sorted_left.len() && r == sorted_right.len() {
                local_merge.push(sorted_left[l]);
                l += 1;
            } else if l == sorted_left.len() && r < sorted_right.len() {
                local_merge.push(sorted_right[r]);
                r += 1;
            } else {
                let left = sorted_left[l];
                let right = sorted_right[r];
                if left < right {
                    local_merge.push(left);
                    l += 1;
                } else {
                    local_merge.push(right);
                    r += 1;
                }
            }
        }
        local_merge
    }

    pub fn sort(list: Vec<i32>) -> Vec<i32> {
        sort_recr(list.as_slice())
    }

    #[test]
    fn simple() {
        assert_eq!(sort(vec![-7, -5, 3, 2]), [-7, -5, 2, 3]);
        assert_eq!(sort(vec![7, 5, 3, 2]), [2, 3, 5, 7]);
    }
}

// iterative was made in the hope that it will be faster, but apparantly it may not always
// be the case
mod iterative {
    fn sort_recr(original: &[i32]) -> Vec<i32> {
        use core::ops::Range;

        #[derive(Clone)]
        struct ElementState {
            val: i32,
            is_visited: bool,
        }

        impl ElementState {
            fn set(&mut self, val: i32) {
                self.val = val;
                self.is_visited = true;
            }
        }

        impl Default for ElementState {
            fn default() -> ElementState {
                ElementState {
                    val: 0,
                    is_visited: false,
                }
            }
        }
        let mut sorted = vec![ElementState::default(); original.len()];
        // Stack collects ranges.
        // If original.len() == 8, at max it shall have
        // [ 0..8, 0..4, 4..8, 4..6, 6..8, 6..7, 7..8 ]
        // Therefore the following capcity
        let mut stack = Vec::<Range<usize>>::with_capacity(
            1 + 2 * ((original.len() as f32).log2().ceil() as usize),
        );
        stack.push(0..original.len());
        // Used for merging two sorted arrays. Avoids allocating space log(n) times.
        // Required to allocated because cannot do merge with O(1) space and O(n) time.
        let mut temporary_merge_space = vec![0i32; original.len()];
        while let Some(current_range) = stack.pop() {
            if current_range.len() == 1 {
                let idx = current_range.start;
                sorted[idx].set(original[idx]);
                continue;
            }
            if sorted[current_range.start].is_visited {
                // Range start is visited <=> Range is visited
                // The two halves are sorted, merge them
                let Range { start, end } = current_range;
                let middle = start + current_range.len() / 2;

                // temporary_merge_space.clear();
                let mut tmp_i = 0usize;
                let mut l = start;
                let mut r = middle;
                loop {
                    if l == middle && r == end {
                        break;
                    } else if l < middle && r == end {
                        temporary_merge_space[tmp_i] = sorted[l].val;
                        tmp_i += 1;
                        l += 1;
                    } else if l == middle && r < end {
                        temporary_merge_space[tmp_i] = sorted[r].val;
                        tmp_i += 1;
                        r += 1;
                    } else {
                        let left = sorted[l].val;
                        let right = sorted[r].val;
                        if left < right {
                            temporary_merge_space[tmp_i] = left;
                            tmp_i += 1;
                            l += 1;
                        } else {
                            temporary_merge_space[tmp_i] = right;
                            tmp_i += 1;
                            r += 1;
                        }
                    }
                }
                for (i, idx) in current_range.enumerate() {
                    sorted[idx].val = temporary_merge_space[i];
                }
            } else {
                // Range start is unvisited <=> Range is unvisited
                // Split into two
                let Range { start, end } = current_range;
                let middle = start + current_range.len() / 2;
                stack.push(current_range);
                stack.push(start..middle);
                stack.push(middle..end);
            }
        }
        sorted.into_iter().map(|x| x.val).collect()
    }

    pub fn sort(list: Vec<i32>) -> Vec<i32> {
        sort_recr(list.as_slice())
    }

    #[test]
    fn simple() {
        assert_eq!(sort(vec![-7, -5, 3, 2]), [-7, -5, 2, 3]);
        assert_eq!(sort(vec![7, 5, 3, 2]), [2, 3, 5, 7]);
    }
}

/// https://leetcode.com/problems/fibonacci-number/
mod fibonacci_number {
    #[test]
    fn simple() {
        assert_eq!(fib_exp(2), 1);
        assert_eq!(fib_exp(3), 2);
        assert_eq!(fib_exp(4), 3);
        assert_eq!(fib_exp(46), 1836311903);

        assert_eq!(fib_bottom_up(2), 1);
        assert_eq!(fib_bottom_up(3), 2);
        assert_eq!(fib_bottom_up(4), 3);
        assert_eq!(fib_bottom_up(46), 1836311903);
        assert_eq!(fib_top_down(2), 1);

        assert_eq!(fib_top_down(3), 2);
        assert_eq!(fib_top_down(4), 3);
        assert_eq!(fib_top_down(46), 1836311903);
    }
    pub fn fib_exp(n: i32) -> i32 {
        match n {
            0 => 0,
            1 => 1,
            _ => fib_exp(n - 1) + fib_exp(n - 2),
        }
    }
    pub fn fib_bottom_up(n: i32) -> i32 {
        let n = n as usize;
        let mut fib_at = vec![0; n + 1];
        fib_at[0] = 0;
        fib_at[1] = 1;
        for i in 2..(n + 1) {
            fib_at[i] = fib_at[i - 1] + fib_at[i - 2];
        }
        fib_at[n]
    }
    pub fn fib_top_down(n: i32) -> i32 {
        let n = n as usize;
        fn top_down(fib_at: &mut Vec<Option<i32>>, n: usize) -> i32 {
            if let Some(ans) = fib_at[n] {
                return ans;
            }
            if n == 0 {
                fib_at[n] = Some(0);
                return 0;
            }
            if n == 1 {
                fib_at[n] = Some(1);
                return 1;
            }
            let number = match n {
                0 => 0,
                1 => 1,
                _ => top_down(fib_at, n - 1) + top_down(fib_at, n - 2),
            };
            fib_at[n] = Some(number);
            number
        }
        let mut fib_at = vec![None; n + 1];
        top_down(&mut fib_at, n)
    }
}

mod subarr {
    /// Given an array, abs it elements, take a subarr, sum it. what is max sum you can get that way?
    fn max_abs_subarr_sum(nums: Vec<i32>) -> i32 {
        nums.iter().map(|&e| e.abs()).sum()
    }

    #[test]
    fn test_max_abs_subarr_sum() {
        assert_eq!(max_abs_subarr_sum(vec![5, 4, -1, 7, 8]), 25);
        assert_eq!(max_abs_subarr_sum(vec![1]), 1);
        assert_eq!(max_abs_subarr_sum(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 23);
    }

    /// https://leetcode.com/problems/maximum-subarray/
    pub fn max_sub_array_kadane(nums: Vec<i32>) -> i32 {
        let mut ans = nums[0];
        let mut prefix = nums[0];
        for i in 1..nums.len() {
            if prefix < 0 {
                prefix = 0;
            }
            let num = nums[i];
            prefix += num;
            ans = ans.max(prefix);
        }
        ans
    }

    #[test]
    fn test_max_sub_array_kadane() {
        assert_eq!(max_sub_array_kadane(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(max_sub_array_kadane(vec![1]), 1);
        assert_eq!(max_sub_array_kadane(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }

    /// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        fn max_sub_array(nums: &Vec<i32>) -> i32 {
            let mut ans = nums[0];
            let mut prefix = nums[0];
            for i in 1..nums.len() {
                if prefix < 0 {
                    prefix = 0;
                }
                let num = nums[i];
                prefix += num;
                ans = ans.max(prefix);
            }
            ans
        }

        let max1 = max_sub_array(&nums);
        let negated = nums.iter().map(|&x| -x).collect::<Vec<i32>>();
        let max2 = max_sub_array(&negated);
        max1.max(max2)
    }

    #[test]
    fn test_max_absolute_sum() {
        assert_eq!(max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
        assert_eq!(max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
    }
}

mod subseq {
    /// https://leetcode.com/problems/is-subsequence/
    pub fn is_subsequence(s: &str, t: &str) -> bool {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut s_idx = 0usize;
        for &c in t.iter() {
            if s_idx == s.len() {
                return true;
            }
            if s[s_idx] == c {
                s_idx += 1;
            }
        }
        if s_idx == s.len() {
            true
        } else {
            false
        }
    }

    #[test]
    fn test_is_subsequence() {
        assert_eq!(is_subsequence("abc", "ahbgdc"), true);
        assert_eq!(is_subsequence("a", "bcd"), false);
        assert_eq!(is_subsequence("abc", "ab"), false);
        assert_eq!(is_subsequence("axc", "ahbgdc"), false);
    }

    /// https://leetcode.com/problems/longest-common-subsequence/
    pub fn longest_common_subsequence(text1: &str, text2: &str) -> i32 {
        let t1 = text1.as_bytes();
        let t2 = text2.as_bytes();
        // LCS table
        let mut lcs = vec![vec![0u32; t2.len()]; t1.len()];
        // Fill first column
        let mut found_0 = false;
        for i in 0..t1.len() {
            if !found_0 && t1[i] == t2[0] {
                found_0 = true;
            }
            if !found_0 {
                lcs[i][0] = 0;
            } else {
                lcs[i][0] = 1;
            }
        }
        // Fill first row
        let mut found_0 = false;
        for i in 0..t2.len() {
            if !found_0 && t2[i] == t1[0] {
                found_0 = true;
            }
            if !found_0 {
                lcs[0][i] = 0;
            } else {
                lcs[0][i] = 1;
            }
        }
        // Fill the rest
        for i in 1..t1.len() {
            for j in 1..t2.len() {
                if t1[i] == t2[j] {
                    lcs[i][j] = lcs[i - 1][j - 1] + 1;
                } else {
                    lcs[i][j] = lcs[i - 1][j].max(lcs[i][j - 1]);
                }
            }
        }
        lcs[t1.len() - 1][t2.len() - 1] as i32
    }

    #[test]
    fn test_lcs_len() {
        assert_eq!(longest_common_subsequence("abcde", "ace"), 3);
        assert_eq!(longest_common_subsequence("abc", "abc"), 3);
        assert_eq!(longest_common_subsequence("abc", "def"), 0);
    }

    /// https://leetcode.com/problems/longest-increasing-subsequence/
    mod longest_increasing_subseq {
        pub fn length_of_lis_bottom_up(nums: Vec<i32>) -> i32 {
            let mut lis_to = vec![1; nums.len()];
            for i in 1..nums.len() {
                for j in 0..i {
                    if nums[j] < nums[i] && lis_to[i] < lis_to[j] + 1 {
                        lis_to[i] = lis_to[j] + 1;
                    }
                }
            }
            *lis_to.iter().max().unwrap()
        }

        pub fn length_of_lis_self_sort_dedup_len(nums: Vec<i32>) -> i32 {
            let t1 = nums;
            let mut t2 = t1.clone();
            t2.sort();
            t2.dedup();
            // LCS table
            let mut lcs = vec![vec![0u32; t2.len()]; t1.len()];
            // Fill first column
            let mut found_0 = false;
            for i in 0..t1.len() {
                if !found_0 && t1[i] == t2[0] {
                    found_0 = true;
                }
                if !found_0 {
                    lcs[i][0] = 0;
                } else {
                    lcs[i][0] = 1;
                }
            }
            // Fill first row
            let mut found_0 = false;
            for i in 0..t2.len() {
                if !found_0 && t2[i] == t1[0] {
                    found_0 = true;
                }
                if !found_0 {
                    lcs[0][i] = 0;
                } else {
                    lcs[0][i] = 1;
                }
            }
            // Fill the rest
            for i in 1..t1.len() {
                for j in 1..t2.len() {
                    if t1[i] == t2[j] {
                        lcs[i][j] = lcs[i - 1][j - 1] + 1;
                    } else {
                        lcs[i][j] = lcs[i - 1][j].max(lcs[i][j - 1]);
                    }
                }
            }
            lcs[t1.len() - 1][t2.len() - 1] as i32
        }

        #[test]
        fn test_lis_len() {
            assert_eq!(length_of_lis_bottom_up(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
            assert_eq!(length_of_lis_bottom_up(vec![0, 1, 0, 3, 2, 3]), 4);
            assert_eq!(length_of_lis_bottom_up(vec![7, 7, 7, 7, 7, 7, 7]), 1);

            assert_eq!(
                length_of_lis_self_sort_dedup_len(vec![10, 9, 2, 5, 3, 7, 101, 18]),
                4
            );
            assert_eq!(length_of_lis_self_sort_dedup_len(vec![0, 1, 0, 3, 2, 3]), 4);
            assert_eq!(
                length_of_lis_self_sort_dedup_len(vec![7, 7, 7, 7, 7, 7, 7]),
                1
            );
        }
    }

    /// https://leetcode.com/problems/longest-palindromic-subsequence/
    pub fn longest_palindrome_subseq(s: &str) -> i32 {
        let t1 = s.as_bytes();
        let t2 = s.chars().rev().collect::<String>();
        let t2 = t2.as_bytes();
        // LCS table
        let mut lcs = vec![vec![0u32; t2.len()]; t1.len()];
        // Fill first column
        let mut found_0 = false;
        for i in 0..t1.len() {
            if !found_0 && t1[i] == t2[0] {
                found_0 = true;
            }
            if !found_0 {
                lcs[i][0] = 0;
            } else {
                lcs[i][0] = 1;
            }
        }
        // Fill first row
        let mut found_0 = false;
        for i in 0..t2.len() {
            if !found_0 && t2[i] == t1[0] {
                found_0 = true;
            }
            if !found_0 {
                lcs[0][i] = 0;
            } else {
                lcs[0][i] = 1;
            }
        }
        // Fill the rest
        for i in 1..t1.len() {
            for j in 1..t2.len() {
                if t1[i] == t2[j] {
                    lcs[i][j] = lcs[i - 1][j - 1] + 1;
                } else {
                    lcs[i][j] = lcs[i - 1][j].max(lcs[i][j - 1]);
                }
            }
        }
        lcs[t1.len() - 1][t2.len() - 1] as i32
    }

    #[test]
    fn test_lps() {
        assert_eq!(longest_palindrome_subseq("bbbab"), 4);
        assert_eq!(longest_palindrome_subseq("cbbd"), 2);
    }
}

// // paths

// stralg::prob! { name(grid_path_non_neg_edges)
//     desc(url("https://leetcode.com/problems/minimum-path-sum/"))
//     corr(_impl::Solution::min_path_sum => i32) [
//         {"[[1,3,1],[1,5,1],[4,2,1]]"} "7"
//         {"[[1,2,3],[4,5,6]]"} "12"
//     ]
//     code(forward, T : MN, S : MN) {
//         mod _impl {
//             pub struct Solution;
//             impl Solution {
//                 pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//                     let mut cost_to = grid.clone();
//                     let m = grid.len();
//                     let n = grid[0].len();
//                     // (0, 0) element already initialized
//                     // First row initialization
//                     for i in 1..n {
//                         cost_to[0][i] += cost_to[0][i - 1];
//                     }
//                     // First col initialization
//                     for i in 1..m {
//                         cost_to[i][0] += cost_to[i - 1][0];
//                     }
//                     // Remaining
//                     for i in 1..m {
//                         for j in 1..n {
//                             cost_to[i][j] += cost_to[i - 1][j].min(cost_to[i][j - 1]);
//                         }
//                     }
//                     cost_to[m - 1][n - 1]
//                 }
//             }
//         }
//     }
//     code(ucs, T : MN, S : MN) {
//         pub mod _impl {
//             use std::cmp::Ordering;
//             use std::collections::BinaryHeap;

//             #[derive(Copy, Clone, Eq, PartialEq, Debug)]
//             struct State {
//                 cost: usize,
//                 position: (usize, usize),
//             }

//             impl Ord for State {
//                 fn cmp(&self, other: &Self) -> Ordering {
//                     other.cost.cmp(&self.cost)
//                 }
//             }

//             impl PartialOrd for State {
//                 fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//                     Some(self.cmp(other))
//                 }
//             }

//             pub struct Solution;

//             impl Solution {
//                 pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
//                     let m = grid.len();
//                     let n = grid[0].len();
//                     let mut explored = vec![vec![false; n]; m];
//                     let mut heap = BinaryHeap::new();
//                     // Start
//                     heap.push(State { cost: grid[0][0] as usize, position: (0, 0) });
//                     // Examine the frontier with lower cost nodes first (min-heap)
//                     while let Some(State { cost, position }) = heap.pop() {
//                         let (i, j) = position;
//                         // println!("{:?}", (position, cost));
//                         // Goal check
//                         if i == m - 1 && j == n - 1 { return cost as i32; }
//                         // Neighbours
//                         if i < m - 1 && !explored[i + 1][j] {
//                             let next = State { cost: cost + grid[i + 1][j] as usize, position: (i + 1, j) };
//                             explored[i + 1][j] = true;
//                             heap.push(next);
//                         }
//                         if j < n - 1 && !explored[i][j + 1] {
//                             let next = State { cost: cost + grid[i][j + 1] as usize, position: (i, j + 1) };
//                             explored[i][j + 1] = true;
//                             heap.push(next);
//                         }
//                         // println!("{:?}", heap);
//                     }

//                     // Goal not reachable
//                     -1
//                 }
//             }
//         }
//     }
// }

// // backward

// stralg::prob! { name(min_cost_climb_stairs)
//     desc(url("https://leetcode.com/problems/min-cost-climbing-stairs/"))
//     corr(Solution::min_cost_climbing_stairs => i32) [
//         {"[10,15,20]"} "15"
//         {"[1,100,1,1,1,100,1,1,100,1]"} "6"
//     ]
//     code(end_to_start, T : N, S : N) {
//         impl Solution {
//             pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
//                 let mut min_cost_arr = cost.clone();
//                 for i in (0..cost.len()).rev() {
//                     if i + 2 >= cost.len() {
//                         min_cost_arr[i] = cost[i];
//                     } else {
//                         min_cost_arr[i] = min_cost_arr[i] + min_cost_arr[i + 1].min(min_cost_arr[i + 2]);
//                     }
//                 }
//                 min_cost_arr[0].min(min_cost_arr[1])
//             }
//         }
//     }
// }

// stralg::prob! { name(min_fall_path_sum_adj)
//     desc(url("https://leetcode.com/problems/minimum-falling-path-sum/"))
//     corr(Solution::min_falling_path_sum => i32) [
//         {"[[2,1,3],[6,5,4],[7,8,9]]"} "13"
//         {"[[-19,57],[-40,-5]]"} "-59"
//         {"[[-48]]"} "-48"
//     ]
//     code(end_to_start, T : N2, S : N2) {
//         impl Solution {
//             pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
//                 let n = matrix.len();
//                 let mut min_sum_from = vec![vec![0; n]; n];
//                 // Fill last row of min_sum_from
//                 for col in 0..n {
//                     min_sum_from[n - 1][col] = matrix[n - 1][col];
//                 }
//                 // Fill other rows of min_sum_from
//                 for row in (0..(n -1)).rev() {
//                     for col in 0..n {
//                         let mut best = min_sum_from[row + 1][col];
//                         if col > 0 {
//                             best = best.min(min_sum_from[row + 1][col - 1]);
//                         }
//                         if col < n - 1 {
//                             best = best.min(min_sum_from[row + 1][col + 1]);
//                         }
//                         min_sum_from[row][col] = matrix[row][col] + best;
//                     }
//                 }
//                 // Find the minimum in the first row in min_sum_from
//                 *min_sum_from[0].iter().min().unwrap()
//             }
//         }
//     }
// }

// stralg::prob! { name(min_fall_path_sum_triangle)
// desc(url("https://leetcode.com/problems/triangle/"))
// corr(Solution::minimum_total => i32) [
//     {"[[2],[3,4],[6,5,7],[4,1,8,3]]"} "11"
//     {"[[-10]]"} "-10"
// ]
// code(end_to_start, T : N2, S : N) {
//         impl Solution {
//             pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
//                 let mut min_cost_from = triangle[triangle.len() - 1].clone();
//                 for row in (0..(triangle.len() - 1)).rev() {
//                     for c in 0..(row + 1) {
//                         min_cost_from[c] = triangle[row][c] + min_cost_from[c].min(min_cost_from[c + 1]);
//                     }
//                 }
//                 min_cost_from[0]
//             }
//         }
//     }
// }

// stralg::prob! { name(min_fall_path_sum_non_zero_shift)
//     desc(url("https://leetcode.com/problems/minimum-falling-path-sum-ii/"))
//     corr(Solution::min_falling_path_sum => i32) [
//         {"[[1,2,3],[4,5,6],[7,8,9]]"} "13"
//         {"[[7]]"} "7"
//     ]
//     code(end_to_start, T : N3, S : N2) {
//         impl Solution {
//             pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
//                 let n = matrix.len();
//                 let mut min_sum_from = vec![vec![0; n]; n];
//                 // Fill last row of min_sum_from
//                 for col in 0..n {
//                     min_sum_from[n - 1][col] = matrix[n - 1][col];
//                 }
//                 // Fill other rows of min_sum_from
//                 for row in (0..(n -1)).rev() {
//                     for col in 0..n {
//                         let mut best = std::i32::MAX;
//                         for down in 0..n {
//                             if col == down {
//                                 continue;
//                             }
//                             best = best.min(min_sum_from[row + 1][down]);
//                         }
//                         min_sum_from[row][col] = matrix[row][col] + best;
//                     }
//                 }
//                 // Find the minimum in the first row in min_sum_from
//                 *min_sum_from[0].iter().min().unwrap()
//             }
//         }
//     }
//     code(end_to_start_with_more_state, T : N2, S : N2) {
//         impl Solution {
//             pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
//                 let n = matrix.len();
//                 if n == 1 {
//                     return matrix[0][0];
//                 }
//                 fn least_two(row: &Vec<i32>) -> (usize, i32, i32) {
//                     let mut min = row[0];
//                     let mut min_idx = 0;
//                     for (i, &e) in row.iter().enumerate() {
//                         if e < min {
//                             min = e;
//                             min_idx = i;
//                         }
//                     }
//                     let min = min;
//                     let min_idx = min_idx;
//                     let mut second_min = std::i32::MAX;
//                     for (i, &e) in row.iter().enumerate() {
//                         if min_idx == i {
//                             continue;
//                         }
//                         if e < second_min {
//                             second_min = e;
//                         }
//                     }
//                     (min_idx, min, second_min)
//                 }
//                 let mut min_sum_from = vec![vec![0; n]; n];
//                 // Fill last row of min_sum_from
//                 for col in 0..n {
//                     min_sum_from[n - 1][col] = matrix[n - 1][col];
//                 }
//                 let (mut min_idx, mut min, mut second_min) = least_two(&min_sum_from[n - 1]);
//                 // Fill other rows of min_sum_from
//                 for row in (0..(n -1)).rev() {
//                     for col in 0..n {
//                         if col == min_idx {
//                             min_sum_from[row][col] = matrix[row][col] + second_min;
//                         } else {
//                             min_sum_from[row][col] = matrix[row][col] + min;
//                         }
//                     }
//                     let (min_idx1, min1, second_min1) = least_two(&min_sum_from[row]);
//                     min_idx = min_idx1;
//                     min = min1;
//                     second_min = second_min1;
//                 }
//                 // Find the minimum in the first row in min_sum_from
//                 *min_sum_from[0].iter().min().unwrap()
//             }
//         }
//     }
// }

// // forward

// stralg::prob! { name(best_buy_sell_times)
//     desc(url("https://leetcode.com/problems/best-time-to-buy-and-sell-stock/"))
//     corr(Solution::max_profit => i32) [
//         {"[7,1,5,3,6,4]"} "5"
//         {"[7,6,4,3,1]"} "0"
//     ]
//     code(start_to_end, T : N, S : C) {
//         impl Solution {
//             pub fn max_profit(prices: Vec<i32>) -> i32 {
//                 let mut ans = 0i32;
//                 let mut min_price_until_now = 10_000i32;
//                 for &price in prices.iter() {
//                     if price > min_price_until_now {
//                         let profit_here = price - min_price_until_now;
//                         if profit_here > ans {
//                             ans = profit_here;
//                         }
//                     } else {
//                        min_price_until_now = price;
//                     }
//                 }
//                 ans
//             }
//         }
//     }
// }

// stralg::prob! { name(count_climbing_ways)
//     desc(url("https://leetcode.com/problems/climbing-stairs/"))
//     corr(Solution::climb_stairs => i32) [
//         {"2"} "2"
//         {"3"} "3"
//     ]
//     code(prev_plus_prev_prev, T : N, S : N) {
//         impl Solution {
//             pub fn climb_stairs(n: i32) -> i32 {
//                 if n == 1 {
//                     return 1;
//                 }
//                 if n == 2 {
//                     return 2;
//                 }
//                 let mut ans_arr = vec![1i32, 2i32];
//                 for i in 2..(n as usize) {
//                     ans_arr.push(ans_arr[i - 1] + ans_arr[i - 2]);
//                 }
//                 ans_arr[n as usize - 1]
//             }
//         }
//     }
// }

// stralg::prob! { name(count_paths_1)
//     desc(url("https://leetcode.com/problems/unique-paths/"))
//     corr(Solution::unique_paths => i32) [
//         {"3" "2"} "3"
//         {"7" "3"} "28"
//         {"3" "7"} "28"
//         {"3" "3"} "6"
//     ]
//     code(left_plus_top, T : MN, S : MN) {
//         impl Solution {
//             pub fn unique_paths(m: i32, n: i32,) -> i32 {
//                 let m = m as usize;
//                 let n = n as usize;
//                 let mut unique_paths_to = vec![vec![0; n]; m];
//                 // We define num of unique paths in case of 1 x 1 grid is 1
//                 unique_paths_to[0][0] = 1;
//                 // Init first row
//                 for i in 1..n {
//                     unique_paths_to[0][i] = 1;
//                 }
//                 // Init first col
//                 for i in 1..m {
//                     unique_paths_to[i][0] = 1;
//                 }
//                 // Remaining positions
//                 for i in 1..m {
//                     for j in 1..n {
//                         unique_paths_to[i][j] = unique_paths_to[i - 1][j] + unique_paths_to[i][j - 1];
//                     }
//                 }
//                 unique_paths_to[m - 1][n - 1]
//             }
//         }
//     }
// }

// stralg::prob! { name(count_paths_2)
//     desc(url("https://leetcode.com/problems/unique-paths-ii/"))
//     corr(Solution::unique_paths_with_obstacles => i32) [
//         {"[[0,0,0],[0,1,0],[0,0,0]]"} "2"
//         {"[[0,1],[0,0]]"} "1"
//     ]
//     code(left_plus_top, T : MN, S : MN) {
//         impl Solution {
//             pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
//                 let m = obstacle_grid.len() as usize;
//                 let n = obstacle_grid[0].len() as usize;
//                 let mut unique_paths_to = vec![vec![0; n]; m];
//                 // We define num of unique paths in case of 1 x 1 grid is 1
//                 // DONOT make any assumptions that you can test easily
//                 unique_paths_to[0][0] = if obstacle_grid[0][0] == 1 {
//                     0
//                 } else {
//                     1
//                 };
//                 // Init first row
//                 let mut found_obstacle = if obstacle_grid[0][0] == 1 {
//                     true
//                 } else {
//                     false
//                 };
//                 for i in 1..n {
//                     if obstacle_grid[0][i] == 1 {
//                         found_obstacle = true;
//                     }
//                     if found_obstacle {
//                         unique_paths_to[0][i] = 0;
//                     } else {
//                         unique_paths_to[0][i] = 1;
//                     }
//                 }
//                 // Init first col
//                 let mut found_obstacle = if obstacle_grid[0][0] == 1 {
//                     true
//                 } else {
//                     false
//                 };
//                 for i in 1..m {
//                     if obstacle_grid[i][0] == 1 {
//                         found_obstacle = true;
//                     }
//                     if found_obstacle {
//                         unique_paths_to[i][0] = 0;
//                     } else {
//                         unique_paths_to[i][0] = 1;
//                     }
//                 }
//                 // Remaining positions
//                 for i in 1..m {
//                     for j in 1..n {
//                         if obstacle_grid[i][j] == 1 {
//                             unique_paths_to[i][j] = 0;
//                         } else {
//                             unique_paths_to[i][j] = unique_paths_to[i - 1][j] + unique_paths_to[i][j - 1];
//                         }
//                     }
//                 }
//                 unique_paths_to[m - 1][n - 1]
//             }
//         }
//     }
// }

// stralg::prob! { name(count_ones_in_bin_repr)
//     desc(url("https://leetcode.com/problems/counting-bits/"))
//     corr(Solution::count_bits => Vec<i32>) [
//         {"2"} "[0, 1, 1]"
//         {"5"} "[0, 1, 1, 2, 1, 2]"
//     ]
//     code(one_plus_other_idx, T : N, S : N) {
//         impl Solution {
//             pub fn count_bits(num: i32) -> Vec<i32> {
//                 if num == 0 {
//                     return vec![0];
//                 }
//                 let num = num as usize;
//                 let mut ones_at = vec![0usize; num + 1];
//                 ones_at[1] = 1;
//                 let mut idx = 0;
//                 let mut next_power_of_2 = 4;
//                 for i in 2..(num + 1) {
//                     ones_at[i] = 1 + ones_at[idx];
//                     idx += 1;
//                     if (i + 1) == next_power_of_2 {
//                         idx = 0;
//                         next_power_of_2 *= 2;
//                     }
//                 }
//                 ones_at.iter().map(|&x| x as i32).collect()
//             }
//         }
//     }
// }

// stralg::prob! { name(count_sqrsubmat_with_all_ones)
//     desc(url("https://leetcode.com/problems/count-square-submatrices-with-all-ones/"))
//     corr(Solution::count_squares => i32) [
//         {r#"
// [
//   [0,1,1,1],
//   [1,1,1,1],
//   [0,1,1,1]
// ]
//             "#} "15"
//         {r#"
// [
//   [1,0,1],
//   [1,1,0],
//   [1,1,0]
// ]
//             "#} "7"
//     ]
//     code(start_to_end, T : MN, S : MN) {
//         impl Solution {
//             pub fn count_squares(mat: Vec<Vec<i32>>) -> i32 {
//                 let m = mat.len();
//                 let n = mat[0].len();
//                 let mut number_to = mat.clone();
//                 for i in 1..m {
//                     for j in 1..n {
//                         if number_to[i][j] != 0 {
//                             number_to[i][j] = (number_to[i - 1][j].min(number_to[i][j -1])).min(number_to[i - 1][j - 1]) + 1;
//                         }
//                     }
//                 }
//                 // println!("{:?}", number_to);
//                 let mut total_num_squares = 0;
//                 for i in 0..m {
//                     for j in 0..n {
//                         total_num_squares += number_to[i][j];
//                     }
//                 }
//                 total_num_squares
//             }
//         }
//     }
// }

// // games

// stralg::prob! { name(stone_game)
//     desc(url("https://leetcode.com/problems/stone-game/"))
//     corr(Solution::stone_game => bool) [
//         {"[5,3,4,5]"} "true"
//         {"[3,7,2,3]"} "true"
//     ]
//     code(top_down_full_play, T : N2, S : N2) {
//         impl Solution {
//             pub fn stone_game(piles: Vec<i32>) -> bool {
//                 use std::collections::HashMap;
//                 fn stone_game_rec(piles: &Vec<i32>,
//                                   l: usize,
//                                   r: usize,
//                                   alex_turn: bool,
//                                   optimal_scores: &mut HashMap<(usize, usize), (usize, usize)>
//                                   ) -> (usize, usize) {
//                      match optimal_scores.get(&(l, r)) {
//                         Some(&(alex_score, lee_score)) => {
//                             return (alex_score, lee_score);
//                          },
//                         None => {
//                             if l + 1 == r {
//                                 let a = piles[l] as usize;
//                                 let b = piles[r] as usize;
//                                 let (alex_score, lee_score) = (a.max(b), a.min(b));
//                                 optimal_scores.insert((l, r), (alex_score, lee_score));
//                                 return (alex_score, lee_score);
//                             }
//                             let (l_alex_score, l_lee_score) = stone_game_rec(piles, l, r - 1, !alex_turn, optimal_scores);
//                             optimal_scores.insert((l, r - 1), (l_alex_score, l_lee_score));
//                             let (r_alex_score, r_lee_score) = stone_game_rec(piles, l + 1, r, !alex_turn, optimal_scores);
//                             optimal_scores.insert((l + 1, r), (r_alex_score, r_lee_score));
//                             if alex_turn {
//                                 let l_alex_curr_score = piles[r] as usize;
//                                 let r_alex_curr_score = piles[l] as usize;
//                                 if l_alex_curr_score + l_alex_score >= r_alex_curr_score + r_alex_score {
//                                     return (l_alex_curr_score + l_alex_score, l_lee_score);
//                                 } else {
//                                     return (r_alex_curr_score + r_alex_score, r_lee_score);
//                                 }
//                             } else {
//                                 let l_lee_curr_score = piles[r] as usize;
//                                 let r_lee_curr_score = piles[l] as usize;
//                                 if l_lee_curr_score + l_lee_score >= r_lee_curr_score + r_lee_score {
//                                     return (l_alex_score, l_lee_curr_score + l_lee_score);
//                                 } else {
//                                     return (r_alex_score, r_lee_curr_score + r_lee_score);
//                                 }
//                             }
//                          }
//                     }
//                 }
//                 let mut optimal_scores = HashMap::new();
//                 let (alex_score, lee_score) = stone_game_rec(&piles, 0, piles.len() - 1, true, &mut optimal_scores);
//                 alex_score > lee_score
//             }
//         }
//     }
// }

// // mountains

// stralg::prob! { name(longest_mount_len)
//     desc(url("https://leetcode.com/problems/longest-mountain-in-array/"))
//     corr(Solution::longest_mountain => i32) [
//         {"[2,1,4,7,3,2,5]"} "5"
//         {"[2,2,2]"} "0"
//     ]
//     code(store_left_heights, T : N, S : N) {
//         impl Solution {
//             pub fn longest_mountain(nums: Vec<i32>) -> i32 {
//                 fn get_left_heights(nums: &Vec<i32>) -> Vec<i32> {
//                     let mut left_heights = vec![0; nums.len()];
//                     for i in 1..nums.len() {
//                         if nums[i -1] < nums[i] {
//                             left_heights[i] = left_heights[i - 1] + 1;
//                         }
//                     }
//                     left_heights
//                 }
//                 let left_heights = get_left_heights(&nums);
//                 let right_heights = {
//                     let mut rev = nums.clone();
//                     rev.reverse();
//                     let mut rev_left_heights = get_left_heights(&rev);
//                     rev_left_heights.reverse();
//                     rev_left_heights
//                 };
//                 let mut longest_mountain_len = 0;
//                 for (&l, &r) in left_heights.iter().zip(right_heights.iter()) {
//                     if l > 0 && r > 0 {
//                         longest_mountain_len = longest_mountain_len.max(l + r + 1);
//                     }
//                 }
//                 longest_mountain_len
//             }
//         }
//     }
//     code(state_machine, T : N, S : C) {
//         // This kind of implementations almost always have bugs, so avoid whenever possible.
//         impl Solution {
//             pub fn longest_mountain(nums: Vec<i32>) -> i32 {
//                 let mut head = 1usize;
//                 let mut up = 0usize;
//                 let mut down = 0usize;
//                 let mut max_mountain = 0usize;
//                 loop {
//                     if head > nums.len() - 1 {
//                         if up > 0 && down > 0 {
//                             max_mountain = max_mountain.max(up + down + 1);
//                         }
//                         break;
//                     }
//                     if nums[head - 1] < nums[head] {
//                         if up == 0 && down == 0 {
//                             up += 1;
//                         } else if up > 0 && down == 0 {
//                             up += 1;
//                         } else if up == 0 && down > 0 {
//                             panic!("Logical error");
//                         } else {
//                             max_mountain = max_mountain.max(up + down + 1);
//                             up = 1;
//                             down = 0;
//                         }
//                     } else if nums[head - 1] > nums[head] {
//                         if up == 0 && down == 0 {
//                             // pass
//                         } else if up > 0 && down == 0 {
//                             down += 1;
//                         } else if up == 0 && down > 0 {
//                             panic!("Logical error");
//                         } else {
//                             down += 1;
//                         }
//                     } else {
//                         if up == 0 && down == 0 {
//                             // descend platue
//                         } else if up > 0 && down == 0 {
//                             // ascend platue
//                             up = 0;
//                             down = 0;
//                         } else if up == 0 && down > 0 {
//                             panic!("Logical error");
//                         } else {
//                             max_mountain = max_mountain.max(up + down + 1);
//                             up = 0;
//                             down = 0;
//                         }
//                     }
//                     head += 1;
//                 }
//                 max_mountain as i32
//             }
//         }
//     }
// }

// stralg::prob! { name(min_removals_to_mount)
//     desc(url("https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/"))
//     corr(Solution::minimum_mountain_removals => i32) [
//         {"[1,3,1]"} "0"
//         {"[2,1,1,5,6,2,3,1]"} "3"
//         {"[4,3,2,1,1,2,3,1]"} "4"
//         {"[1,2,3,4,4,3,2,1]"} "1"
//     ]
//     code(lis_lds_based, T : N2, S : N) {
//         impl Solution {
//             pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
//                 fn lis_lens(nums: &Vec<i32>) -> Vec<usize> {
//                     let mut lis = vec![1; nums.len()];
//                     for i in 1..nums.len() {
//                         for j in 0..i {
//                             if nums[j] < nums[i] && lis[j] + 1 > lis[i] {
//                                 lis[i] = lis[j] + 1;
//                             }
//                         }
//                     }
//                     lis
//                 }
//                 let lis = lis_lens(&nums);
//                 let lds = {
//                     let mut rev = nums.clone();
//                     rev.reverse();
//                     let mut lis = lis_lens(&rev);
//                     lis.reverse();
//                     lis
//                 };
//                 let mut min_removals = std::usize::MAX;
//                 for (&i, &d) in lis.iter().zip(lds.iter()) {
//                     if i > 1 && d > 1 {
//                         min_removals = min_removals.min(nums.len() - (i + d - 1));
//                     }
//                 }
//                 min_removals as i32
//             }
//         }
//     }
// }

// // pseudo poly

// use serde::Deserialize;

// stralg::prob! { name(count_sorted_vowel_strings)
//     desc(url("https://leetcode.com/problems/count-sorted-vowel-strings/"))
//     corr(Solution::count_vowel_strings => i32) [
//         {"1"} "5"
//         {"2"} "15"
//         {"33"} "66045"
//     ]
//     code(bottom_up, T : MN, S : N) {
//         impl Solution {
//             pub fn count_vowel_strings(n: i32) -> i32 {
//                 let mut sum_to_n = [1usize, 2, 3, 4, 5];
//                 for _ in 2..=n {
//                     for j in 1..5 {
//                         sum_to_n[j] += sum_to_n[j - 1];
//                     }
//                 }
//                 sum_to_n[4] as i32
//             }
//         }
//     }
// }

// #[derive(Deserialize, Debug)]
// pub struct Knapsack01Item {
//     pub name: &'static str,
//     pub weight: usize,
//     pub value: i32,
// }

// stralg::prob! { name(knapsack_01)
//     desc(string(r#"- There is a cupboard with inventory items. Each item has a quantity of exactly one.
// - Each item has a weight and a value.
// - You have a knapsack with a maximum weight capacity.
// - You need to pack some items from inventory into the bag.
// - How can you do it to maximize the total value in the bag without exceeding the weight capacity?
// - Testcase from: <https://rosettacode.org/wiki/Knapsack_problem/0-1>.
// - PS.
//     - A solution for 0..1 knapsack problem can be used for a 0..n_i knapsack problem.
//     - Because 0..n_i knapsack problem can be converted to 0..1 knapsack problem by just flattening the inventory.
// "#))
//     corr(_impl::pack => std::collections::BTreeSet<usize>) [
//         {"400" r#"[{"name":"map","weight":9,"value":150},{"name":"compass","weight":13,"value":35},{"name":"water","weight":153,"value":200},{"name":"sandwich","weight":50,"value":160},{"name":"glucose","weight":15,"value":60},{"name":"tin","weight":68,"value":45},{"name":"banana","weight":27,"value":60},{"name":"apple","weight":39,"value":40},{"name":"cheese","weight":23,"value":30},{"name":"beer","weight":52,"value":10},{"name":"suntan cream","weight":11,"value":70},{"name":"camera","weight":32,"value":30},{"name":"T-shirt","weight":24,"value":15},{"name":"trousers","weight":48,"value":10},{"name":"umbrella","weight":73,"value":40},{"name":"waterproof trousers","weight":42,"value":70},{"name":"waterproof overclothes","weight":43,"value":75},{"name":"note-case","weight":22,"value":80},{"name":"sunglasses","weight":7,"value":20},{"name":"towel","weight":18,"value":12},{"name":"socks","weight":4,"value":50},{"name":"book","weight":30,"value":10}]"#} "[0, 1, 2, 3, 4, 6, 10, 15, 16, 17, 20, 18]"
//         {"400" r#"[{"name":"map","weight":9,"value":-150},{"name":"compass","weight":13,"value":-35},{"name":"water","weight":153,"value":-200},{"name":"sandwich","weight":50,"value":-160},{"name":"glucose","weight":15,"value":-60}]"#} "[]"
//     ]
//     code(powerset, T: TwoToN, S: TwoToN) {
//         // Space : 2^N subsets + max N clones at a time == 2^N + N
//         mod _impl {
//             fn power_set_of(index: usize, size: usize, parent: &Vec<usize>) -> Vec<Vec<usize>> {
//                 if index == size {
//                     return vec![parent.clone()];
//                 }
//                 let left = {
//                     let mut clone = parent.clone();
//                     clone.push(index);
//                     clone
//                 };
//                 let left_subsets = power_set_of(index + 1, size, &left);
//                 let right = parent.clone();
//                 let right_subsets = power_set_of(index + 1, size, &right);
//                 return left_subsets
//                     .into_iter()
//                     .chain(right_subsets.into_iter())
//                     .collect();
//             }

//             use std::collections::BTreeSet;
//             use std::iter::FromIterator;
//             pub fn pack(capacity: usize, inventory: Vec<super::super::super::Knapsack01Item>) -> BTreeSet<usize> {
//                 let all_combinations = power_set_of(0usize, inventory.len(), &vec![]);
//                 // println!("number of combinations = {}", all_combinations.len());
//                 // Highest value combination
//                 let mut highest_value_index = 0;
//                 let mut highest_value = i32::min_value();
//                 for (cmb_i, cmb) in all_combinations.iter().enumerate() {
//                     let total_weight: usize = cmb.iter().map(|index| inventory[*index].weight).sum();
//                     let total_value: i32 = cmb.iter().map(|index| inventory[*index].value).sum();
//                     if total_weight <= capacity && total_value > highest_value {
//                         highest_value = total_value;
//                         highest_value_index = cmb_i;
//                     }
//                 }
//                 BTreeSet::from_iter(all_combinations[highest_value_index].clone().into_iter())
//             }
//         }
//     }
//     code(powerset_prune, T: TwoToN, S: TwoToN) {
//         // Space : 2^N subsets + max N clones at a time == 2^N + N
//         mod _impl {
//             fn power_set_of(
//                 index: usize,
//                 size: usize,
//                 parent: &Vec<usize>,
//                 capacity: usize,
//                 inventory: &Vec<super::super::super::Knapsack01Item>,
//             ) -> Vec<Vec<usize>> {
//                 // Pruning
//                 if parent
//                     .iter()
//                     .map(|index| inventory[*index].weight)
//                     .sum::<usize>()
//                     > capacity
//                     || index == size
//                 {
//                     return vec![parent.clone()];
//                 }

//                 let left = {
//                     let mut clone = parent.clone();
//                     clone.push(index);
//                     clone
//                 };
//                 let left_subsets = power_set_of(index + 1, size, &left, capacity, inventory);

//                 let right = parent.clone();
//                 let right_subsets = power_set_of(index + 1, size, &right, capacity, inventory);

//                 return left_subsets
//                     .into_iter()
//                     .chain(right_subsets.into_iter())
//                     .collect();
//             }

//             use std::collections::BTreeSet;
//             use std::iter::FromIterator;
//             pub fn pack(capacity: usize, inventory: Vec<super::super::super::Knapsack01Item>) -> BTreeSet<usize> {
//                 let all_combinations = power_set_of(0usize, inventory.len(), &vec![], capacity, &inventory);
//                 // println!("number of combinations = {}", all_combinations.len());
//                 // Highest value combination
//                 let mut highest_value_index = 0;
//                 let mut highest_value = i32::min_value();
//                 for (cmb_i, cmb) in all_combinations.iter().enumerate() {
//                     let total_weight: usize = cmb.iter().map(|index| inventory[*index].weight).sum();
//                     let total_value: i32 = cmb.iter().map(|index| inventory[*index].value).sum();
//                     if total_weight <= capacity && total_value > highest_value {
//                         highest_value = total_value;
//                         highest_value_index = cmb_i;
//                     }
//                 }
//                 BTreeSet::from_iter(all_combinations[highest_value_index].clone().into_iter())
//             }
//         }
//     }
//     code(powerset_prune_implicitcmp, T: TwoToN, S: TwoToN) {
//         // Space : 2^N subsets + max N clones at a time == 2^N + N
//         mod _impl {
//             fn pack_recur(
//                 index: usize,
//                 size: usize,
//                 parent: &Vec<usize>,
//                 capacity: usize,
//                 inventory: &Vec<super::super::super::Knapsack01Item>,
//             ) -> Option<Vec<usize>> {
//                 // Pruning
//                 if parent
//                     .iter()
//                     .map(|index| inventory[*index].weight)
//                     .sum::<usize>()
//                     > capacity
//                 {
//                     return None;
//                 }
//                 if index == size {
//                     return Some(parent.clone());
//                 }

//                 let left = {
//                     let mut clone = parent.clone();
//                     clone.push(index);
//                     clone
//                 };
//                 let left_best_pack = pack_recur(index + 1, size, &left, capacity, inventory);

//                 let right = parent.clone();
//                 let right_best_pack = pack_recur(index + 1, size, &right, capacity, inventory);

//                 // Implicit comparision
//                 match (left_best_pack, right_best_pack) {
//                     (Some(left), Some(right)) => {
//                         let left_value: i32 = left.iter().map(|index| inventory[*index].value).sum();
//                         let right_value: i32 = right.iter().map(|index| inventory[*index].value).sum();
//                         if right_value > left_value {
//                             return Some(right);
//                         } else {
//                             return Some(left);
//                         }
//                     }
//                     (Some(left), None) => return Some(left),
//                     (None, Some(right)) => return Some(right),
//                     (None, None) => return None,
//                 }
//             }

//             use std::collections::BTreeSet;
//             use std::iter::FromIterator;
//             pub fn pack(capacity: usize, inventory: Vec<super::super::super::Knapsack01Item>) -> BTreeSet<usize> {
//                 let best_combination =
//                     pack_recur(0usize, inventory.len(), &vec![], capacity, &inventory).unwrap();
//                 BTreeSet::from_iter(best_combination.into_iter())
//             }
//         }
//     }
//     code(bottom_up, T: MN, S: MN) {
//         mod _impl {
//             use std::collections::BTreeSet;
//             use std::iter::FromIterator;
//             pub fn pack(capacity: usize, inventory: Vec<super::super::super::Knapsack01Item>) -> BTreeSet<usize> {
//                 let best_until_1idxitem_capacity = {
//                     let mut best_until_1idxitem_capacity = vec![vec![0i32; capacity + 1]; inventory.len() + 1];
//                     // Initilization
//                     for curr_capacity in 0..(capacity + 1) {
//                         best_until_1idxitem_capacity[0][curr_capacity] = 0;
//                     }
//                     for item_1_idx in 1..(inventory.len() + 1) {
//                         let super::super::super::Knapsack01Item { name: _, weight, value } = inventory[item_1_idx - 1];
//                         for curr_capacity in 0..(capacity + 1) {
//                             let value_not_considering_this = best_until_1idxitem_capacity[item_1_idx - 1][curr_capacity];
//                             best_until_1idxitem_capacity[item_1_idx][curr_capacity] = if weight <= curr_capacity {
//                                 let value_considering_this = value + best_until_1idxitem_capacity[item_1_idx - 1][curr_capacity - weight];
//                                 value_considering_this.max(value_not_considering_this)
//                             } else {
//                                 value_not_considering_this
//                             }
//                         }
//                     }
//                     best_until_1idxitem_capacity
//                 };
//                 let best_combination = {
//                     let mut best_combination = Vec::with_capacity(inventory.len());
//                     let mut c = capacity;
//                     let mut r = inventory.len();
//                     while r > 0 {
//                         if best_until_1idxitem_capacity[r][c] == best_until_1idxitem_capacity[r - 1][c] {
//                             r = r - 1;
//                             c = c;
//                         } else {
//                             best_combination.push(r - 1);
//                             r = r - 1;
//                             c = c - inventory[r].weight;
//                         }
//                     }
//                     best_combination
//                 };
//                 BTreeSet::from_iter(best_combination.into_iter())
//             }
//         }
//     }
//     code(top_down, T: MN, S: MN) {
//         mod _impl {
//             fn best_until(best_until_1idxitem_capacity: &mut Vec<Vec<Option<i32>>>,
//                         inventory: &Vec<super::super::super::Knapsack01Item>,
//                         item_1_idx: usize,
//                         curr_capacity: usize,
//             ) -> i32 {
//                 if let Some(val) = best_until_1idxitem_capacity[item_1_idx][curr_capacity] {
//                     return val;
//                 }
//                 if item_1_idx == 0 {
//                     best_until_1idxitem_capacity[0][curr_capacity] = Some(0);
//                     return best_until_1idxitem_capacity[0][curr_capacity].unwrap();
//                 }
//                 let value_not_considering_this = best_until_1idxitem_capacity[item_1_idx - 1][curr_capacity].unwrap_or(
//                         best_until(best_until_1idxitem_capacity, inventory, item_1_idx - 1, curr_capacity)
//                     );
//                 let super::super::super::Knapsack01Item { name: _, weight, value } = inventory[item_1_idx - 1];
//                 best_until_1idxitem_capacity[item_1_idx][curr_capacity] = Some(if weight <= curr_capacity {
//                     let remaining_value = best_until_1idxitem_capacity[item_1_idx - 1][curr_capacity - weight].unwrap_or(
//                             best_until(best_until_1idxitem_capacity, inventory, item_1_idx - 1, curr_capacity - weight)
//                         );
//                     let value_considering_this = value + remaining_value;
//                     value_considering_this.max(value_not_considering_this)
//                 } else {
//                     value_not_considering_this
//                 });
//                 best_until_1idxitem_capacity[item_1_idx][curr_capacity].unwrap()
//             }

//             use std::collections::BTreeSet;
//             use std::iter::FromIterator;
//             pub fn pack(capacity: usize, inventory: Vec<super::super::super::Knapsack01Item>) -> BTreeSet<usize> {
//                 let mut best_until_1idxitem_capacity: Vec<Vec<Option<i32>>> = vec![vec![None; capacity + 1]; inventory.len() + 1];
//                 best_until(&mut best_until_1idxitem_capacity, &inventory, inventory.len(), capacity);
//                 let best_combination = {
//                     let mut best_combination = Vec::with_capacity(inventory.len());
//                     let mut c = capacity;
//                     let mut r = inventory.len();
//                     while r > 0 {
//                         if best_until_1idxitem_capacity[r][c] == best_until_1idxitem_capacity[r - 1][c] {
//                             r = r - 1;
//                             c = c;
//                         } else {
//                             best_combination.push(r - 1);
//                             r = r - 1;
//                             c = c - inventory[r].weight;
//                         }
//                     }
//                     best_combination
//                 };
//                 BTreeSet::from_iter(best_combination.into_iter())
//             }
//         }
//     }
// }

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


fn main() {}
