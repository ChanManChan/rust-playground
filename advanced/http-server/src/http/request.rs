use super::method::Method;

pub struct Request {
    method: Method,
    path: String,
    query_string: Option<String>
}

impl TryFrom<&[u8]> for Request {
    type Error = String;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}