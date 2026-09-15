use std::ops::Mul;

use eframe::egui;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "cappuccinocosmico",
        native_options,
        Box::new(|cc| Ok(Box::new(SiteApp::new(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast;

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async move {
        let document = web_sys::window()
            .expect("browser window must exist")
            .document()
            .expect("browser document must exist");

        let canvas = document
            .get_element_by_id("site_canvas")
            .expect("index.html must contain a #site_canvas element")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("#site_canvas must be a canvas element");

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(SiteApp::new(cc)))),
            )
            .await
            .expect("eframe web app must start");
    });
}

#[derive(Clone, Copy)]
struct Cf64 {
    re: f64,
    im: f64,
}

impl Mul for Cf64 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

struct SiteApp {
    started_at: web_time::Instant,
}

impl SiteApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            started_at: web_time::Instant::now(),
        }
    }
}

impl eframe::App for SiteApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("cappuccinocosmico");
        ui.label(format!(
            "scaffold alive for {:.0}s",
            self.started_at.elapsed().as_secs_f32()
        ));
    }
}
