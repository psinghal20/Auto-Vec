use trybuild;
use auto_vec::auto_vec;

#[test]
fn trybuild_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/failures/no_input.rs");
    t.compile_fail("tests/failures/no_output.rs");
    t.compile_fail("tests/failures/no_typed_input.rs");
}

#[test]
fn example() {
    #[auto_vec]
    fn foo(a: usize, b: usize) -> usize {
        return a + b;
    }

    let results = foo_vec(vec![1,2,3], vec![1,2,3]);
    assert_eq!(vec![2,4,6], results);
}

#[test]
#[should_panic(expected = "Input vectors of not the same length to vectorized function foo_vec")]
fn unequal_vector_inputs() {    
    #[auto_vec]
    fn foo(a: usize, b: usize) -> usize {
        return a + b;
    }

    foo_vec(vec![1, 2, 3], vec![1, 2]);
}

#[test]
fn methods() {
    struct T;

    impl T {
        #[auto_vec]
        fn foo(&self, a: usize, b: usize) -> usize {
            return a + b;
        }
    }

    let t = T{};
    let results = t.foo_vec(vec![1,2], vec![3,4]);
    assert_eq!(vec![4, 6], results);
}

#[test]
fn generics() {
    #[auto_vec]
    fn yay<T: Into<i64>>(
    a: String, 
    b: Vec<i64>, 
    c: T,
    ) -> i64 {
        return a.parse::<i64>().unwrap() + b.iter().sum::<i64>() + c.into();
    }

    let results = yay_vec(vec!["1".to_string(), "2".to_string()], vec![vec![1,2,3], vec![4,5,6]], vec![10, 100] );
    assert_eq!(results, vec![17, 117]);
}