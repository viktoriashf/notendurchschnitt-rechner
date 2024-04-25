slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_average_mark({
        let ui_handle = ui.as_weak();

        move |grade1, grade2, grade3, grade4, grade5_2, grade5_8| {
            let ui = ui_handle.unwrap();

            let g1: f64 = grade1.trim().parse().unwrap();
            let g2: f64 = grade2.trim().parse().unwrap();
            let g3: f64 = grade3.trim().parse().unwrap();
            let g4: f64 = grade4.trim().parse().unwrap();
            let g5_2: f64 = grade5_2.trim().parse().unwrap();
            let g5_8: f64 = grade5_8.trim().parse().unwrap();

            let average: f64 = (g1 + g2 + g3 + g4 + (g5_2 * 0.2 + g5_8 * 0.8)) / 5.0;

            let result: String = format!("Notendurschnitt: {:.2}", {average});
            
            ui.set_results(result.into());
        }
    });
    ui.run()
}
