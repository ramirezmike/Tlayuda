use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct, Type};

#[proc_macro_derive(Tlayuda)]
pub fn entry_point(input: TokenStream) -> TokenStream {
    let item_struct = parse_macro_input!(input as ItemStruct);
    let source_struct_name = item_struct.ident.clone();
    let fields = get_fields(item_struct);

    let inner_builder_name = quote::format_ident!("Tlayuda{}Builder", source_struct_name);
    let OutputTokenPartials {
        field_declarations,
        field_builder_intializers,
        field_setter_functions,
    } = generate_output_tokens(&fields);
    let fields = fields.iter().map(|FieldInfo { identifier: x, .. }| x);

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
            pub fn tlayuda() -> #inner_builder_name {
                #inner_builder_name::new()
            }
        }
    };

    TokenStream::from(output)
}

struct FieldInfo {
    identifier: proc_macro2::Ident,
    field_type: syn::Type,
}

fn get_fields(item_struct: ItemStruct) -> Vec<FieldInfo> {
    item_struct
        .fields
        .iter()
        .filter(|x| x.ident.is_some())
        .map(|x| FieldInfo {
            identifier: x.ident.as_ref().unwrap().clone(),
            field_type: x.ty.clone(),
        })
        .collect()
}

struct OutputTokenPartials {
    field_setter_functions: Vec<proc_macro2::TokenStream>,
    field_builder_intializers: Vec<proc_macro2::TokenStream>,
    field_declarations: Vec<proc_macro2::TokenStream>,
}

fn generate_output_tokens(fields: &Vec<FieldInfo>) -> OutputTokenPartials {
    let field_setter_functions = fields
        .iter()
        .map(|field| {
            let func_name = quote::format_ident!("set_{}", field.identifier);
            let identifier = &field.identifier;
            let field_type = &field.field_type;

            quote! {
                pub fn #func_name<F: 'static>(mut self, f: F) -> Self where
                    F: Fn(usize) -> #field_type {
                        self.#identifier = Box::new(f);
                        self
                }
            }
        })
        .collect();

    let field_builder_intializers = fields
        .iter()
        .map(|field| {
            let identity = match field.field_type.clone() {
                Type::Path(type_path) => match type_path.path.get_ident() {
                    Some(ident) => (ident.clone(), ident.into_token_stream()),
                    None => (
                        type_path.path.segments.last().unwrap().ident.clone(),
                        type_path.into_token_stream(),
                    ),
                },
                _ => todo!("Type {:?} not supported", field.field_type),
            };

            let identifier = &field.identifier;
            let identity_tokens = identity.1;
            let f = match identity.0.to_string().as_str() {
                "String" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                "OsString" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                "char" => quote! { |i| std::char::from_digit(i as u32, 10).unwrap_or('a') },
                "bool" => quote! { |i| false },
                "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "i64" | "i128" | "isize" | "u64"
                | "u128" | "usize" | "f32" | "f64" => quote! { |i| i as #identity_tokens },
                _ => {
                    // attempt to call a builder that may be on this type
                    // this will end up causing a compile error if the type doesn't have
                    // the #[derive(Tlayuda)] macro.
                    // TODO: Need to figure out a way to communicate this better in the compiler
                    quote! { |i| #identity_tokens::tlayuda().with_index(i).build() }
                }
            };

            quote! { #identifier: Box::new(#f), }
        })
        .collect();

    let field_declarations = fields
        .iter()
        .map(
            |FieldInfo {
                 identifier: x,
                 field_type: t,
             }| {
                quote! {
                    #x: Box<dyn FnMut(usize) -> #t>,
                }
            },
        )
        .collect();

    OutputTokenPartials {
        field_declarations,
        field_builder_intializers,
        field_setter_functions,
    }
}
