use maud::{html, Markup};

use generust_example_project_core::Result;

pub(crate) fn flash(k: &str, v: &str) -> Result<Markup> {
  let cls = match k {
    "status" => "uk-alert-primary",
    "success" => "uk-alert-success",
    "warning" => "uk-alert-warning",
    "error" => "uk-alert-danger",
    _ => ""
  };

  Ok(html! {
    div.alert-top.(cls) data-uk-alert? {
      a.uk-alert-close href="#" data-uk-close="" {}
      p { (v) }
    }
  })
}
