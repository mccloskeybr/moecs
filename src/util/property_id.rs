pub trait PropertyId {
    fn property_id() -> u64
        where Self: Sized;

    fn self_property_id(&self) -> u64;
}
