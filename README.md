# Auto-Vec
Auto-vec is simple proc macro to vectorize your scalar functions.

## Example
```rust
    #[auto_vec]
    pub fn foo(arg1: i64, arg2: i32) -> i64 {
        return (arg1 + arg2 as i64);
    }
```
Allowing you to do something like this:
```rust
    fn main() {
        let results = foo_vec(vec![1,2,3,4], vec![1,2,3,4]);
        // returned vector is of form [foo(arg1[0], arg2[0]), ...]
        assert_eq!(results, vec![2, 4, 6, 8]);
    }
```

It keeps the original function and allows computation of the specified function over a vector of inputs. The generated function inherits the visibility and generics from the original function definition. The generated function takes the vectors as mutable so as to take ownership of the inner elements.

The vectorized function expects to get input vectors of the equal length, and panics if that is not the case.
```rust
    // Panics due to unequal length of input vectors
    foo_vec(vec![1,2,3], vec![1,2]);
```

Auto vec also requires the function to have one or more input types(other than self) and a return type, and fails to compile otherwise.

```rust
    #[auto_vec]
    pub fn bar() -> String; // Compile time error

    #[auto_vec]
    pub fn ha(a:i64); // Compile time error
```

It allows vectorization of methods too, accepting vectors for every argument except the first self argument.
```rust
    struct T;
    impl T {
        #[auto_vec]
        fn foo(&self, arg1: usize, arg2: usize) -> usize {
            return arg1 + arg2;
        }
    }
    fn main() {
        let a = T{};
        assert_eq!(a.foo_vec(vec![1, 2, 3], vec![1, 2, 3]), vec![2, 4, 6])
    }
```

**Warning**: Methods accepting self receviers with a specific type, such as `self: Box<Self>` are not parsed correctly currently and lead to wrong code generation. Auto Vec currently works only for untyped `self` receviers.

## License
Auto-Vec is under the MIT license. See the [LICENSE](LICENSE) file for details.