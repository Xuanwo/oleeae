use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};
use uuid::Uuid;

pub struct RequestIdMiddleware;

#[derive(Default, Clone)]
pub struct RequestId(Option<String>);

impl Fairing for RequestIdMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Request ID",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        request
            .local_cache(|| RequestId(Some(format!("{}", Uuid::new_v4().to_simple().to_string()))));
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        let id = request.local_cache(|| RequestId(None)).clone();
        response.set_raw_header("x-oleeae-request-id", id.0.unwrap());
    }
}
