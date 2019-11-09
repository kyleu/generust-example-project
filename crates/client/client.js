function on_event(t, k, v) {
  window.generust_example_project.client.on_event(t, k, v);
};

window.generust_example_project = {
  on_event: on_event
};
