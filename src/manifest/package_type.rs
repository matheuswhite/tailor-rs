#[derive(Default, Clone, Copy, PartialEq)]
pub enum PackageType {
    #[default]
    Binary,
    Library,
}
