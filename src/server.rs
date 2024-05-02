use leptos_spin::{render_best_match_to_stream, RouteTable, server_fn::register_explicit};
use spin_sdk::http::{IncomingRequest, ResponseOutparam};
use spin_sdk::http_component;

#[http_component]
async fn handle_a_nvlkv_xyz(req: IncomingRequest, resp_out: ResponseOutparam) {
    let mut conf = leptos::get_configuration(None).await.unwrap();
    conf.leptos_options.output_name = "a_nvlkv_xyz".to_owned();

    // Register server functions
    register_explicit::<crate::app::SaveCount>();

    let app = crate::app::App;

    let mut routes = RouteTable::build(app);
    routes.add_server_fn_prefix("/api").unwrap();

    render_best_match_to_stream(req, resp_out, &routes, app, &conf.leptos_options).await
}
