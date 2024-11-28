# Usage

## TODO

## Test examples from prompts

- One example per part
```rust
elvish::example!(
    Part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "

    Part2: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "
);
```

- Same example for both parts
```rust
elvish::example!("
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
");
```

- More than one example for a part
```rust
elvish::example!(
    Part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "

    Part1: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "

    Part2: "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "
);
```


# Other

## Roadmap

- Library
    - [x] Fetch & save data
    - [ ] Fetch descriptions
        - [ ] Figure out if there is a nice way to get day number at compile time
        - [ ] Put descriptions annotated with proc macros
        - [ ] 
    - [ ] Decide on nice API for tests

- Maybe
    - [ ] Warn when day shouldn't be available yet
    - [ ] Assume current year if not set, but warn about it

- [ ] Port 2023 version
