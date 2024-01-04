// When using a enum with variants that have conditional compilation attributes, you can use the
// `#[non_exhaustive]` attribute to prevent users from matching on the enum directly.
#[non_exhaustive]
enum MyFeaturesEnum {
    Feature1,

    #[cfg(feature = "feature2")]
    Feature2,

    #[cfg(feature = "feature3")]
    Feature3,
}
