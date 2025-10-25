use std::ops::Range;
use plot_starter::{Plotter, Chart, Color};

fn scale_by_factor(factor: f64, iter: impl Iterator<Item=isize>) -> impl Iterator<Item=f64> {
    iter.map(move |x| x as f64 / factor)
}
struct ARange {
    range: Range<f64>,
    step: f64,
    previous: Option<f64>,
}

impl Iterator for ARange {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if let Some(previous) = self.previous {
            let ret = previous + self.step;
            if ret > self.range.end {
                None
            } else {
                self.previous = Some(ret);
                Some(ret)
            }
        } else {
            self.previous = Some(self.range.start);
            Some(self.range.start)
        }
    }
}

fn arange(range: Range<f64>, step: f64) -> ARange {
    ARange { range, step, previous: None }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plotter = Plotter::new();

    let plain_data = (-500..=500).map(|x| x as f64 / 50.0).map(|x| (x, x.sin()));

    Chart::on(&plotter).data(plain_data).color(Color::BLUE);


    Chart::on(&plotter)
        .data(scale_by_factor(50.0, -500 .. 500).map(|x| (x, 3.0 + x.sin())))
        .color(Color::RED);


    Chart::on(&plotter)
        .data(arange(-10.0 .. 10.0, 0.1).map(|x| (x, 6.0 + x.sin())))
        .color(Color::ORANGE);

    plotter.present()
}
