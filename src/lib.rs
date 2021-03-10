use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Type};

#[proc_macro_derive(tob)]
pub fn tob(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);
    let fields = input
        .fields
        .iter()
        .filter(|x| x.ident.is_some())
        .map(|x| (x.ident.as_ref(), x.ty.clone()));

    let source_struct_name = &input.ident;
    let inner_builder_name = quote::format_ident!("TobInner{}Builder", source_struct_name);

    let field_setter_functions = fields.clone().map(|(x, t)| {
        let func_name = quote::format_ident!("set_{}", x.unwrap());
        quote! {
            pub fn #func_name<F: 'static>(mut self, f: F) -> Self where
                F: Fn(usize) -> #t {
                    self.#x = Box::new(f);
                    self
            }
        }
    });

    let field_builder_intializers = fields.clone().map(|(x, t)| {
        let identity = match t {
            Type::Path(type_path) => match type_path.path.get_ident() {
                Some(ident) => ident.clone(),
                _ => todo!("Path Type {:?} not supported", type_path),
            },
            _ => todo!("Type {:?} not supported", t),
        };

        let f = match identity.to_string().as_str() {
            "String" => quote! { |i| format!("{}{}", stringify!(#x), i).into() },
            "char" => quote! { |i| std::char::from_digit(i as u32, 10).unwrap_or('a') },
            "bool" => quote! { |i| false },
            t @ "i8"
            | t @ "i16"
            | t @ "i32"
            | t @ "u8"
            | t @ "u16"
            | t @ "u32"
            | t @ "i64"
            | t @ "i128"
            | t @ "isize"
            | t @ "u64"
            | t @ "u128"
            | t @ "usize"
            | t @ "f32"
            | t @ "f64" => {
                let parameter_type = quote::format_ident!("{}", t);
                quote! { |i| i as #parameter_type }
            }
            _ => todo!("Identity not implemented {:?}", identity),
        };

        quote! { #x: Box::new(#f), }
    });

    let field_declarations = fields.clone().map(|(x, t)| {
        quote! {
            #x: Box<dyn FnMut(usize) -> #t>,
        }
    });

    let fields = fields.map(|(x, _)| x);

    let output = quote! {
        pub struct #inner_builder_name {
            index: usize,
            #(#field_declarations)*
        }

        impl #inner_builder_name {
            pub fn new() -> #inner_builder_name {
                #inner_builder_name {
                    index: 0,
                    #(#field_builder_intializers)*
                }
            }

            #(#field_setter_functions)*

            pub fn with_index(mut self, index: usize) -> Self {
                self.index = index;
                self
            }

            fn take_index(&mut self) -> usize {
                self.index = self.index + 1;
                self.index - 1
            }

            pub fn build(&mut self) -> #source_struct_name {
                let i = self.take_index();
                #source_struct_name {
                    #(#fields: self.#fields.as_mut()(i)),*
                }
            }

            pub fn build_vec(&mut self, count: usize) -> Vec::<#source_struct_name> {
                std::iter::repeat_with(|| self.build()).take(count).collect()
            }
        }

        impl #source_struct_name {
            pub fn test_obj_builder() -> #inner_builder_name {
                #inner_builder_name::new()
            }
        }
    };

    TokenStream::from(output)
}
