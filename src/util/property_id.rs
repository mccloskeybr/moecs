/// `PropertyId` is essentially a stand-in for `TypeId` that's usable on non-`static` traits.
/// Useful when a given implementation does not have a `static` lifetime (e.g. `SystemParam`s).
pub trait PropertyId {
    fn property_id() -> u64
        where Self: Sized;

    fn self_property_id(&self) -> u64;
}
