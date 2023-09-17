# My Rust Project

This project is a simple web application that allows the user to roll dice and display the result on the page.

## Usage

To use this application, simply open the `index.html` file in a web browser. Click the "Roll Dice" button to roll the dice and display the result on the page.

## Dependencies

This project requires the following dependencies:

- `js_sys`: A Rust library that provides access to Web APIs from Rust to JavaScript.
- `web_sys`: A Rust library that provides access to Web APIs from Rust to the browser's DOM.

## File Structure

- `Cargo.toml`: Configuration file for Rust's package manager, Cargo.
- `src/lib.rs`: Rust code for the project, including the `roll_dice` function that rolls the dice and updates the DOM.
- `src/utils.rs`: Utility functions used by `src/lib.rs`.
- `index.html`: Entry point for the web application, including the "Roll Dice" button and the result display.
- `README.md`: Documentation for the project.

## Build instruction

cargo install wasm-pack

- cargo build  # build the rust part
- wasm-pack build --target web --release  