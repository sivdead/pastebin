pub mod mermaid;
pub mod plugin;
pub mod prism;

use std::collections::HashMap;

pub fn new<'r>(plugins: Vec<Box<dyn plugin::Plugin<'r>>>) -> plugin::PluginManager<'r> {
    let base_static_resources = load_static_resources!(
    "/static/index.html" => "../static/index.html",
    "/static/custom.js" => "../static/custom.js",
    "/static/custom.css" => "../static/custom.css",
    "/static/favicon.ico" => "../static/favicon.ico",
    "/static/bootstrap.min.css" => "../static/bootstrap.min.css",
    "/static/all.min.css" => "../static/all.min.css",
    "/static/jquery.min.js" => "../static/jquery.min.js",
    "/static/crypto-js.min.js" => "../static/crypto-js.min.js",
    "/static/popper.min.js" => "../static/popper.min.js",
    "/static/bootstrap.min.js" => "../static/bootstrap.min.js",
    "/static/clipboard.min.js" => "../static/clipboard.min.js",
    "/static/bootstrap-notify.min.js" => "../static/bootstrap-notify.min.js"
    );

    let base_css_imports = vec![
        "/static/bootstrap.min.css",
        "/static/all.min.css",
        "/static/custom.css",
    ];

    let base_js_imports = vec![
        "/static/jquery.min.js",
        "/static/crypto-js.min.js",
        "/static/popper.min.js",
        "/static/bootstrap.min.js",
        "/static/clipboard.min.js",
        "/static/bootstrap-notify.min.js",
        "/static/custom.js",
    ];

    plugin::PluginManager::build()
        .plugins(plugins)
        .static_resources(base_static_resources)
        .css_imports(base_css_imports)
        .js_imports(base_js_imports)
        .finalize()
}
