use generust_example_project_service::RequestContext;
use maud::{html, Markup};

pub(crate) fn card(ctx: &RequestContext, content: &Markup) -> Markup {
  html! {
    div.uk-margin.uk-card.uk-card-body.(ctx.user_profile().theme().card_class()) {
      (content)
    }
  }
}
