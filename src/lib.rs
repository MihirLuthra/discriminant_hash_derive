use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Error};

/// Deriving `DiscriminantHash` implements [`Hash`] trait
/// for the underlying enum. Here hash is only dependent on discriminant
/// and isn't effected by variant's fields.
///
/// [`Hash`]: std::hash::Hash
///
/// # Example
///
/// ```
/// use discriminant_hash_derive::DiscriminantHash;
/// use std::{
///     collections::hash_map::DefaultHasher,
///     hash::{Hash, Hasher},
/// };
///
/// #[derive(DiscriminantHash)]
/// enum Abc<T> {
///     Simple,
///     HashNotImplemented(Xyz),
///     Generic(T),
/// }
///
/// #[allow(unused)]
/// #[derive(Hash)]
/// enum Pqr<'a> {
///     Simple,
///     Lifetime(&'a str),
/// }
///
/// // Xyz doesn't impl Hash
/// struct Xyz;
///
/// fn main() {
///     assert_eq!(my_hash(Abc::Simple::<i32>), my_hash(Abc::Simple::<Xyz>));
///     assert_eq!(
///         my_hash(Abc::HashNotImplemented::<i32>(Xyz)),
///         my_hash(Abc::HashNotImplemented::<String>(Xyz))
///     );
///     assert_eq!(
///         my_hash(Abc::Generic::<i32>(4)),
///         my_hash(Abc::Generic::<Xyz>(Xyz))
///     );
///
///     assert_ne!(
///         my_hash(Abc::Simple::<i32>),
///         my_hash(Abc::Generic::<Xyz>(Xyz))
///     );
///
///     
///     // This may be same depending on how Pqr is defined
///     // assert_eq!(
///     //     my_hash(Abc::Simple::<i32>),
///     //     my_hash(Pqr::Simple)
///     // );
///     
/// }
///
/// fn my_hash<T>(obj: T) -> u64
/// where
///     T: Hash,
/// {
///     let mut hasher = DefaultHasher::new();
///     obj.hash(&mut hasher);
///     hasher.finish()
/// }
/// ```
#[proc_macro_derive(DiscriminantHash)]
pub fn discriminant_hash_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    impl_hash(&derive_input).into()
}

fn impl_hash(derive_input: &DeriveInput) -> TokenStream2 {
    // TODO attribute for having specific variant's hash include its value
    let _data_enum = if let Data::Enum(ref data_enum) = derive_input.data {
        data_enum
    } else {
        return Error::new(
            Span::call_site(),
            "#[derive(DiscriminantHash)] only works with enum",
        )
        .to_compile_error();
    };

    let enum_name = &derive_input.ident;
    let (impl_generics, ty_generics, where_clause) = derive_input.generics.split_for_impl();

    quote! {
        impl #impl_generics std::hash::Hash for #enum_name #ty_generics #where_clause {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(
                    &std::mem::discriminant(self),
                    state
                )
            }
        }
    }
}
