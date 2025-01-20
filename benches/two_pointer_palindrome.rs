#![allow(unused)]
fn main() {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn two_pointer_palindrome(array: &Vec<String>) -> bool {
        let mut left_array_index = 0;
        let mut right_array_index = array.len() - 1;

        while left_array_index < right_array_index {
            if array[left_array_index] != array[right_array_index] {
                return false;
            }
            left_array_index += 1;
            right_array_index -= 1;
        }
        true
    }

    fn criterion_benchmark(c: &mut Criterion) {
        let test_input: Vec<String> = "Bob".chars().map(|c| c.to_string()).collect();
        c.bench_function("Two Pointer Palindrome", |b| {
            b.iter(|| two_pointer_palindrome(black_box(&test_input)))
        });
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);
}
