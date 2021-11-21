mod demo;

use demo::Demo;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(Box::new(Demo::default()), options);
}
