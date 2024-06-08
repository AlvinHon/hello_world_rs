//! Provides a procedure macro "hello_world" that adds a statement to a function to
//! print out hello world message.
//!
//! ```
//! use hello_world_in_rust_derive::hello_world;
//!
//! #[hello_world]
//! fn main() {
//!  // The macro will generate println! to print the hello world message here.
//!  
//!  // .. The rest of the code ..
//! }
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_quote, ItemFn};

/// Derive proc-macro for printing hello world message at the beginning
/// of a function.
///
/// The effect of this is equivalent to inserting `println!`:
///
/// ```
/// println!("hello world!!");
/// ```
///
/// ### Panics
/// Panic if this macro is not applied on a function.
#[proc_macro_attribute]
pub fn hello_world(_attributes: TokenStream, input: TokenStream) -> TokenStream {
    if let Ok(mut input) = syn::parse::<ItemFn>(input) {
        let word = hello_world::hello();
        input
            .block
            .as_mut()
            .stmts
            .insert(0, parse_quote! { println!("hello {}!", #word); });
        TokenStream::from(quote! { #input })
    } else {
        panic!("This macro `hello_world` can only apply to a function!")
    }
}
