use crate::ast::Hotlist;

use std::error::Error;
use std::path::Path;

pub fn emit<T: AsRef<Path>>(
    filename: Option<T>,
    hl: &Hotlist,
    multi: bool,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    Err(From::from("HTML generation yet implemented."))
}
