use crate::components;

use anyhow::Result;
use generust_example_project_service::{RequestContext, Router};
use maud::{html, Markup, PreEscaped};

pub(crate) fn page(ctx: &RequestContext, router: &dyn Router, title: &str, content: Markup) -> Result<Markup> {
  Ok(html! {
    (PreEscaped("<!DOCTYPE html>"))
    html lang="en" {
      (components::header::header(&ctx, router, &format!("{} - {}", title, generust_example_project_core::APPNAME))?)
      body.(ctx.user_profile().theme().body_class()) {
        (content)
      }
    }
  })
}
