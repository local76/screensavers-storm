#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod storm;

fn main() {
    let effect = storm::Storm::new();
    library::screensaver_runner::run_main(effect, "storm");
}
