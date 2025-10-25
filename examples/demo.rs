use plot_starter::{Plotter, Chart, Color, arange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();

    Chart::on(&plotter)
        .time_series(0.1, -10.0 .. 10.0, f64::sin)
        .color(Color::RED);

    Chart::on(&plotter)
        .data(arange(-10.0 .. 10.0, 0.1).map(|x| (x, 3.0 + x.sin())))
        .color(Color::ORANGE);

    plotter.present()
}
