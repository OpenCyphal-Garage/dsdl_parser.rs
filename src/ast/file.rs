//! Everything related to uavcan file `File`.

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;


use ast::file_name::FileName;
use ast::type_definition::TypeDefinition;

/// A DSDL file
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub name: FileName,
    pub definition: TypeDefinition,
}

impl Display for File {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "File: {}\n{}", self.name, self.definition)
    }
}
