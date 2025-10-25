use plot_starter::{Plotter, Chart, Color, arange};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();

    let plain_data =
        (-500..=500)
            .map(|x| x as f64 / 50.0)
            .map(|x| (x, x.sin()));

    Chart::on(&plotter).data(plain_data).color(Color::BLUE);


    Chart::on(&plotter)
        .data(arange(-10.0 .. 10.0, 0.1).map(|x| (x, x.sin())))
        .color(Color::RED);

    plotter.present()
}
