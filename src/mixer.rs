use crate::{address, api::Api};

pub struct Mixer {
    house_addr: address::House,
}

impl Mixer {
    pub fn new(api: &Api) -> Self {
        Self {
            house_addr: address::Unused::new(&api).into(),
        }
    }
}
