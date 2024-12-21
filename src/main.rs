// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

use slint::SharedString;

slint::include_modules!();

const MOMS: f64 = 0.25;
const TOLD: f64 = 0.05;
const IMPORT: f64 = 165.00;

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_divide_income({
        let ui_handle = ui.as_weak();
        move |pris: SharedString, fragt: SharedString| {
            let ui = ui_handle.unwrap();
            let pris_int: f64 = pris.trim().parse().unwrap();
            let fragt_int: f64 = fragt.trim().parse().unwrap();
            let samlet_pris: String = beregn_pris(pris_int, fragt_int);
            ui.set_result(samlet_pris.into());
        }
    });

    ui.run()?;

    Ok(())
}

//f64 choosen for ~15-17 decimal digits of precision on financial calculations
//The performance difference between f32 and f64 on modern hardware is negligible
fn beregn_pris (pris: f64, fragt: f64) -> String {
    let samlet_pris: f64 = pris + fragt;
    if pris > 1150.00 {
        let told: f64 = samlet_pris * TOLD;
        let moms: f64 = samlet_pris * MOMS;
        return build_string(told, moms, samlet_pris);
    } 
    let told: f64 = 0.00;
    let moms: f64 = samlet_pris * MOMS;
    return build_string(told, moms, samlet_pris);
}

fn build_string(told: f64, moms: f64, samlet_pris: f64) -> String {
    let result: f64 = told + moms + IMPORT + samlet_pris;
    let result: String = format!("
    Told(5%): {:>8.2}
    Moms(25%): {:>8.2}
    Import gebyr: {:>8.2}
    Samlet pris: {:>8.2}",
    told, moms, IMPORT, result);
    return result;
}
