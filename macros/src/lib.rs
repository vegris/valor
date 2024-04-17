use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(EnumIndex)]
pub fn enum_index_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_enum_index_macro(&ast)
}

fn impl_enum_index_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ::common::EnumIndex for #name {
            fn into_index(self) -> usize {
                self as usize
            }
        }
    };
    gen.into()
}
