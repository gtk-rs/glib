fn main() {
    manage_docs();
}

#[cfg(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs"))]
fn manage_docs() {
    extern crate lgpl_docs;
    const PATH: &str = "src";
    const IGNORES: &[&str] = &[
        "lib.rs",
        "prelude.rs",
        "signal.rs",
        "boxed.rs",
        "byte_array.rs",
        "bytes.rs",
        "char.rs",
        "clone.rs",
        "enums.rs",
        "error.rs",
        "gobject/mod.rs",
        "log.rs",
        "main_context.rs",
        "main_context_channel.rs",
        "main_context_futures.rs",
        "object.rs",
        "send_unique.rs",
        "shared.rs",
        "source.rs",
        "source_futures.rs",
        "string.rs",
        "subclass/boxed.rs",
        "subclass/interface.rs",
        "subclass/mod.rs",
        "subclass/object.rs",
        "subclass/simple.rs",
        "subclass/types.rs",
        "translate.rs",
        "types.rs",
        "utils.rs",
        "value.rs",
        "variant.rs",
        "variant_dict.rs",
        "variant_type.rs",
        "wrapper.rs",
    ];
    lgpl_docs::purge(PATH, IGNORES);
    if cfg!(feature = "embed-lgpl-docs") {
        lgpl_docs::embed(lgpl_docs::Library::Glib, PATH, IGNORES);
    }
}

#[cfg(not(any(feature = "embed-lgpl-docs", feature = "purge-lgpl-docs")))]
fn manage_docs() {}
