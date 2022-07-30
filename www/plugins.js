function perform_demo() {
  alert("something");
}

register_plugin = function (importObject) {
  importObject.env.perform_demo = perform_demo;
};

miniquad_add_plugin({ register_plugin, on_init });
