//! A parser for the DSDL (Data structure description language) used in [uavcan](http://uavcan.org)
//!
//! For full description have a look at the [specification](http://uavcan.org/Specification/3._Data_structure_description_language/)
//!
//! ## Examples
//! ### Parse DSDL directory
//!
//! ```
//! use dsdl_parser::DSDL;
//!
//! assert!(DSDL::read("tests/dsdl/").is_ok());
//!
//! ```
//!
//! ### Parse single file
//!
//! ```
//! use dsdl_parser::DSDL;
//! 
//! assert!(DSDL::read("tests/dsdl/uavcan/protocol/341.NodeStatus.uavcan").is_ok());
//! 
//! ```
//!
//! ### Display a file
//!
//! ```
//! use dsdl_parser::DSDL;
//!
//! let dsdl = DSDL::read("./tests/dsdl/").unwrap();
//!
//! println!("{}", dsdl.get_file("uavcan.protocol.GetNodeInfo").unwrap());
//! 
//! ```
//!
//! ### Calculate data type signature
//!
//! ```
//! use dsdl_parser::DSDL;
//!
//! let dsdl = DSDL::read("./tests/dsdl/").unwrap();
//!
//! assert_eq!(dsdl.data_type_signature("uavcan.protocol.GetNodeInfo").unwrap(), 0xee468a8121c46a9e);
//! ```



#[macro_use]
extern crate log;

extern crate lalrpop_util;



mod ast;
mod parser;
mod crc;



// Re-export ast

pub use ast::ty::Ty;
pub use ast::ty::CompositeType;
pub use ast::ty::PrimitiveType;

pub use ast::ident::Ident;

pub use ast::comment::Comment;

pub use ast::directive::Directive;

pub use ast::attribute_definition::AttributeDefinition;
pub use ast::attribute_definition::ConstDefinition;
pub use ast::attribute_definition::FieldDefinition;

pub use ast::file_name::FileName;
pub use ast::file_name::Version;

pub use ast::array::ArrayInfo;

pub use ast::cast_mode::CastMode;

pub use ast::lit::Lit;
pub use ast::lit::Sign;

pub use ast::type_definition::TypeDefinition;
pub use ast::type_definition::MessageDefinition;
pub use ast::type_definition::ServiceDefinition;

pub use ast::line::Line;

pub use ast::file::File;
pub use ast::file::NormalizedFile;

pub use ast::dsdl::DSDL;

pub use parser::error;
