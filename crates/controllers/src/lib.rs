#![warn(anonymous_parameters)]
#![warn(bare_trait_objects)]
#![warn(elided_lifetimes_in_paths)]
#![warn(missing_debug_implementations)]
#![warn(single_use_lifetimes)]
#![warn(trivial_casts)]
#![warn(unreachable_pub)]
#![warn(unsafe_code)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]
#![warn(variant_size_differences)]
#![doc(
  html_favicon_url = "https://raw.githubusercontent.com/generust-example-project/generust-example-project/master/crates/assets/embed/favicon.ico"
)]
#![doc(
  html_logo_url = "https://raw.githubusercontent.com/generust-example-project/generust-example-project/master/crates/assets/embed/favicon.png"
)]
#![doc(issue_tracker_base_url = "https://github.com/generust-example-project/generust-example-project/issues/")]

//! `generust-example-project-controllers` contains actix-web HTTP controllers, usually calling methods from [generust-example-project-service](generust_example_project_service).

pub mod routes;
pub mod util {
  pub mod actions;
  pub mod ctx;
  pub mod router;
}
pub mod forms {
  pub mod profile_form;
}
pub mod home;
pub mod static_file;
pub mod testbed;
pub mod websocket;

pub(crate) use crate::util::actions::act;
pub(crate) use crate::util::actions::not_found;
pub(crate) use crate::util::actions::redir;
pub(crate) use crate::util::ctx::req_context;
