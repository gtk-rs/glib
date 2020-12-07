use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Fields, FieldsNamed, FieldsUnnamed, Ident, Type};

pub struct DowngradeStructParts {
    pub weak_fields: TokenStream,
    pub end_of_struct: TokenStream,
    pub destruct: TokenStream,
    pub downgrade: TokenStream,
    pub upgrade: TokenStream,
}

pub fn derive_downgrade_fields(fields: syn::Fields) -> DowngradeStructParts {
    match fields {
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let fields: Vec<Type> = unnamed
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| field.ty)
                .collect();

            let weak_fields: Vec<_> = fields
                .iter()
                .map(|ty| {
                    quote! {
                        <#ty as glib::clone::Downgrade>::Weak
                    }
                })
                .collect();

            let field_ident: Vec<Ident> =
                (0..fields.len()).map(|i| format_ident!("_{}", i)).collect();

            DowngradeStructParts {
                weak_fields: quote! {
                    (#(
                        #weak_fields
                    ),*)
                },
                end_of_struct: quote!(;),
                destruct: quote! {
                    (#(
                        ref #field_ident
                    ),*)
                },
                downgrade: quote! {
                    (#(
                        glib::clone::Downgrade::downgrade(#field_ident)
                    ),*)
                },
                upgrade: quote! {
                    (#(
                        glib::clone::Upgrade::upgrade(#field_ident)?
                    ),*)
                },
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let fields: Vec<(Ident, Type)> = named
                .into_pairs()
                .map(|pair| pair.into_value())
                .map(|field| (field.ident.expect("Field ident is specified"), field.ty))
                .collect();

            let weak_fields: Vec<_> = fields
                .iter()
                .map(|(ident, ty)| {
                    quote! {
                        #ident: <#ty as glib::clone::Downgrade>::Weak
                    }
                })
                .collect();

            let field_ident: Vec<_> = fields.iter().map(|(ident, _ty)| ident).collect();

            DowngradeStructParts {
                weak_fields: quote! {
                    {#(
                        #weak_fields
                    ),*}
                },
                end_of_struct: quote!(),
                destruct: quote! {
                    {#(
                        ref #field_ident
                    ),*}
                },
                downgrade: quote! {
                    {#(
                        #field_ident: glib::clone::Downgrade::downgrade(#field_ident)
                    ),*}
                },
                upgrade: quote! {
                    {#(
                        #field_ident: glib::clone::Upgrade::upgrade(#field_ident)?
                    ),*}
                },
            }
        }
        Fields::Unit => DowngradeStructParts {
            weak_fields: quote! {},
            end_of_struct: quote! { ; },
            destruct: quote! {},
            downgrade: quote! {},
            upgrade: quote! {},
        },
    }
}
