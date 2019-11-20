use maud::{html, Markup};

use generust_example_project_core::Result;
use generust_example_project_service::{RequestContext, Router};

pub fn list(ctx: &RequestContext, router: &dyn Router) -> Result<Markup> {
  let content = crate::components::card::card(
    ctx,
    html!(
      h3 { "Project Administration" }
      ul {
        li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.connections")?) { "Connection List" } }
        li { a.(ctx.user_profile().link_class()) href=(router.route_simple("admin.settings")?) { "Edit Settings" } }
      }
    )
  );
  crate::section(ctx, router, "Project Administration", content)
}
