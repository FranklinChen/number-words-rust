use number_words::{default_config, NumberParser, NaiveParser, DfsParser, MemoizedParser};
use divan::Bencher;

fn main() {
    divan::main();
}

#[divan::bench(args = [10, 15, 20, 25])]
fn parse_digits_naive(bencher: Bencher, len: usize) {
    let parser = NaiveParser::new(&default_config());
    let input = "1".repeat(len);

    bencher.bench(|| {
        parser.parse(&input)
    });
}

#[divan::bench(args = [10, 15, 20, 25, 30])]
fn parse_digits_dfs(bencher: Bencher, len: usize) {
    let parser = DfsParser::new(&default_config());
    let input = "1".repeat(len);

    bencher.bench(|| {
        parser.parse(&input)
    });
}

#[divan::bench(args = [10, 15, 20, 25, 30])]
fn parse_digits_memoized(bencher: Bencher, len: usize) {
    let parser = MemoizedParser::new(&default_config());
    let input = "1".repeat(len);

    bencher.bench(|| {
        parser.parse(&input)
    });
}
