use actix_session::Session;
use actix_web::{web, HttpRequest, HttpResponse};

use generust_example_project_service::AppConfig;

/// Available at `/testbed/{key}`
pub fn testbed_key(session: Session, cfg: web::Data<AppConfig>, key: web::Path<String>, req: HttpRequest) -> HttpResponse {
  crate::act(&session, &cfg, req, |ctx, router| {
    let k: &str = &key;
    match k {
      "dump" => generust_example_project_templates::testbed::dump(&ctx, router),
      "gallery" => generust_example_project_templates::testbed::gallery(&ctx, router),
      "prototype" => generust_example_project_templates::testbed::prototype(&ctx, router),
      "scroll" => generust_example_project_templates::testbed::scroll(&ctx, router),
      _ => Err(anyhow::anyhow!(format!("Cannot find testbed matching [{}]", key)))
    }
  })
}
