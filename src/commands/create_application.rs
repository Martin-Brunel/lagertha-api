use crate::{db::connect::DbPool, dto::application::application_input::ApplicationInput, repositories::application::ApplicationRepository, services::{application::ApplicationService, hsm::HsmService}, traits::application::ApplicationContract, utils::cli::CLIUtils};
use dotenv::dotenv;
use std::env;

/// ### CommandCreateApplication
///
/// launch this command to create a new application ressource
/// this command init the new applications
/// ```
/// let _ = CommandCreateApplication::exec(pool: DbPool).await;
/// ```
///
pub struct CommandCreateApplication;

impl CommandCreateApplication {
    pub async fn exec(pool: DbPool, app_name: &str, app_email: &str) {
        dotenv().ok();
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::empty_line();
        let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
        let application_service = ApplicationService::new(application_repository);
        let new_app = application_service.create_application(ApplicationInput {
            name: app_name.to_string(),
            contact_email: app_email.to_string()
        });
        CLIUtils::write("Application system created");
        CLIUtils::empty_line();
        CLIUtils::separator();
        println!("application name:  {}", new_app.name);
        println!("application id:  {}", new_app.id);
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::empty_line();
    }
}
