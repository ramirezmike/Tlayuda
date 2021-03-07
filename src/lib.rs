use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Fields, FieldsNamed};
use quote::quote;

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}


#[proc_macro_derive(tob)]
pub fn tob(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let fields = input.fields
                      .iter()
                      .filter(|x| x.ident.is_some())
                      .map(|x| x.ident.as_ref());

    let name = &input.ident;
    let builder_name = quote::format_ident!("tob_{}_builder", name);

    let output = quote! {
        struct #builder_name {
            index: usize
        }
        impl #builder_name {
            pub fn new() -> Self {
                #builder_name {
                    index: 0
                }
            }

            fn take_index(&mut self) -> usize {
                self.index = self.index + 1;
                self.index - 1
            }

            pub fn build(&mut self) -> #name {
                let i = self.take_index();
                #name {
                    #(#fields: format!("{}{}", stringify!(#fields), i)),*
                }
            }

            pub fn build_vec(&mut self) -> Vec::<#name> {
                std::iter::repeat_with(|| self.build()).take(5).collect()
            }
        }

        impl #name {
            pub fn tob() -> #builder_name {
                #builder_name::new()
            }
        }
    };

    TokenStream::from(output)
}
