/*
This problem was asked by Facebook.

A builder is looking to build a row of N houses that can be of K different colors. He has a goal of
minimizing cost while ensuring that no two neighboring houses are of the same color.

Given an N by K matrix where the nth row and kth column represents the cost to build the nth house
with kth color, return the minimum cost which achieves this goal.
*/

fn min_cost_dp(costs: &Vec<Vec<u64>>) -> u64 {
    let num_colors = costs[0].len();
    let num_houses = costs.len();
    let mut cache = vec![vec![std::u64::MAX; num_colors]; num_houses];
    // Initialize first row for cache.
    for k in 0..num_colors {
        cache[0][k] = costs[0][k];
    }
    // Fill the rest.
    for house in 1..num_houses {
        for color in 0..num_colors {
            for neighbor_color in 0..num_colors {
                if neighbor_color == color {
                    continue;
                }
                cache[house][color] =
                    cache[house][color].min(costs[house][color] + cache[house - 1][neighbor_color]);
            }
        }
    }
    // Min cost will be in final row.
    *cache[num_houses - 1].iter().min().unwrap()
}

fn min_cost(costs: &Vec<Vec<u64>>) -> u64 {
    fn recur(house: usize, neighbor_color: usize, costs: &Vec<Vec<u64>>) -> u64 {
        let num_colors = costs[0].len();
        let num_houses = costs.len();
        if house == num_houses {
            // We've painted all the houses
            return 0;
        }
        // the minimum cost to paint this house is: the mininum of painting this house some color +
        // the mininum cost of painting all the other houses
        let mut min = std::u64::MAX;
        for color in 0..num_colors {
            if color == neighbor_color {
                // We can't paint this house the same color as the neighbour
                continue;
            }
            min = min.min(costs[house][color] + recur(house + 1, color, costs))
        }
        min
    }
    // This is not a legal color. We use this to avoid special casing the first house.
    let sentinel_color = costs[0].len();
    recur(0, sentinel_color, costs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn costs() -> Vec<Vec<u64>> {
        vec![
            vec![40, 50, 60],
            vec![10, 90, 60],
            vec![30, 20, 5],
            vec![10, 20, 30],
        ]
    }

    #[test]
    fn test_min_cost() {
        assert_eq!(min_cost(&costs()), 75);
    }

    #[test]
    fn test_min_cost_dp() {
        assert_eq!(min_cost_dp(&costs()), 75);
    }
}
