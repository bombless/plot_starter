use plot_starter::{Plotter, Chart};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();
    let data = (-500..=500).map(|x| x as f64 / 50.0).map(|x| (x, x.sin()));
    Chart::on(&plotter).data(data);

    plotter.present()
}
