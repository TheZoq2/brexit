# Brexit

For when you want to get rid of an object, you just need to do some
administration first.

## Usage example

This program will print "The UK has left" after 3 years.

```rust
    struct UnitedKingdom {}

    impl Drop for UnitedKingdom {
        fn drop(&mut self) {
            println!("The UK has left")
        }
    }

    fn main() {
        let uk = UnitedKingdom{};
        brexit(uk);
        thread::sleep(Duration::from_secs(60*24*365*4));
    }
```

I put all the blame @Dylan-DPC for encouraging me to make this ~~abomination~~
very useful crate.
