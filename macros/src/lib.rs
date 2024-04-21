use darling::FromDeriveInput;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[derive(FromDeriveInput)]
#[darling(attributes(cmd))]
struct CommandOpts {
    #[darling(rename = "subsys")]
    subsystem: syn::Path,
    id: u8,

    req_type: syn::Path,
    rsp_type: syn::Path,
}

#[proc_macro_derive(Command, attributes(cmd))]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = CommandOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let subsystem = opts.subsystem;
    let id = opts.id;
    let req_type = opts.req_type;
    let resp_type = opts.rsp_type;

    let output = quote! {
        impl Command for #ident {
            const ID: CommandID = CommandID {
                subsystem: #subsystem,
                id: #id,
            };
            const REQUEST_TYPE: CommandType = #req_type;
            const RESPONSE_TYPE: CommandType = #resp_type;
        }
    };
    output.into()
}

#[proc_macro_derive(EmptyCommand)]
pub fn empty_command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let DeriveInput { ident, .. } = input;
    let output = quote! {
        impl ser::Command for #ident {
            fn len(&self) -> u8 {
                u8::MIN
            }
            fn data(&self) -> Vec<u8> {
                vec![]
            }
        }
    };
    output.into()
}
