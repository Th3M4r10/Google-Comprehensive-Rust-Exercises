# Google Comprehensive Rust 🦀

**This is the Rust fundamental course developed by Google and these are the exercises from the Google Comprehensive Rust.**

To clone the repository:


```bash
git clone https://github.com/Th3M4r10/Google-Comprehensive-Rust-Exercises
cd Google-Comprehensive-Rust-Exercises
```
## Running the Code
**`rustc` is used for compiling and running individual tests**

To run the Rust code using `rustc`, navigate to the specific exercise directory and use the following commands:

```bash
cd <exercise_directory>
rustc sol.rs    
./sol           # Execute the compiled binary
```

### Testing

For compiling and running individual tests with `rustc`, use the following command:

```bash
cd exercise_directory
rustc --test sol.rs   
./sol
```

### Deployment

For deployment or building a release version of your Rust project, use the following `rustc` command with optimization level  3:

```bash
rustc --opt-level 3 -o exe sol.rs
./exe
```
## Using Cargo

### Run

Navigate to the specific exercise directory and use the following commands:

```bash
cd <exercise_directory>
cargo run
```

### Testing

For running all tests in the project, it's recommended to use `cargo test`:

```bash
cd <exercise_directory>
cargo test
```

### Deployment

For deployment or building a release version of your Rust project, use the following `cargo` command:

```bash
cd <exercise_directory>
cargo build --release   # We can use without specifying an optimization level 
./target/release/<binary_name>

```

This will generate an optimized executable in the `target/release/` directory.

### Additional Resources
- [Google Comprehensive Rust Course](https://google.github.io/comprehensive-rust/)
- [Comprehensive Rust](https://github.com/google/comprehensive-rust)
- [Rust Documentation](https://doc.rust-lang.org/)
- [Cargo Guide](https://doc.rust-lang.org/cargo/)
- [Rust Testing](https://doc.rust-lang.org/book/ch11-00-testing.html)

Please note: Replace `<exercise_directory>` with the actual directory name of the exercise you are working on.