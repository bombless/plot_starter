use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use eframe::egui::{self, Color32, Context, CentralPanel};
use eframe::Frame;
use egui_plot::{Line, Plot, PlotPoint, PlotPoints};

pub struct Chart<'a> {
    id: usize,
    plotter: &'a Plotter,
}

impl<'a> Chart<'a> {
    pub fn on(plotter: &'a Plotter) -> Self {
        Chart { id: plotter.next_id(), plotter }
    }
    pub fn data(&'a self, data: impl Iterator<Item = (f64, f64)>) -> &'a Self {
        self.plotter.data(self.id, data.map(|(x, y)| PlotPoint {x, y}).collect());
        self
    }
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

pub struct PlotterApp {
    charts: HashMap<usize, ChartData>,
}

impl From<Plotter> for PlotterApp {
    fn from(plotter: Plotter) -> Self {
        Self {
            charts: plotter.charts.take()
        }
    }
}

pub struct Plotter {
    next_id: RefCell<usize>,
    charts: RefCell<HashMap<usize, ChartData>>,
}

impl Plotter {
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
                    let points = Line::new("", PlotPoints::Borrowed(&data.data));
                    plot_ui.line(points)
                }


            });
        });
    }
}