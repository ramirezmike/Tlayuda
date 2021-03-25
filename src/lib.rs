use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, ItemStruct, Meta, Type};

#[proc_macro_derive(Tlayuda, attributes(tlayuda_ignore))]
pub fn entry_point(input: TokenStream) -> TokenStream {
    let source_struct = parse_macro_input!(input as ItemStruct);
    let source_struct_name = source_struct.ident.clone();
    let fields = get_fields(source_struct);
    let inner_builder_name = quote::format_ident!("Tlayuda{}Builder", source_struct_name);

    let OutputTokenPartials {
        field_declarations,
        field_builder_intializers,
        field_setter_functions,
    } = generate_output_tokens(&fields);

    let builder_parameters = fields
        .iter()
        .filter(|f| f.is_ignored)
        .map(|f| {
            let identifier = &f.identifier;
            let field_type = &f.field_type;

            quote! { #identifier: #field_type }
        })
        .collect::<Vec<_>>();

    let (ignored_fields, fields): (Vec<_>, Vec<_>) = fields.iter().partition(|f| f.is_ignored);

    // Ignored fields will be manually populated by the user with a clonable
    // instance passed into the initial .tlayuda() call. The following
    // is intended to create those parameters.
    let inner_builder_constructor_parameters = ignored_fields.iter()
                                                             .map(|f| {
                                                                 let i = &f.identifier;
                                                                 quote! { #i }
                                                             });
    let ignored_fields = ignored_fields.iter()
                                       .map(|f| {
                                           let i = &f.identifier;
                                           quote! { #i: self.#i.clone(), }
                                       });

    let fields = fields.iter().map(|f| f.identifier.clone());

    let output = quote! {
        pub struct #inner_builder_name {
            index: usize,
            #(#field_declarations),*
        }

        impl #inner_builder_name {
            pub fn new(#(#builder_parameters),*) -> #inner_builder_name {
                #inner_builder_name {
                    index: 0,
                    #(#field_builder_intializers),*
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
                    #(#ignored_fields)*
                    #(#fields: self.#fields.as_mut()(i)),*
                }
            }

            pub fn build_vec(&mut self, count: usize) -> Vec::<#source_struct_name> {
                std::iter::repeat_with(|| self.build()).take(count).collect()
            }
        }

        impl #source_struct_name {
            pub fn tlayuda(#(#builder_parameters),*) -> #inner_builder_name {
                #inner_builder_name::new(#(#inner_builder_constructor_parameters),* )
            }
        }
    };

    TokenStream::from(output)
}

#[derive(Debug)]
struct FieldInfo {
    identifier: proc_macro2::Ident,
    field_type: syn::Type,
    is_ignored: bool,
}

fn get_fields(item_struct: ItemStruct) -> Vec<FieldInfo> {
    item_struct
        .fields
        .iter()
        .filter(|x| x.ident.is_some())
        .map(|x| FieldInfo {
            identifier: x.ident.as_ref().unwrap().clone(),
            field_type: x.ty.clone(),
            is_ignored: x.attrs.iter().any(|attribute| {
                if let Ok(meta) = attribute.parse_meta() {
                    match meta {
                        Meta::Path(path) => path.is_ident("tlayuda_ignore".into()),
                        _ => false,
                    }
                } else {
                    false
                }
            }),
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
        .filter(|f| !f.is_ignored)
        .map(|field| {
            let set_func_name = quote::format_ident!("set_{}", field.identifier);
            let identifier = &field.identifier;
            let field_type = &field.field_type;

            quote! {
                pub fn #set_func_name<F: 'static>(mut self, f: F) -> Self where
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
            let identifier = &field.identifier;

            if field.is_ignored {
                quote! { #identifier: #identifier }
            } else {
                let field_type = parse_field_type(&field.field_type);

                let f =
                match field_type {
                    FieldType::Basic(field_type, full_field_type) => {
                        match field_type.to_string().as_str() {
                            "String" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                            "OsString" => quote! { |i| format!("{}{}", stringify!(#identifier), i).into() },
                            "char" => quote! { |i| std::char::from_digit(i as u32, 10).unwrap_or('a') },
                            "bool" => quote! { |i| false },
                            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "i64" | "i128" | "isize"
                            | "u64" | "u128" | "usize" | "f32" | "f64" => {
                                quote! { |i| i as #full_field_type }
                            },
                            "Vec" => quote! { |i| Vec::new() },
                            _ => {
                                // attempt to call a builder that may be on this type
                                // this will end up causing a compile error if the type doesn't have
                                // the #[derive(Tlayuda)] macro.
                                // TODO: Need to figure out a way to communicate this better in the compiler
                                quote! { |i| #full_field_type::tlayuda().with_index(i).build() }
                            }
                        }
                    },
                    FieldType::Array(field_type, full_field_type, length) => {
                        match field_type.to_string().as_str() {
                            "i8" | "i16" | "i32" | "u8" | "u16" | "u32" | "i64" | "i128" | "isize"
                            | "u64" | "u128" | "usize" | "f32" | "f64" => {
                                quote! { |i| [i as #full_field_type; #length] }
                            },
                            _ => panic!("Type {:?} not yet supported for arrays", field_type)
                        }
                    }
                };

                quote! { #identifier: Box::new(#f) }
            }
        })
        .collect();

    let field_declarations = fields
        .iter()
        .map(
            |FieldInfo {
                 identifier,
                 field_type,
                 is_ignored,
             }| {
                if *is_ignored {
                    quote! { #identifier: #field_type }
                } else {
                    quote! {
                        #identifier: Box<dyn FnMut(usize) -> #field_type>
                    }
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

enum FieldType {
    Basic(syn::Ident, proc_macro2::TokenStream),
    Array(syn::Ident, proc_macro2::TokenStream, usize),
}

fn parse_field_type(field_type: &syn::Type) -> FieldType {
    match field_type {
        Type::Path(type_path) => match type_path.path.get_ident() {
            Some(ident) => FieldType::Basic(ident.clone(), ident.into_token_stream()),
            None => (
                FieldType::Basic(type_path.path.segments.last().unwrap().ident.clone(),
                                 type_path.into_token_stream())
            ),
        },
        Type::Array(type_array) => {
            match parse_field_type(&type_array.elem) {
                FieldType::Basic(i, ts) => {
                    match &type_array.len {
                        syn::Expr::Lit(expr) => {
                            match &expr.lit {
                                syn::Lit::Int(number) => {
                                    match number.base10_parse::<usize>() {
                                        Ok(parsed_number) => FieldType::Array(i, ts, parsed_number),
                                        _ => panic!("Number literal in array was invalid: {:?}", number)
                                    }
                                },
                                _ => todo!("Array length literal {:?} not yet supported", expr)
                            }
                            
                        },
                        _ => todo!("Array length expression {:?} not yet supported", type_array.len)
                    }
                },
                _ => todo!("Nested arrays not yet supported")
            }
        },
        _ => todo!("Type {:?} not supported", field_type),
    }
}
