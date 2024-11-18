# Managarr Tree Widget

[Ratatui](https://docs.rs/ratatui) Widget built to show Tree Data structures.

![Screenshot](media/screenshot.png)

## Installation
Add this widget to your project using the following command: 

```shell
cargo add managarr-tree-widget
```

## Running the example
To run the example widget, simply run:

```shell
cargo run --example example
```

## Usage
The following is an example of how to create a tree of strings (namely one like the one used in the [example](./examples/example.rs)):

```rust
fn draw(&mut self, frame: &mut Frame) {
    let area = frame.area();
    let tree_items = vec![
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
    ];
    let widget = Tree::new(&tree_items)
        .expect("all item identifiers are unique")
        .block(
            Block::bordered()
                .title("Tree Widget"),
        )
        .highlight_style(
            Style::new()
                .fg(Color::Black)
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    frame.render_stateful_widget(widget, area, &mut self.state);
}
```

This will generate the following tree structure:

```
┌── Alfa
├── Bravo
│   ├── Charlie
│   ├── Delta
│   │ ├── Echo
│   │ └── Foxtrot
│   └── Golf
└── Hotel
```

This example assumes the existence of a `self.state` field that is initialized with `TreeState::default()`. The `TreeItem` struct is used to create a tree of items, and the `Tree` struct is used to create the widget itself.

A more detailed and feature-complete example is available in the [example](./examples/example.rs) file.

## Credit
The original project for this widget is the [Ratatui Tree Widget](https://github.com/EdJoPaTo/tui-rs-tree-widget), which was purpose built for the specific use
case of [`mqttui`](https://github.com/EdJoPaTo/mqttui).

The updated version of the tree widget that allows more generic types is created by me, [Alex Clarke](https://github.com/Dark-Alex-17).

