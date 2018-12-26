use rocket::request::Form;
use rocket_contrib::json::JsonValue;

use std::format;
use utils::RequestId;

#[derive(Clone, Debug, Serialize, Deserialize, FromForm)]
pub struct GetPackagesRequest {
    id: Option<i64>,
    name: Option<String>,

    maintainer_id: Option<i64>,
    maintainer_name: Option<String>,
    maintainer_email: Option<String>,

    offset: Option<i64>,
    limit: Option<i32>,
}

#[get("/packages?<req..>")]
pub fn get_packages(request_id: RequestId, req: Option<Form<GetPackagesRequest>>) -> JsonValue {
    println!("{:?}", request_id.0);
    json!(format!("{:?}", req))
}

#[derive(Clone, Debug, Serialize, Deserialize, FromForm)]
pub struct GetMaintainersRequest {
    id: Option<i64>,
    name: Option<String>,

    email: Option<String>,
    package_id: Option<i64>,
    package_name: Option<String>,

    offset: Option<i64>,
    limit: Option<i32>,
}

#[get("/maintainers?<req..>")]
pub fn get_maintainers(req: Option<Form<GetMaintainersRequest>>) -> JsonValue {
    json!(format!("{:?}", req))
}

#[derive(Clone, Debug, Serialize, Deserialize, FromForm)]
pub struct GetBuildsRequest {
    package_id: Option<i64>,
    package_name: Option<String>,

    maintainer_id: Option<i64>,
    maintainer_name: Option<String>,
    maintainer_email: Option<String>,

    offset: Option<i64>,
    limit: Option<i32>,
}

#[get("/builds?<req..>")]
pub fn get_builds(req: Option<Form<GetBuildsRequest>>) -> JsonValue {
    json!(format!("{:?}", req))
}
