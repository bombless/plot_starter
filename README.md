# plot_starter

A simple library to quickly plot data using `egui_plot`.

## Usage

Add `plot_starter` to your `Cargo.toml`:

```toml
[dependencies]
plot_starter = { git = "https://github.com/your-repo/plot_starter.git" } # Replace with the actual path once published
```

Then, use it in your code:

```rust
use plot_starter::{Plotter, Chart};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();
    let data = (-500..=500).map(|x| x as f64 / 50.0).map(|x| (x, x.sin()));
    Chart::on(&plotter).data(data);

    plotter.present()
}
```

## Running the Example

To run the included demo, clone the repository and run:

```bash
cargo run --example demo
```