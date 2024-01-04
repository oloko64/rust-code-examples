// When using a enum with variants that have conditional compilation attributes, you can use the
// `#[non_exhaustive]` attribute to prevent users from matching on the enum directly.
// Users cannot exhaustively match on the enum, they have to have a default case.
#[non_exhaustive]
enum MyFeaturesEnum {
    Feature1,

    #[cfg(feature = "feature2")]
    Feature2,

    #[cfg(feature = "feature3")]
    Feature3,
}

// On structs, the `#[non_exhaustive]` attribute prevents users from directly constructing the struct.
// Users can construct the struct using the `Default` trait or some method that you provide in the impl block.
// They also need to add the `..` syntax to the struct when destructuring it.
#[non_exhaustive]
struct MyStruct {
    val: bool,
}
