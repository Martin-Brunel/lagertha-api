use dotenv::dotenv;
use std::env;

use crate::{
    db::connect::DbPool, dto::{
        application::application_insertable::ApplicationInsertable,
        user::user_insertable::UserInsertable,
    }, repositories::{application::ApplicationRepository, user::UserRepository}, traits::application::ApplicationContract, utils::{cli::CLIUtils, password::PasswordUtils}
};

/// ### CommandInit
///
/// launch this command to init the Api
/// this command create an main application and a first user ROLE_SUPER_ADMIN
/// ```
/// let _ = CommandInit::exec(pool: DbPool).await;
/// ```
///
/// the user and application informations are display on the command line interface
pub struct CommandInit;

impl CommandInit {
    pub async fn exec(pool: DbPool) {
        dotenv().ok();
        let sysadmin_email = env::var("SYS_ADMIN_EMAIL").unwrap();
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::empty_line();
        let insertable =
            ApplicationInsertable::new(String::from("System"), sysadmin_email.clone(), true);
        let application_repository: ApplicationRepository = ApplicationContract::new(&pool);
        let system_app = application_repository.create_application(insertable);
        CLIUtils::write("Application system created");
        CLIUtils::empty_line();
        CLIUtils::separator();
        println!("application name:  {}", system_app.name);
        println!("application id:  {}", system_app.id);
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::empty_line();
        // generation du user system (super admin)
        let login = PasswordUtils::generate_password(32);
        let password = PasswordUtils::generate_password(32);
        let insertable = UserInsertable::new_command_line(
            sysadmin_email,
            "".to_string(),
            "".to_string(),
            system_app.id,
            &login,
            &password,
            true,
        );
        UserRepository::new(&pool).create_user(insertable);
        CLIUtils::write("system user (ROLE_SUPER_ADMIN) created");
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::write(&format!("login : {}", login));
        CLIUtils::write(&format!("password : {}", password));
        CLIUtils::empty_line();
        CLIUtils::separator();
        CLIUtils::empty_line();
    }
}
