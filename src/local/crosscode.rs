use model::CrossCodeInstallation;

trait InstallationFinder {
    fn find() -> Result<Option<CrossCodeInstallation>, String>;
}

struct SteamInstallationFinder {}
impl InstallationFinder for SteamInstallationFinder {
    fn find() -> Result<Option<CrossCodeInstallation>, String> {
        // TODO
        Err("Not yet implemented".to_string())
    }
}

impl CrossCodeInstallation {
    pub fn find() -> Result<CrossCodeInstallation, String> {
        // TODO
        Err("Not yet implemented".to_string())
    }
}