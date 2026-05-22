use super::handlers::{
    get_users,
    create_user,
    login_user,
    dashboard,
    update_user,
    delete_user,
    logout_user,
    profile,
    index,
    // me
};

pub fn app_config(cfg: &mut actix_web::web::ServiceConfig) {
    cfg
    .service(index)
    // .service(me)
    .service(get_users)
    .service(create_user)
    .service(login_user)
    .service(dashboard)
    .service(update_user)
    .service(delete_user)
    .service(logout_user)
    .service(profile);
}