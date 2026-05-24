/// functions to generate SQL strings
use syn::Type;

use crate::database_config::{DATABASE_PALCE_HOLDER, DEFAULT_PRIMARY_KEY_SQL_STR, get_mapper};
use crate::field_processor:: FieldProcessor ;
use crate::type_mapper::TypeMapper;

pub mod gen_insert_sql_str;
pub mod gen_create_table_sql_str;