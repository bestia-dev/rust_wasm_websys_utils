// region: lmake_readme include "readme.md" //! A
//! # rust_wasm_websys_utils
//! 
//! version: 0.4.3  date: 2020-04-25 authors: bestia.dev  
//! **micro crate web_sys and js_sys functions for my project**
//! 
//! 
//! ## web_sys and js_sys
//! 
//! When developing the mem6 game  
//! <https://github.com/bestia-dev/mem6_game>  
//! I needed a lot of web_sys and js_sys functions.  
//! To hide away the javascript part as much as possible I created this library  
//! and fill it with functions I needed for that particular project.  
//! I made a separate crate as a library to share code with other projects.  
//! Functions will be eventually added and also modified or refactored in specialized crates.  
// endregion: lmake_readme include "readme.md" //! A

// This is a library, the functions are not used in this crate.
#![allow(dead_code)]

pub mod websysmod;

