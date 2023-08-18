extern crate proc_macro;

use proc_macro::TokenStream;

#[macro_use]
mod util;

//TODO: split out things that will be common with the method macro
mod command;

#[proc_macro_attribute]
pub fn command(args: TokenStream, input: TokenStream) -> TokenStream {
    command::command(args, input)
}
