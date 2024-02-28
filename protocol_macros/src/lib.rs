use proc_macro::TokenStream;
use proc_macro2::Ident;

use quote::quote;
use syn;
use syn::{Data, DeriveInput};

//extern crate proc_macro;

#[proc_macro_derive(EnumToString)]
pub fn enum_to_string_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    let output = impl_enum_to_string(&ast).unwrap_or_else(|e| e.to_compile_error());
    TokenStream::from(output)
    // let s = TokenStream::from(output).to_string();
    // TokenStream::from(create_err(&ast, s.as_str()).to_compile_error())
}

fn impl_enum_to_string(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let name = &ast.ident;

    let data = &ast.data;


    let data = match data {
        Data::Struct(data) => return Err(syn::Error::new(data.struct_token.span, "不支持struct")),
        Data::Enum(data) => data,
        Data::Union(data) => return Err(syn::Error::new(data.union_token.span, "不支持union")),
    };

    //return Err(create_err(ast, format!("{:?}", &data.variants.first().unwrap()).as_str()));


    let fields: Vec<_> = data.variants.iter().map(|v| &v.ident).collect();

    let gen = quote! {
        impl EnumToString for #name {
            fn enum_to_string(&self) -> &'static str {
                match self {
                    #(Self::#fields => stringify!(#fields)),*
                }
            }
        }
    };
    Ok(gen)
}


fn create_err(input: &syn::DeriveInput, message: &str) -> syn::Error {
    syn::Error::new_spanned(input, message.to_string())
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

fn get_fields_from_derive_input(input: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct {
                                 fields: syn::Fields::Named(
                                     syn::FieldsNamed { ref named, .. }
                                 ),
                                 ..
                             }) = input.data {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(input, "Must define on a Struct, not Enum".to_string()))
}
