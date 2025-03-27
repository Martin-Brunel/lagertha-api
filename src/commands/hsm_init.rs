use crate::services::hsm::HsmService;

/// ### CommandHsmInit
///
/// launch this command to init the Hsm part of Hb_cyber_core
/// this command init the hsm and prompt the user to register his SO pin
/// ```
/// let _ = CommandHsmInit::exec().await;
/// ```
///
/// the user and application informations are display on the command line interface
pub struct CommandHsmInit;

impl CommandHsmInit {
    pub async fn exec() {
        let _ = HsmService::new().init();
    }
}
