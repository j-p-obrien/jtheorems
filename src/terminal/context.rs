use crate::term::variable::FreeVariable;

pub(super) struct LocalContext {
    vars: Vec<FreeVariable>,
}

impl LocalContext {
    pub(super) fn new() -> Self {
        Self { vars: Vec::new() }
    }
}
