fn main() {
    let time = vec![49979494u64];
    let distance = vec![263153213781851u64];

    let mut rec = 1;
    for (t, d) in time.into_iter().zip(distance) {
        let mut num_of_ways = 0;
        for i in 1..t {
            if i * (t - i) > d {
                num_of_ways += 1
            }
        }
        if num_of_ways != 0 {
            rec *= num_of_ways
        }
    }

    println!("{rec}");
}
