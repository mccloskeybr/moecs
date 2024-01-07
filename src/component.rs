use std::any::Any;

pub trait Anyable: 'static + Any {
    fn as_ref_any(&mut self) -> & dyn Any;
    fn as_mut_any(&mut self) -> &mut dyn Any;
}

impl <T: 'static> Anyable for T {
    fn as_ref_any(&mut self) -> & dyn Any {
        self
    }
    fn as_mut_any(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait Component: Anyable {}
