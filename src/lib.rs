//!
//! 
//! A simple library to quickly plot data using `egui_plot`.
//! 
//!  ## Usage
//! 
//! Add `plot_starter` to your `Cargo.toml`:
//! 
//!  ```toml
//!  [dependencies]
//!  plot_starter = "0.34"
//!  ```
//! 
//!  Then, use it in your code:
//! 
//!  ```rust
//!  use plot_starter::{Plotter, Chart, Color, arange};
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let plotter = Plotter::new();
//! 
//!     Chart::on(&plotter)
//!         .data(arange(-10.0 .. 10.0, 0.1).map(|x| (x, x.sin())))
//!         .color(Color::RED);
//! 
//!     plotter.present()
//! }
//!  ```
//! 
//!  ## Running the Example
//! 
//!  To run the included demo, clone the repository and run:
//! 
//!  ```bash
//!  cargo run --example demo
//!  ```

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Range;
use eframe::egui::{self, Color32, Context, CentralPanel};
use eframe::Frame;
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

/// color of line
pub use eframe::egui::Color32 as Color;

/// Represents a chart to be plotted.
///
/// A `Chart` is created using `Chart::on(&plotter)` and can be customized by chaining methods like `data` and `color`.
 pub struct Chart<'a> {
    id: usize,
    plotter: &'a Plotter,
}

impl<'a> Chart<'a> {
    /// Creates a new chart on the given `Plotter`.
    ///
    /// # Arguments
    ///
    /// * `plotter` - A reference to the `Plotter` that will display the chart.
    pub fn on(plotter: &'a Plotter) -> Self {
        Chart { id: plotter.next_id(), plotter }
    }
    /// Sets the data for the chart.
    ///
    /// # Arguments
    ///
    /// * `data` - An iterator of tuples, where each tuple represents a point (x, y) in the chart.
    pub fn data(&'a self, data: impl Iterator<Item = (f64, f64)>) -> &'a Self {
        self.plotter.data(self.id, data.map(|(x, y)| PlotPoint {x, y}).collect());
        self
    }
    /// Sets data from a step, a range and a callback function.
    ///
    /// example:
    /// ```rust
    /// use plot_starter::{Plotter, Chart, Color, arange};
    /// let plotter = Plotter::new();
    ///
    /// Chart::on(&plotter)
    ///  .time_series(0.1, -10.0 .. 10.0, f64::sin)
    ///  .color(Color::RED);
    /// ```
    pub fn time_series(&'a self, step: f64, range: Range<f64>, mut f: impl FnMut(f64) -> f64) -> &'a Self {
        self.plotter.data(self.id, arange(range, step).map(|x| PlotPoint {x, y: f(x)}).collect());
        self
    }
    /// Sets the color of the chart.
    ///
    /// # Arguments
    ///
    /// * `color` - The `Color32` to use for the chart's line.
    pub fn color(&'a self, color: Color32) -> &'a Self {
        self.plotter.color(self.id, color);
        self
    }
}

struct ChartData {
    data: Vec<PlotPoint>,
    color: Color32,
}

impl ChartData {
    fn new(data: Vec<PlotPoint>, color: Color32) -> Self {
        ChartData { data, color }
    }
}

struct PlotterApp {
    charts: HashMap<usize, ChartData>,
}

impl From<Plotter> for PlotterApp {
    fn from(plotter: Plotter) -> Self {
        Self {
            charts: plotter.charts.take()
        }
    }
}

/// The main struct for creating and managing plots.
///
/// The `Plotter` is the entry point for creating charts and displaying them in a window.
pub struct Plotter {
    next_id: RefCell<usize>,
    charts: RefCell<HashMap<usize, ChartData>>,
}

impl Plotter {
    /// Creates a new `Plotter` instance.
    pub fn new() -> Self {
        Self {
            charts: RefCell::new(HashMap::new()),
            next_id: RefCell::new(0),
        }
    }
    fn next_id(&self) -> usize {
        let mut id = self.next_id.borrow_mut();
        let ret = *id;
        *id = ret + 1;
        ret
    }
    fn data(&self, id: usize, data: Vec<PlotPoint>) {
        let mut charts = self.charts.borrow_mut();
        if let Some(chart) = charts.get_mut(&id) {
            chart.data = data;
        } else {
            charts.insert(id, ChartData::new(data, Color32::TRANSPARENT));
        }
    }
    fn color(&self, id: usize, color: Color32) {
        let mut charts = self.charts.borrow_mut();
        if let Some(chart) = charts.get_mut(&id) {
            chart.color = color;
        } else {
            charts.insert(id, ChartData::new(Vec::new(), color));
        }
    }

    /// Displays the plot in a native window.
    ///
    /// This method consumes the `Plotter` and runs the `eframe` application loop.
    pub fn present(self) -> Result<(), Box<dyn Error>> {

        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1280.0, 800.0])
                .with_title("plot_starter"),
            ..Default::default()
        };

        eframe::run_native(
            "plot_starter",
            native_options,
            Box::new(|_cc| {
                Ok(Box::new(PlotterApp::from(self)))
            }),
        )?;

        Ok(())
    }

}

impl eframe::App for PlotterApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            Plot::new("").show(ui, |plot_ui| {
                for data in self.charts.values() {
                    let points = Line::new("", PlotPoints::Borrowed(&data.data)).color(data.color);
                    plot_ui.line(points)
                }
            });
        });
    }
}

/// Data structure to be used by `arange()`
pub struct ARange {
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

/// Create f64 stream with step within range.
///
/// e.g.
/// ```rust
/// use plot_starter::arange;
/// arange(-10.0 .. 10.0, 0.1);
pub fn arange(range: Range<f64>, step: f64) -> ARange {
    ARange { range, step, previous: None }
}
