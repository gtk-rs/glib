use crate::downgrade_fields::{derive_downgrade_fields, DowngradeStructParts};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

pub fn derive_downgrade_for_enum(ident: Ident, data_enum: syn::DataEnum) -> TokenStream {
    let weak_ref = format_ident!("{}WeakRef", ident);

    let variants: Vec<(Ident, DowngradeStructParts)> = data_enum
        .variants
        .into_iter()
        .map(|variant| (variant.ident, derive_downgrade_fields(variant.fields)))
        .collect();

    let weak_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let weak_fields = &parts.weak_fields;
            quote! {
                #ident #weak_fields
            }
        })
        .collect();

    let downgrade_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let destruct = &parts.destruct;
            let downgrade = &parts.downgrade;
            quote! {
                Self::#ident #destruct => Self::Weak::#ident #downgrade
            }
        })
        .collect();

    let upgrade_variants: Vec<_> = variants
        .iter()
        .map(|(ident, parts)| {
            let destruct = &parts.destruct;
            let upgrade = &parts.upgrade;
            quote! {
                Self::#ident #destruct => Self::Strong::#ident #upgrade
            }
        })
        .collect();

    let derived = quote! {
        pub enum #weak_ref {#(
            #weak_variants
        ),*}

        impl glib::clone::Downgrade for #ident {
            type Weak = #weak_ref;

            fn downgrade(&self) -> Self::Weak {
                match self {#(
                    #downgrade_variants
                ),*}
            }
        }

        impl glib::clone::Upgrade for #weak_ref {
            type Strong = #ident;

            fn upgrade(&self) -> Option<Self::Strong> {
                Some(match self {#(
                    #upgrade_variants
                ),*})
            }
        }
    };

    derived.into()
}
