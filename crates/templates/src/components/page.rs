use maud::{html, Markup, PreEscaped};

use crate::components;
use generust_example_project_core::Result;
use generust_example_project_service::RequestContext;

pub(crate) fn page(ctx: &RequestContext, title: &str, content: Markup) -> Result<Markup> {
  Ok(html! {
    (PreEscaped("<!DOCTYPE html>"))
    html lang="en" {
      (components::header::header(&ctx, &format!("{} - {}", title, generust_example_project_core::APPNAME))?)
      body.(ctx.user_profile().theme().body_class()) {
        (content)
      }
    }
  })
}
