use utoipa::{
    openapi::{
        self,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(

    ),
components(
    schemas(

    )
),
tags((name = "BasicAPI", description = "A very Basic API")),

)]
pub struct ApiDoc;