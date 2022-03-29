# rust_wasm_websys_utils

[comment]: # (lmake_readme cargo.toml data start)

**micro crate web_sys and js_sys functions for my project**  
***version: 0.4.3  date: 2020-04-25 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/rust_wasm_websys_utils)***  

[comment]: # (lmake_readme cargo.toml data end)  

## web_sys and js_sys

When developing the mem6 game  
<https://github.com/bestia-dev/mem6_game>  
I needed a lot of web_sys and js_sys functions.  
To hide away the javascript part as much as possible I created this library  
and fill it with functions I needed for that particular project.  
I made a separate crate as a library to share code with other projects.  
Functions will be eventually added and also modified or refactored in specialized crates.  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://web.crev.dev/rust-reviews/crate/num-traits/>  
