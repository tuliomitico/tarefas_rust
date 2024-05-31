fn max(x: i32,y: i32) -> i32   {
    if x > y { x } else { y }
}

fn find_max(vec: Vec<i32>, n: usize) -> i32 {
    if n == 1 {
        return vec[0];
    }

    return max(vec[n - 1], find_max(vec, n - 1));
}