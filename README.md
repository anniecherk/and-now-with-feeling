**Friends! Are you sick of returning Results without really expressing them?**

Look no further! We present for your amusement:

# good! and bad!
## macros for redefining Ok and Err... now with feeling!

For example:

```
good!(yay);
bad!(nay);

fn maybe_divide(a: i32, b: i32) -> Result<32, &'static str> {
    if b == 0 {
        nay!("Dividing by zero? No way no how!")
    } else {
        yay!(a / b)
    }
}
```
The exclamation points are mandatory!!! (Limit of one exclamation point per result value please!)

They also work in pattern matches!

```
good!(wow);
bad!(woe);

fn parse_default(raw: &str, default: i32) -> i32 {
    match raw.parse::<i32>() {
        wow!(n) => n,
        woe!(_) => default,
    }
}
```

Other favorite pairs include:
```
uh_huh! / uh_uh!
heck_yeah! / heck_no!
oh_yes! / oh_no!
```

Happy Hacking!
