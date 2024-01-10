use crate::component::Component;

#[derive(Default)]
pub struct Query {
    components: Vec<u64>,
}

impl Query {
    pub fn with<T: 'static + Component>(&mut self) -> &mut Query {
        self.components.push(T::property_id());
        self
    }

    pub fn get_components(&self) -> &Vec<u64> {
        &self.components
    }
}
