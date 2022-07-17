use itertools::Itertools as _; // for group_by

/// Returns the values 1 through 9, inclusive.
fn digits() -> std::ops::RangeInclusive<u32> {
    1..=9
}

/// Returns all 81 numbers having (in base 10) two non-zero digits.
fn numbers() -> impl Iterator<Item = u32> {
    digits().flat_map(|d0| digits().map(move |d1| d0 * 10 + d1))
}

fn main() {
    numbers()
        .flat_map(|a| numbers().filter(move |&b| b <= a).map(move |b| (a, b)))
        .filter(|(a, b)| a * b % 100 == (a % 10) * (b % 10))
        .inspect(|(a, b)| {
            println!("{a:2} * {b:2} == {:4}", a * b);
        })
        .group_by(|&(a, _)| a)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .for_each(|(key, count)| {
            println!("---------------");
            println!("             {count:2}");
            println!("");
        });
}
