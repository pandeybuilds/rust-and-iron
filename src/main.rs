// ============================================================
// Rust Fundamentals: Day 1
// Topics: variables, mutability, data types, functions
// ============================================================

fn main() {
    hello_world();

    println!("\n=== Variables & Mutability ===");
    variables_and_mutability();

    println!("\n=== Basic Data Types ===");
    data_types();

    println!("\n=== Functions ===");
    functions_demo();
}

fn hello_world() {
    println!("Hello, world!");
}

// ---- Variables & Mutability --------------------------------

fn variables_and_mutability() {
    // Variables are immutable by default in Rust.
    // The compiler refuses to compile code that reassigns an immutable variable.
    let x = 5;
    println!("Immutable x = {x}");

    // `mut` opts the binding into mutability.
    let mut count = 0;
    count += 1;
    println!("Mutable count after increment = {count}");

    // Shadowing: re-declare with `let` in the same scope.
    // Different from mutation — the new binding can even change type.
    let x = x * 2; // shadows the earlier x
    let x = x + 1; // shadows again
    println!("Shadowed x (5 * 2 + 1) = {x}");

    // Constants: always immutable, type annotation required, evaluated at compile time.
    const MAX_POINTS: u32 = 100_000; // underscores improve readability in numeric literals
    println!("Constant MAX_POINTS = {MAX_POINTS}");
}

// ---- Basic Data Types -------------------------------------

fn data_types() {
    // --- Integers ---
    // Rust infers `i32` by default for integer literals.
    let signed: i32 = -42;         // signed: i8, i16, i32, i64, i128, isize
    let unsigned: u64 = 1_000_000; // unsigned: u8, u16, u32, u64, u128, usize
    let byte: u8 = 255;            // u8 is Rust's "byte" type
    println!("signed i32={signed}, unsigned u64={unsigned}, byte u8={byte}");

    // Integer overflow panics in debug builds and wraps in release builds.
    // Use checked_*/wrapping_*/saturating_* methods for explicit control.
    let checked = 200u8.checked_add(100); // returns Option<u8>
    println!("200u8 + 100 (checked) = {checked:?}"); // None — would overflow

    // --- Floats ---
    // f64 is the default (double precision). f32 is single precision.
    let pi: f64 = 3.141_592_653_589_793;
    let small: f32 = 1.5;
    println!("pi f64={pi:.6}, small f32={small}");

    // --- Booleans ---
    // Exactly one byte. Values are `true` or `false`.
    let is_learning: bool = true;
    let has_errors: bool = false;
    println!("is_learning={is_learning}, has_errors={has_errors}");

    // --- Characters ---
    // `char` is a Unicode scalar value — 4 bytes, supports any Unicode character.
    let letter: char = 'R';
    let emoji: char = '\u{1F980}'; // Rust mascot crab (Unicode escape)
    println!("letter='{letter}', emoji='{emoji}'");

    // --- Strings ---
    // &str  — string slice: an immutable reference to UTF-8 bytes (often a literal).
    // String — heap-allocated, growable, owned UTF-8 string.
    let greeting: &str = "Hello";          // string slice (borrowed)
    let mut owned: String = String::from("World"); // owned String
    owned.push_str("!"); // String is growable
    println!("{greeting}, {owned}");

    // Convert between them:
    let _slice: &str = &owned;               // &String coerces to &str automatically
    let _owned2: String = greeting.to_string(); // &str -> String

    // --- Tuples ---
    // Fixed-length, heterogeneous. Access elements with .0, .1, …
    let point: (f64, f64) = (3.0, -1.5);
    let (px, py) = point; // destructuring
    println!("point = ({px}, {py}), direct = ({}, {})", point.0, point.1);

    // --- Arrays ---
    // Fixed length, same type, stack-allocated. Use Vec<T> when size varies.
    let primes: [u32; 5] = [2, 3, 5, 7, 11];
    println!("primes = {primes:?}, third = {}", primes[2]);
}

// ---- Functions --------------------------------------------

// Functions are declared with `fn`. Parameter and return types are required.
// Rust uses snake_case for function names.

// The last expression in a function body is the implicit return value (no semicolon).
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Return multiple values via a tuple.
fn min_max(slice: &[i32]) -> (i32, i32) {
    let mut min = slice[0];
    let mut max = slice[0];
    for &val in slice {
        if val < min {
            min = val;
        }
        if val > max {
            max = val;
        }
    }
    (min, max) // implicit return
}

// A function returning `()` (unit) is like `void` in other languages.
fn greet(name: &str) {
    println!("Hello, {name}! Welcome to Rust.");
}

fn functions_demo() {
    let sum = add(10, 32);
    println!("add(10, 32) = {sum}");

    let numbers = [4, -2, 17, 0, -8, 3];
    let (lo, hi) = min_max(&numbers);
    println!("min={lo}, max={hi} in {numbers:?}");

    greet("rustacean");
}
