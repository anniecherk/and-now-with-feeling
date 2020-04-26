//! # and-now-with-feeling
//!
//! Macros for returning results


// The good
macro_rules! good {
    ( $($name: ident),* ) => {
        $(
            macro_rules! $name {
                ($some_pat: item) => {
                    Ok($some_pat)
                };
                // ($some_expr: expr) => {
                //     Ok($some_expr)
                // };
            }
        )*
    };
}

// The bad
macro_rules! bad {
    ( $($name: ident),* ) => {
        $(
            macro_rules! $name {
                ($some_expr: expr) => {
                    Err($some_expr)
                };
                ($some_pat: pat) => {
                    Err($some_pat)
                };
            }
        )*
    };
}

// The ugly
good!(Tada, tada, wow, uh_huh, party);
bad!(uh_uh, oh_no, say_it_aint_so, woe);

bad!(test);

good!(yay);
bad!(nay);

fn maybe_divide(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        nay!("")
    } else {
        yay!(a / b)
    }
}

fn parse_default(raw: &str, default: i32) -> i32 {
    let t: Result<i32, i32> = wow!(32+1);
    match raw.parse::<i32>() {
        wow!(n) => n,
        woe!(_) => default,
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(maybe_divide(4, 2), Ok(2));
        assert_eq!(party!(42), Ok::<i32, &str>(42));
    }
}
