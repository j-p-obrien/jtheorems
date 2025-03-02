use crate::term::variable::FreeVariable;

pub(crate) struct LocalContext {
    vars: Vec<FreeVariable>,
}

impl LocalContext {
    pub(crate) fn new() -> Self {
        Self { vars: Vec::new() }
    }
}
