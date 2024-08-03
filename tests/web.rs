//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
extern crate wasm_game_of_life;
use wasm_game_of_life::pattern::Pattern;
use wasm_game_of_life::universe::Universe;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_blinker() -> Pattern {
    Pattern::from_str("
        000
        111
        000
    ")
}

#[cfg(test)]
pub fn expected_blinker() -> Pattern {
    Pattern::from_str("
        010
        010
        010
    ")
}

#[wasm_bindgen_test]
fn test_can_convert_pattern_to_string() {
    let pattern = input_blinker();
    let expected = "000
111
000";
    assert_eq!(pattern.to_str(), expected);
}

#[wasm_bindgen_test]
fn test_can_equate_patterns() {
    let pattern0 = input_blinker();
    let pattern1 = input_blinker();
    assert_eq!(pattern0, pattern1);
}

#[wasm_bindgen_test]
fn test_tick() {
    let mut universe = Universe::new(5, 5);
    let blinker = input_blinker();
    blinker.place(&mut universe, 1, 1);
    universe.tick();

    let actual = Pattern::from_universe(&universe, 1, 1, 3, 3);
    let expected = expected_blinker();
    assert_eq!(actual, expected);
}

