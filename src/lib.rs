use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, Fields, FieldsNamed};
use quote::quote;

use std::sync::atomic::{AtomicBool, Ordering};

static BUILDER_DECLARED: AtomicBool = AtomicBool::new(false);

#[proc_macro_derive(tob)]
pub fn tob(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let fields = input.fields
                      .iter()
                      .filter(|x| x.ident.is_some())
                      .map(|x| x.ident.as_ref());

    let name = &input.ident;

    let object_builder_declaration = create_object_builder_declaration();

    let inner_builder_name = quote::format_ident!("tob_inner_{}_builder", name);

    let fields_1 = fields.clone();
    let fields_2 = fields.clone();
    let fields_3 = fields.clone();
    let fields_4 = fields.clone();

    let funcs = fields.clone().map(|x| {
                    let func_name = quote::format_ident!("set_{}", x.unwrap());
                    quote! {
                        pub fn #func_name<F: 'static>(mut self, f: F) -> Self where
                            F: Fn(usize) -> String {
                                self.#x = Some(Box::new(f));
                                self
                        }
                    }
                });

    let output = quote! {
        #object_builder_declaration

        struct #inner_builder_name {
            index: usize,
            #(#fields_2: Option<Box<dyn FnMut(usize) -> String>>),*
        }

        impl #inner_builder_name {
            pub fn new() -> #inner_builder_name {
                #inner_builder_name {
                    index: 0,
                    #(#fields_3: None),* 
                }
            }

            #(#funcs)*

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

        impl ObjectBuilder<#name> {
            pub fn new() -> #inner_builder_name {
                #inner_builder_name::new()
            }
        }
    };

    TokenStream::from(output)
}

fn create_object_builder_declaration() -> proc_macro2::TokenStream {
    if BUILDER_DECLARED.load(Ordering::Relaxed) {
        return quote! {};
    } 

    BUILDER_DECLARED.store(true, Ordering::SeqCst);
    quote! {
        struct ObjectBuilder<T> { 
            phantom: std::marker::PhantomData<T>
        }
    }
}
