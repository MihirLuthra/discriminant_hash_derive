# discriminant_hash_derive

Deriving `DiscriminantHash` implements [Hash](https://doc.rust-lang.org/std/hash/trait.Hash.html) trait for the underlying enum.
Here hash is only dependent on discriminant and isn’t effected by variant’s fields.

# Example

```rust
use discriminant_hash_derive::DiscriminantHash;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

#[derive(DiscriminantHash)]
enum Abc<T> {
    Simple,
    HashNotImplemented(Xyz),
    Generic(T),
}

#[allow(unused)]
#[derive(Hash)]
enum Pqr<'a> {
    Simple,
    Lifetime(&'a str),
}

// Xyz doesn't impl Hash
struct Xyz;

fn main() {
    assert_eq!(my_hash(Abc::Simple::<i32>), my_hash(Abc::Simple::<Xyz>));
    assert_eq!(
        my_hash(Abc::HashNotImplemented::<i32>(Xyz)),
        my_hash(Abc::HashNotImplemented::<String>(Xyz))
    );
    assert_eq!(
        my_hash(Abc::Generic::<i32>(4)),
        my_hash(Abc::Generic::<Xyz>(Xyz))
    );

    assert_ne!(
        my_hash(Abc::Simple::<i32>),
        my_hash(Abc::Generic::<Xyz>(Xyz))
    );

     
    // This may be same depending on how Pqr is defined
    // assert_eq!(
    //     my_hash(Abc::Simple::<i32>),
    //     my_hash(Pqr::Simple)
    // );
     
}

fn my_hash<T>(obj: T) -> u64
where
    T: Hash,
{
    let mut hasher = DefaultHasher::new();
    obj.hash(&mut hasher);
    hasher.finish()
}
```
