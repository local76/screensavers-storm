use crate::storm::Storm;
use crate::runner::core::TerminalCell;
use crate::runner::core::screensaver::Screensaver;
use std::time::{Duration, Instant};

#[test]
fn test_screensaver_performance() {
    let mut storm = Storm::new();
    // Prevent slow system info queries during the benchmark
    storm.sys_refresh_timer = -1000.0;
    
    let cols = 80;
    let rows = 24;
    let mut grid = vec![TerminalCell::default(); cols * rows];
    
    let start = Instant::now();
    for _ in 0..100 {
        storm.update(Duration::from_millis(16), cols, rows);
        storm.draw(&mut grid, cols, rows);
    }
    let duration = start.elapsed();
    
    println!("Completed 100 frames of Storm update/draw in {:?}", duration);
    // Assert it finishes within a budget (e.g., 1500ms)
    assert!(duration < Duration::from_millis(1500), "Performance test exceeded budget of 1500ms: {:?}", duration);
}
