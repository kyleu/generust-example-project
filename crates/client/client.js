function on_event(t, k, v) {
  window.generust_example_project.client.on_event(t, k, v);
};

function notify(level, content) {
  UIkit.notification(content, { status: level });
};

window.generust_example_project = {
  on_event: on_event,
  notify: notify
};
