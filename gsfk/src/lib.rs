pub mod window;
pub mod api;

pub struct API<T> {
    context: T
}

pub struct Version(pub u32, pub u32, pub u32);

pub struct APIDescription {
    pub version: Version,
}
