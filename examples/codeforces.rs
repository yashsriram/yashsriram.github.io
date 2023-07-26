/// https://codeforces.com/problemset/problem/1352/A
fn decompose_as_round_numbers(num: u32) -> Vec<u32> {
    let mut rem = num;
    let mut round_numbers: Vec<u32> = vec![];
    let mut place_value = 1;
    while rem > 0 {
        let digit = rem % 10;
        if digit > 0 {
            round_numbers.push(digit * place_value);
        }

        // Loop update
        place_value *= 10;
        rem /= 10;
    }
    round_numbers
}

#[test]
fn simple() {
    assert_eq!(decompose_as_round_numbers(1), vec![1]);
    assert_eq!(decompose_as_round_numbers(7), vec![7]);
    assert_eq!(decompose_as_round_numbers(10), vec![10]);
    assert_eq!(decompose_as_round_numbers(5009), vec![9, 5000]);
    assert_eq!(decompose_as_round_numbers(10000), vec![10000]);
    assert_eq!(decompose_as_round_numbers(9876), vec![6, 70, 800, 9000]);
}

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

/// https://codeforces.com/contest/4/problem/A
fn can_split_into_even_parts(num: u32) -> bool {
    // Odd => NO
    if num % 2 == 1 {
        return false;
    }
    // Exception
    if num < 3 {
        return false;
    }
    return true;
}

#[test]
fn simple3() {
    assert_eq!(can_split_into_even_parts(1), false);
    assert_eq!(can_split_into_even_parts(2), false);
    assert_eq!(can_split_into_even_parts(3), false);
    assert_eq!(can_split_into_even_parts(5), false);
    assert_eq!(can_split_into_even_parts(7), false);
    assert_eq!(can_split_into_even_parts(9), false);
    assert_eq!(can_split_into_even_parts(4), true);
    assert_eq!(can_split_into_even_parts(6), true);
    assert_eq!(can_split_into_even_parts(8), true);
    assert_eq!(can_split_into_even_parts(11), false);
    assert_eq!(can_split_into_even_parts(13), false);
    assert_eq!(can_split_into_even_parts(15), false);
    assert_eq!(can_split_into_even_parts(17), false);
    assert_eq!(can_split_into_even_parts(19), false);
    assert_eq!(can_split_into_even_parts(10), true);
    assert_eq!(can_split_into_even_parts(12), true);
    assert_eq!(can_split_into_even_parts(14), true);
    assert_eq!(can_split_into_even_parts(16), true);
    assert_eq!(can_split_into_even_parts(18), true);
    assert_eq!(can_split_into_even_parts(21), false);
    assert_eq!(can_split_into_even_parts(23), false);
    assert_eq!(can_split_into_even_parts(25), false);
    assert_eq!(can_split_into_even_parts(27), false);
    assert_eq!(can_split_into_even_parts(29), false);
    assert_eq!(can_split_into_even_parts(20), true);
    assert_eq!(can_split_into_even_parts(22), true);
    assert_eq!(can_split_into_even_parts(24), true);
    assert_eq!(can_split_into_even_parts(26), true);
    assert_eq!(can_split_into_even_parts(28), true);
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

fn main() {}
