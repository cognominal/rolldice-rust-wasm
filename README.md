# A sample app using rust, wasm, the js_sys and web_sys crates

## the project

This project is a sample web application that allows the user to roll dice and display the result on the page.
The roll is done by pressing the (button)[https://github.com/cognominal/rolldice-rust-wasm/blob/main/index.html#L10]
this calls 
[roll_dice](https://github.com/cognominal/rolldice-rust-wasm/blob/main/src/lib.rs#L7)
in src/lib.rs which has been compiled to was.
[add_roll_to_dom](https://github.com/cognominal/rolldice-rust-wasm/blob/main/src/lib.rs#L17) call back into js land.
This should probably done in js but it shows the use of the `web_sys`crate
as I asked to github copilot.


I started by using the quick [chat copilot](https://docs.github.com/en/copilot/github-copilot-chat/using-github-copilot-chat), iterated on /createWorkspace prompt and eventually pressed the button `create Workspace`.
There was many thing missing or astray but that was better than doing it from scratch

## Not quite there yet

Uncaught TypeError: Cannot read properties of undefined (reading 'roll_dice')

## TBD

doing it in a container. A good way to see everything what is needed. I have not
kept track.

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

Assuming that rust and node are installed

- cargo install wasm-pack                  # maybe or not, cuz I added it as dev dependancy
- cargo build                              # build the rust part
- wasm-pack build --target web --release   # build the web part