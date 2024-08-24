From the output, we can see that Rust places the results of the tests in separate sections. Unit test results come first, then integration results, and finally, documentation results.

In the integration tests section, we can see that our two tests inside the tests/pizzas.rs file were collected and executed by the test suite.

Only library crates can be tested via integration tests because binary crates don't expose any functionality that other crates can use. As a result, many Rust binary crates include a src/lib.rs file that contains most of the code in src/main.rs. Integration tests can then test the binary's functionality by importing the crate as a library with use.