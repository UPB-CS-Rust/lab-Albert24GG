fn main() {
    let mut data = [22, 12, 13, 17, 18];
    data.iter_mut().for_each(|val| *val = floored_half(*val));
}

fn floored_half(data: i32) -> i32 {
    data / 2
}
