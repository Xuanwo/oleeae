use rocket::request::{self, Request, FromRequest, State};
use rocket::outcome::Outcome::*;
use uuid::Uuid;

/// A type that represents a request's ID.
pub struct RequestId(pub String);

/// Returns the current request's ID, assigning one only as necessary.
impl<'a, 'r> FromRequest<'a, 'r> for RequestId {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        let id = format!("{}", Uuid::new_v4().to_simple().to_string());

        request.local_cache(|| RequestId(id.clone()));

        Success(RequestId(id))
    }
}