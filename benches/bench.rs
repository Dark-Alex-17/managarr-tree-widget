use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;
use managarr_tree_widget::{Tree, TreeItem, TreeState};

#[must_use]
fn example_items() -> Vec<TreeItem<&'static str, String>> {
    vec![
        TreeItem::new_leaf("a", "Alfa".to_owned()),
        TreeItem::new(
            "b",
            "Bravo".to_owned(),
            vec![
                TreeItem::new_leaf("c", "Charlie".to_owned()),
                TreeItem::new(
                    "d",
                    "Delta".to_owned(),
                    vec![
                        TreeItem::new_leaf("e", "Echo".to_owned()),
                        TreeItem::new_leaf("f", "Foxtrot".to_owned()),
                    ],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("g", "Golf".to_owned()),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("h", "Hotel".to_owned()),
        TreeItem::new(
            "i",
            "India".to_owned(),
            vec![
                TreeItem::new_leaf("j", "Juliett".to_owned()),
                TreeItem::new_leaf("k", "Kilo".to_owned()),
                TreeItem::new_leaf("l", "Lima".to_owned()),
                TreeItem::new_leaf("m", "Mike".to_owned()),
                TreeItem::new_leaf("n", "November".to_owned()),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("o", "Oscar".to_owned()),
        TreeItem::new(
            "p",
            "Papa".to_owned(),
            vec![
                TreeItem::new_leaf("q", "Quebec".to_owned()),
                TreeItem::new_leaf("r", "Romeo".to_owned()),
                TreeItem::new_leaf("s", "Sierra".to_owned()),
                TreeItem::new_leaf("t", "Tango".to_owned()),
                TreeItem::new_leaf("u", "Uniform".to_owned()),
                TreeItem::new(
                    "v",
                    "Victor".to_owned(),
                    vec![
                        TreeItem::new_leaf("w", "Whiskey".to_owned()),
                        TreeItem::new_leaf("x", "Xray".to_owned()),
                        TreeItem::new_leaf("y", "Yankee".to_owned()),
                    ],
                )
                .expect("all item identifiers are unique"),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("z", "Zulu".to_owned()),
    ]
}

fn init(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("init");
    group.throughput(Throughput::Elements(1)); // Frames per second

    group.bench_function("empty", |bencher| {
        bencher.iter(|| {
            let items = vec![];
            let _ = black_box(Tree::new(black_box(&items))).unwrap();
        });
    });

    group.bench_function("example-items", |bencher| {
        bencher.iter(|| {
            let items = example_items();
            let _ = black_box(Tree::new(black_box(&items))).unwrap();
        });
    });

    group.finish();
}

fn renders(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("render");
    group.throughput(Throughput::Elements(1)); // Frames per second

    let buffer_size = Rect::new(0, 0, 100, 100);

    group.bench_function("empty", |bencher| {
        let items: Vec<TreeItem<usize, String>> = vec![];
        let tree = Tree::new(&items).unwrap();
        let mut state = TreeState::default();
        bencher.iter_batched(
            || (tree.clone(), Buffer::empty(buffer_size)),
            |(tree, mut buffer)| {
                black_box(tree).render(buffer_size, black_box(&mut buffer), black_box(&mut state));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function("example-items", |bencher| {
        let items = example_items();
        let tree = Tree::new(&items).unwrap();
        let mut state = TreeState::default();
        state.open(vec!["b"]);
        state.open(vec!["b", "d"]);
        bencher.iter_batched(
            || (tree.clone(), Buffer::empty(buffer_size)),
            |(tree, mut buffer)| {
                black_box(tree).render(buffer_size, black_box(&mut buffer), black_box(&mut state));
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

/// Create flamegraphs with `cargo bench --bench bench -- --profile-time=5`
#[cfg(unix)]
fn profiled() -> Criterion {
    use pprof::criterion::{Output, PProfProfiler};
    Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)))
}
#[cfg(not(unix))]
fn profiled() -> Criterion {
    Criterion::default()
}

criterion_group! {
    name = benches;
    config = profiled();
    targets = init, renders
}
criterion_main!(benches);
