//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn universe_tick() {
    let mut expected_univ = Universe::new(6, 6);
    expected_univ.make_all_dead();
    expected_univ.make_cells_alive(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);

    let mut actual_univ = Universe::new(6, 6);
    actual_univ.make_all_dead();
    actual_univ.make_cells_alive(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);

    actual_univ.tick();

    assert_eq!(expected_univ.cells_as_slice(), actual_univ.cells_as_slice());
}
