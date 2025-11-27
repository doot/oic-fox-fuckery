use std::time::Duration;

use async_trait::async_trait;
use axum::{
    Extension,
    http::{HeaderMap, HeaderValue},
};
use loco_openapi::prelude::*;
use loco_rs::{
    Result,
    app::{AppContext, Hooks, Initializer},
    bgworker::Queue,
    boot::{BootResult, StartMode, create_app},
    config::Config,
    controller::AppRoutes,
    environment::Environment,
    task::Tasks,
};
use reqwest::{Client, header::ACCEPT_ENCODING};

use crate::common;
#[allow(unused_imports)]
use crate::{controllers, tasks};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        // env!("CARGO_CRATE_NAME")
        "oic-fox-fuckery"
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA").or(option_env!("GITHUB_SHA")).unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment, config: Config) -> Result<BootResult> {
        create_app::<Self>(mode, environment, config).await
    }

    async fn initializers(ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        let mut initializers: Vec<Box<dyn Initializer>> = vec![];

        if ctx.environment != Environment::Test {
            initializers.push(
                Box::new(
                    loco_openapi::OpenapiInitializerWithSetup::new(
                        |ctx| {
                            #[derive(OpenApi)]
                            #[openapi(
                                modifiers(&SecurityAddon),
                                info(
                                    title = "oic-fox-fuckery",
                                    description = "An API that provides an augmented version of an OIC beer league ice hockey calendar with information about any overlapping shows at The Fox Theater"
                                )
                            )]
                            struct ApiDoc;
                            set_jwt_location(ctx.into());

                            ApiDoc::openapi()
                        },
                        None,
                    ),
                ) as Box<dyn Initializer>
            );
        }

        Ok(initializers)
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes() // controller routes below
            .add_route(controllers::cal::routes())
    }
    async fn connect_workers(_ctx: &AppContext, _queue: &Queue) -> Result<()> {
        Ok(())
    }

    #[allow(unused_variables)]
    fn register_tasks(tasks: &mut Tasks) {
        // tasks-inject (do not remove)
    }

    async fn after_routes(router: axum::Router, ctx: &AppContext) -> Result<axum::Router> {
        // Add a shared reqwest client that can be reused across requests
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("zstd, br, gzip, deflate"));
        let client = Client::builder()
            .zstd(true)
            .brotli(true)
            .gzip(true)
            .deflate(true)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .default_headers(headers)
            .tcp_nodelay(true)
            .tcp_keepalive(Duration::from_secs(60))
            .build()
            .unwrap();

        // Add a shared settings object that can be reused across requests
        let settings = common::settings::Settings::from_json(&ctx.config.settings.clone().ok_or(0).unwrap())?;

        Ok(router.layer(Extension(client)).layer(Extension(settings)))
    }
}
