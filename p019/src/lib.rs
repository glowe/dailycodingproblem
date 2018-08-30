/*
This problem was asked by Facebook.

A builder is looking to build a row of N houses that can be of K different colors. He has a goal of
minimizing cost while ensuring that no two neighboring houses are of the same color.

Given an N by K matrix where the nth row and kth column represents the cost to build the nth house
with kth color, return the minimum cost which achieves this goal.
*/
fn min_cost_recur(n: usize, last_k: usize, costs: &Vec<Vec<u64>>) -> u64 {
    let num_colors = costs[0].len();
    if n == costs.len() {
        // We've painted all the houses
        return 0;
    }
    // the minimum cost to paint this house is the mininum of painting this house some color +
    // the mininum cost of paiting all the other houses
    let mut min = std::u64::MAX;
    for k in 0..num_colors {
        if k == last_k {
            // We can't paint this house the same color as the neighbour
            continue;
        }
        let cost = costs[n][k] + min_cost_recur(n + 1, k, costs);
        min = min.min(cost);
    }
    min
}

fn min_cost(costs: &Vec<Vec<u64>>) -> u64 {
    let sentinel_color = costs[0].len();
    min_cost_recur(0, sentinel_color, costs)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let costs = vec![vec![40, 50, 60], vec![10, 90, 60], vec![30, 20, 5]];
        assert_eq!(min_cost(&costs), 65);
    }
}
