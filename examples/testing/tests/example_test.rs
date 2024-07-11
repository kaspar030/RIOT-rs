#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::println;

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    // Optional: A init function which is called before every test
    #[init]
    fn init() {}

    // A test which takes the state returned by the init function (optional)
    #[test]
    fn trivial() {
        assert!(true)
    }
}
