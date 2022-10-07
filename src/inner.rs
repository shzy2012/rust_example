use std::{borrow::Cow, collections::HashMap};

pub struct Form {
    pub inner: FormParts<Part>,
}

pub struct FormParts<P> {
    pub boundary: P,
    // pub(crate) computed_headers: Vec<Vec<u8>>,
    // pub(crate) fields: Vec<(Cow<'static, str>, P)>,
}

pub struct Part {}

pub struct Body {}
