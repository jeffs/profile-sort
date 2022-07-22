pub fn sqrt(x: f64) -> Option<f64> {
    (!x.is_sign_negative()).then(|| x.sqrt())
}

pub fn sum_sqrts(x: f64, y: f64) -> Option<f64> {
    Some(sqrt(x)? + sqrt(y)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_io() {
        let n = std::fs::read("count")
            .ok()
            .and_then(|bytes| bytes.try_into().ok().map(i32::from_be_bytes))
            .unwrap_or(1);
        println!("{n}");
        std::fs::write("count", (n + 1).to_be_bytes()).expect("can't write count");
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(Some(2.0), sqrt(4.0));
        assert_eq!(None, sqrt(-4.0));
    }

    #[test]
    fn test_sum_sqrts() {
        assert_eq!(Some(5.0), sum_sqrts(4.0, 9.0));
        assert_eq!(None, sum_sqrts(4.0, -9.0));
        assert_eq!(None, sum_sqrts(-4.0, 9.0));
        assert_eq!(None, sum_sqrts(-4.0, -9.0));
    }
}

fn main() {}
