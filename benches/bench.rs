use std::hint::black_box;

use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use managarr_tree_widget::{Tree, TreeItem, TreeState};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::StatefulWidget;

#[must_use]
fn example_items() -> Vec<TreeItem<&'static str>> {
    vec![
        TreeItem::new_leaf("Alfa"),
        TreeItem::new(
            "Bravo",
            vec![
                TreeItem::new_leaf("Charlie"),
                TreeItem::new(
                    "Delta",
                    vec![TreeItem::new_leaf("Echo"), TreeItem::new_leaf("Foxtrot")],
                )
                .expect("all item identifiers are unique"),
                TreeItem::new_leaf("Golf"),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("Hotel"),
        TreeItem::new(
            "India",
            vec![
                TreeItem::new_leaf("Juliet"),
                TreeItem::new_leaf("Kilo"),
                TreeItem::new_leaf("Lima"),
                TreeItem::new_leaf("Mike"),
                TreeItem::new_leaf("November"),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("Oscar"),
        TreeItem::new(
            "Papa",
            vec![
                TreeItem::new_leaf("Quebec"),
                TreeItem::new_leaf("Romeo"),
                TreeItem::new_leaf("Sierra"),
                TreeItem::new_leaf("Tango"),
                TreeItem::new_leaf("Uniform"),
                TreeItem::new(
                    "Victor",
                    vec![
                        TreeItem::new_leaf("Whiskey"),
                        TreeItem::new_leaf("Xray"),
                        TreeItem::new_leaf("Yankee"),
                    ],
                )
                .expect("all item identifiers are unique"),
            ],
        )
        .expect("all item identifiers are unique"),
        TreeItem::new_leaf("Zulu"),
    ]
}

fn init(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("init");
    group.throughput(Throughput::Elements(1)); // Frames per second

    group.bench_function("empty", |bencher| {
        bencher.iter(|| {
            let items = Vec::<TreeItem<String>>::new();
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
        let items: Vec<TreeItem<String>> = vec![];
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
        state.open(vec![2]);
        state.open(vec![2, 4]);
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
