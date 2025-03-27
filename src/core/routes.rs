use crate::controlers::{
    anonymous_sentinel, application, auth, cluster, oauth, oidc, sentinel, system, user,
};
use rocket::Route;
use rocket_okapi::openapi_get_routes;
pub struct CoreRoutes;

impl CoreRoutes {
    pub fn get() -> Vec<Route> {
        let routes = openapi_get_routes![
            // auth controller
            auth::login,
            auth::refresh,
            // oauth controller
            oauth::authorize,
            oauth::token,
            // oidc controller
            oidc::verify,
            // application controller
            application::post_application,
            application::update_application,
            application::delete_application,
            // user controller
            user::post_user_public,
            user::validate_user,
            user::post_user,
            user::forget_user_password,
            user::reset_user_password,
            user::update_user_password,
            user::delete_user,
            user::get_2fa_code,
            user::activate_2fa,
            // cluster controller
            cluster::create,
            cluster::add_memberships,
            cluster::remove_memberships,
            cluster::add_sentinels,
            cluster::remove_sentinels,
            cluster::add_anonymous_sentinels,
            cluster::remove_anonymous_sentinels,
            cluster::delete_cluster,
            cluster::get_cluster_users,
            // sentinel controller
            sentinel::create,
            sentinel::get_by_id,
            sentinel::delete_by_id,
            // anonymous sentinel controller
            anonymous_sentinel::create,
            anonymous_sentinel::create_public,
            anonymous_sentinel::get_public_by_id,
            anonymous_sentinel::get_by_id,
            anonymous_sentinel::delete_by_id,
            // system controller
            system::get_version,
            system::get_system_informations
        ];
        routes
    }
}
