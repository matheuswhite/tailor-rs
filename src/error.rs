#[derive(PartialEq, PartialOrd, Debug)]
pub enum TailorErr {
    PreperingDirFail,
    CreateDirsFail,
    DownloadFilesFail,
    RustFileGenerationFail,
    CFileGenerationFail,
    GitInitError,
    CreateHatTomlFail,
}
