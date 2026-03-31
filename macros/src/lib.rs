use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Fields, Type};

#[proc_macro_attribute]
pub fn auto_skip_none(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as ItemStruct);

    let fields = match &mut input.fields {
        Fields::Named(fields) => &mut fields.named,
        _ => {
            return syn::Error::new_spanned(
                input,
                "auto_skip_none поддерживает только struct с именованными полями",
            )
            .to_compile_error()
            .into();
        }
    };

    for field in fields.iter_mut() {
        if is_option(&field.ty) {
            field.attrs.push(syn::parse_quote!(
                #[serde(skip_serializing_if = "Option::is_none")]
            ));
        }
    }

    quote!(#input).into()
}

fn is_option(ty: &Type) -> bool {
    match ty {
        Type::Path(tp) => tp
            .path
            .segments
            .last()
            .map(|s| s.ident == "Option")
            .unwrap_or(false),
        _ => false,
    }
}
