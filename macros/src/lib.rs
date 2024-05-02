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
}

#[proc_macro_derive(Command, attributes(cmd))]
pub fn command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = CommandOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;

    let subsystem = opts.subsystem;
    let id = opts.id;

    let output = quote! {
        impl Command for #ident {
            const ID: CommandID = CommandID {
                subsystem: #subsystem,
                id: #id,
            };
        }
    };
    output.into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(req))]
struct RequestOpts {
    kind: syn::Path,
}

#[proc_macro_derive(EmptyReq, attributes(req))]
pub fn empty_command_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = RequestOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let req_type = opts.kind;
    let output = quote! {
        impl ser::Command for #ident {
            const REQUEST_TYPE: CommandType = #req_type;
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

#[derive(FromDeriveInput)]
#[darling(attributes(rsp))]
struct ResponseOpts {
    kind: syn::Path,
}

#[proc_macro_derive(PassRsp, attributes(rsp))]
pub fn passthrough_response_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    let opts = ResponseOpts::from_derive_input(&input).expect("Wrong options");
    let DeriveInput { ident, .. } = input;
    let rsp_type = opts.kind;
    let output = quote! {
        impl de::Command for #ident {
            const RESPONSE_TYPE: CommandType = #rsp_type;
            type Output = Vec<u8>;
            fn to_output(&self, data_frame: Vec<u8>) -> Result<Self::Output, de::Error> {
                Ok(data_frame) // passthrough
            }
        }
    };
    output.into()
}
