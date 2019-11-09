use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use generust_example_project_core::Error;
use generust_example_project_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, &req, |ctx| {
    let k: &str = &key;
    match k {
      "dump" => generust_example_project_templates::testbed::dump(&ctx),
      "gallery" => generust_example_project_templates::testbed::gallery(&ctx),
      "prototype" => generust_example_project_templates::testbed::prototype(&ctx),
      "scroll" => generust_example_project_templates::testbed::scroll(&ctx),
      _ => Err(Error::from(format!("Cannot find testbed matching [{}]", key)))
    }
  })
}
