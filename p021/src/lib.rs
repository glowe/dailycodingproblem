/*
Good morning! Here's your coding interview problem for today.

This problem was asked by Snapchat.

Given an array of time intervals (start, end) for classroom lectures (possibly overlapping), find the minimum number of rooms required.

For example, given [(30, 75), (0, 50), (60, 150)], you should return 2.
*/
fn min_rooms_quadratic(intervals: &mut [(u64, u64)]) -> u64 {
    // [(30, 75), (0, 50), (60, 150)],
    intervals.sort_by_key(|&(start, _end)| start);
    // [(0, 50), (30, 75), (60, 150)],
    let mut scheduled = vec![false; intervals.len()];
    let mut rooms: u64 = 0;
    for i in 0..intervals.len() {
        if scheduled[i] {
            continue;
        }
        // We are scheduling this interval
        scheduled[i] = true;
        // We need a room for this interval
        rooms += 1;
        // Now look for subsequent rooms that start after this one ends
        let (_start, mut last_end) = intervals[i];
        for j in i + 1..intervals.len() {
            if scheduled[j] {
                continue;
            }
            let (start, end) = intervals[j];
            if last_end > start {
                // overlap, can't schedule
                continue;
            }
            scheduled[j] = true;
            last_end = end;
        }
    }
    rooms
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_min_rooms_quadratic() {
        // (0, 50) -> (60, 150) | (30, 75)
        assert_eq!(min_rooms_quadratic(&mut [(30, 75), (0, 50), (60, 150)]), 2);
        // (0, 50) -> (60, 150) | (30, 75) | (35, 70)
        assert_eq!(
            min_rooms_quadratic(&mut [(30, 75), (0, 50), (60, 150), (35, 70)]),
            3
        );
        // (0, 25) -> (30, 75) -> (76, 1200) | (0, 50) -> (60, 150)
        assert_eq!(
            min_rooms_quadratic(&mut [(30, 75), (0, 50), (60, 150), (0, 25), (76, 1200)]),
            2
        );
    }
}
