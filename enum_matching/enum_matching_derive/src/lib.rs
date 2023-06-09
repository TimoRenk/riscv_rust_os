use proc_macro2::{Ident, TokenStream};
use quote::quote;

/// Adds a [TryFrom] implementation for enums to create an enum variant from an [isize].
#[proc_macro_derive(EnumTryFrom)]
pub fn enum_matching_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input).unwrap();
    let output: TokenStream = impl_enum_matching(&ast);
    output.into()
}
fn impl_enum_matching(ast: &syn::DeriveInput) -> TokenStream {
    if let syn::Data::Enum(data) = &ast.data {
        let enum_name = &ast.ident;

        let each_variant: Vec<Ident> = data.variants.iter().map(|i| i.ident.clone()).collect();
        quote! {
            impl TryFrom<isize> for #enum_name {
                type Error = enum_matching::Error;

                fn try_from(num: isize) -> Result<Self, enum_matching::Error> {
                    #![allow(non_upper_case_globals)]
                    #(
                        const #each_variant: isize = #enum_name::#each_variant as isize;
                    ) *

                    match num {
                        #(#each_variant => return Ok(Self::#each_variant),) *
                        _ => Err(enum_matching::Error{num})
                    }
                }
            }
        }
    } else {
        panic!("Enum matching is not possible with structs or unions");
    }
}
