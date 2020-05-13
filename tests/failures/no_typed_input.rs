use auto_vec::auto_vec;

struct T;

impl T {
    #[auto_vec]
    fn foo(&self) -> usize {
        return 5;
    }
}

fn main() {}