pub mod window;
pub mod api;

pub struct API<T> {
    context: T
}

impl<T> API<T> {
    pub fn get_api(&self) -> &T {
        &self.context
    }
}

pub struct Version(pub u32, pub u32, pub u32);

pub struct APIDescription {
    pub version: Version,
}
