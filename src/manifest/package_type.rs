#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum PackageType {
    #[default]
    Binary,
    Library,
}
