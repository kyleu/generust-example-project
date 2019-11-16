use maud::{html, Markup};

use generust_example_project_core::Result;
use generust_example_project_service::{RequestContext, Router};

pub(crate) fn simple(ctx: &RequestContext, router: &dyn Router, title: &str, content: Markup) -> Result<Markup> {
  Ok(crate::components::page::page(ctx, router, title, html! {
    (crate::components::navbar::navbar(&ctx, router)?)

    div#content data-uk-height-viewport="expand: true" {
      @if let Some(f) = ctx.flash() {
        (crate::components::flash::flash(&f.0, &f.1)?)
      }
      (content)
    }
  })?)
}
