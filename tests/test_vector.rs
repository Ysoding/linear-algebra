#[cfg(test)]
mod tests {
    use vector::Vector;
    use yla::*;

    #[test]
    fn test_addition() {
        let v1 = Vector::new(vec![1.0, 2.0]);
        let v2 = Vector::new(vec![3.0, 4.0]);

        let result = v1 + v2;

        assert_eq!(*result.values(), vec![4.0, 6.0]);
    }

    #[test]
    fn test_subtraction() {
        let v1 = Vector::new(vec![1.0, 2.0]);
        let v2 = Vector::new(vec![3.0, 4.0]);

        let result = v1 - v2;

        assert_eq!(*result.values(), vec![-2.0, -2.0]);

        let result = -Vector::new(vec![1.0, 2.0]);
        assert_eq!(*result.values(), vec![-1.0, -2.0]);
    }

    #[test]
    fn test_scalar_multiplication() {
        let vec = Vector::new(vec![1.0, 2.0]);
        let k = 2.0;

        let r1 = vec * k;

        let vec = Vector::new(vec![1.0, 2.0]);
        let r2 = k * vec;

        assert_eq!(*r1.values(), vec![2.0, 4.0]);
        assert_eq!(*r2.values(), vec![2.0, 4.0]);
        assert_eq!(*r1.values(), *r2.values());
    }

    #[test]
    fn test_display() {
        let v = Vector::new(vec![1.0, 2.0, 3.0]);
        let expected = "(1.0, 2.0, 3.0)";
        println!("{}", v);
        assert_eq!(v.to_string(), expected);
    }

    #[test]
    fn test_index() {
        let v = Vector::new(vec![1.0, 2.0, 3.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
    }

    #[test]
    fn test_iteration() {
        let v = Vector::new(vec![1.0, 2.0, 3.0]);
        let values: Vec<f64> = v.into_iter().collect();
        assert_eq!(values, vec![1.0, 2.0, 3.0]);
    }
}
