use crate::downgrade_fields::{derive_downgrade_fields, DowngradeStructParts};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Generics, Ident};

pub fn derive_downgrade_for_struct(
    ident: Ident,
    generics: Generics,
    data_struct: syn::DataStruct,
) -> TokenStream {
    let weak_ref = format_ident!("{}WeakRef", ident);

    let DowngradeStructParts {
        weak_fields,
        end_of_struct,
        destruct,
        downgrade,
        upgrade,
    } = derive_downgrade_fields(data_struct.fields);

    let derived = quote! {
        pub struct #weak_ref #generics #weak_fields #end_of_struct

        impl #generics glib::clone::Downgrade for #ident #generics {
            type Weak = #weak_ref #generics;

            fn downgrade(&self) -> Self::Weak {
                let Self #destruct = self;
                #weak_ref #downgrade
            }
        }

        impl #generics glib::clone::Upgrade for #weak_ref #generics {
            type Strong = #ident #generics;

            fn upgrade(&self) -> Option<Self::Strong> {
                let Self #destruct = self;
                Some(#ident #upgrade)
            }
        }
    };

    derived.into()
}
