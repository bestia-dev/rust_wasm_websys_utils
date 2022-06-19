# rust_wasm_websys_utils

[comment]: # (lmake_readme cargo.toml data start)

**micro crate web_sys and js_sys functions for my project**  
***version: 0.4.3  date: 2020-04-25 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/rust_wasm_websys_utils)***  

[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Frust_wasm_websys_utils&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

[comment]: # (lmake_readme cargo.toml data end)  

Hashtags: #rustlang #tutorial #web #wasm #webassembly  
My projects on Github are more like a tutorial than a finished product: [bestia-dev tutorials](https://github.com/bestia-dev/tutorials_rust_wasm).

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
