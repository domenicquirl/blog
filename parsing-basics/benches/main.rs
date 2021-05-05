use parsing_basics::{lexer::Lexer, parser::Parser};
use std::time::Duration;
use unindent::unindent;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};

pub fn lex_function(c: &mut Criterion) {
    let input = r#"
        // tests stuff
        fn test(var: Type, var2_: bool) {
            let x = "String content \" test" + 7 / 27.3e-2^4;
            let chars = x.chars();
            if let Some(c) = chars.next() {
                x = x + c;
            } else if !var2_ {
                x = x + ",";
            }
        }
    "#;
    let input = unindent(input);
    bench_lexer(c, "function", input.as_str());
}

pub fn lex_struct(c: &mut Criterion) {
    let input = r#"
        struct Foo<T> {
            bar: Bar<T>,
        }
    "#;
    let input = unindent(input);
    bench_lexer(c, "struct", input.as_str());
}

fn bench_lexer(c: &mut Criterion, name: &str, input: &str) {
    // Lexing: measured in bytes
    let mut group = c.benchmark_group("lexer");
    group.measurement_time(Duration::from_millis(7500));

    // To measure throughput, we need to tell `criterion`
    // how big our input is.
    group.throughput(Throughput::Bytes(input.as_bytes().len() as u64));
    group.bench_with_input(name, input, |b, input| {
        b.iter_batched(
            || Lexer::new(input),         // <- Our lexer is made HERE
            |mut lexer| lexer.tokenize(), // <- and runs HERE
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

pub fn parse_file(c: &mut Criterion) {
    let input = r#"
        fn wow_we_did_it(x: String, bar: Bar<Baz<T>, U>) {
            let x = 7 + sin(y);
            {
                x = 3;
                if (bar < 3) {
                    x = x + 1;
                    y = 3 * x;
                } else if (bar < 2) {
                    let i = 2!;
                    x = x + i;
                } else {
                    x = 1;
                }
            }
        }

        struct Foo<T, U> {
            x: String,
            bar: Bar<Baz<T>, U>
        }
    "#;
    let input = unindent(input);
    bench_parser(c, "file", input.as_str());
}

fn bench_parser(c: &mut Criterion, name: &str, input: &str) {
    let mut group = c.benchmark_group("parser");
    group.measurement_time(Duration::from_secs(10));

    group.throughput(Throughput::Bytes(input.as_bytes().len() as u64));
    group.bench_with_input(name, input, |b, input| {
        b.iter_with_setup(
            || Parser::new(input),
            |mut parser| {
                let _tree = parser.file();
            },
        )
    });
    group.finish();
}

criterion_group!(benches, lex_function, lex_struct, parse_file);
criterion_main!(benches);
