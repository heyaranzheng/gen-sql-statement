/// the configuration for diffent databases

use crate::type_mapper::{PostgresTypeMapper, SqliteTypeMapper, TypeMapper};


///get the current mapper for the database
//for postgres
#[cfg(feature = "only-for-postgres")]
pub fn get_mapper() -> impl TypeMapper {
    PostgresTypeMapper
}
//for sqlite
#[cfg(feature = "only-for-sqlite")]
pub fn get_mapper() -> impl TypeMapper {
    SqliteTypeMapper
}

///The palce_holder of the database configuration

//for sqlite
#[cfg(feature = "only-for-sqlite")]
pub const DATABASE_PALCE_HOLDER: &str = "?";
//for postgres
#[cfg(feature = "only-for-postgres")]
pub const DATABASE_PALCE_HOLDER: &str = "$";

///The default primary key of the database
//for postgres
#[cfg(feature = "only-for-postgres")]
pub const DEFAULT_PRIMARY_KEY_SQL_STR:  &str = " SERIAL PRIMARY KEY ";
//for sqlite
#[cfg(feature = "only-for-sqlite")]
pub const DEFAULT_PRIMARY_KEY_SQL_STR:  &str = " INTEGER PRIMARY KEY AUTOINCREMENT ";

/// a name of current database
#[cfg(feature = "only-for-postgres")]
pub const DEFAULT_DATABASE_NAME: &str = "postgres";
#[cfg(feature = "only-for-sqlite")]
pub const DEFAULT_DATABASE_NAME: &str = "test.db";
