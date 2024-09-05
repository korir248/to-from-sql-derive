extern crate proc_macro;

use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::{Comma, Eq};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, LitByteStr, Variant};

#[proc_macro_derive(FromSqlDerive, attributes(diesel))]
pub fn from_sql_derive(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, attrs, data, ..
    } = parse_macro_input!(input as DeriveInput);

    let (sql_type, variants) = parse_sql_type(attrs, data);

    let from_sql_arms = variants.iter().map(|Variant { ident, fields, .. }| {
        if let Fields::Unit = fields {
            let ident_str = format!("{}", ident);
            let ident_bytes = ident_str.as_bytes();
            let ident_bytes = LitByteStr::new(ident_bytes, Span::call_site());
            quote! { #ident_bytes => Ok(Self::#ident), }
        } else {
            panic!("FromSqlDerive can only be derived for enums with unit variants");
        }
    });

    let gen = quote! {
        impl diesel::deserialize::FromSql<#sql_type, diesel::pg::Pg> for #ident {
            fn from_sql(bytes: diesel::pg::PgValue<'_>) -> diesel::deserialize::Result<Self> {
                match bytes.as_bytes() {
                    #(#from_sql_arms)*
                    _ => Err("Unrecognized enum variant".into()),
                }
            }
        }
    };

    gen.into()
}

fn parse_sql_type(attrs: Vec<Attribute>, data: Data) -> (Ident, Punctuated<Variant, Comma>) {
    let sql_type: Vec<Ident> = attrs
        .iter()
        .filter(|a| a.path().is_ident("diesel"))
        .flat_map(|a| {
            let mut p = Vec::new();
            let parser = a
                .parse_args_with(Punctuated::<syn::Ident, Eq>::parse_separated_nonempty)
                .unwrap();
            let ident = parser.first().cloned().map(|f| f.to_string());
            let value = parser.last().cloned();
            if let Some("sql_type") = ident.as_deref() {
                if let Some(value) = value {
                    p.push(value)
                }
            }
            p
        })
        .collect();

    let sql_type = match sql_type.last() {
        Some(idnt) => idnt,
        None => panic!("Expected attribute diesel(sql_type=\"YourSqlType\")"),
    };

    let variants = if let Data::Enum(data) = data {
        data.variants
    } else {
        panic!("FromSqlDerive can only be derived for enums");
    };

    (sql_type.to_owned(), variants)
}
