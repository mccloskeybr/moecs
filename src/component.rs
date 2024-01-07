use std::any::Any;

pub trait Anyable: 'static {
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl <T: 'static> Anyable for T {
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: Anyable {}
