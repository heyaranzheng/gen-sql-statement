// This file includes all the functions related to the sql of creating a new table 
//in the database.
use syn::{Type, Field, Data, DeriveInput};
use proc_macro2::TokenStream;
use quote::quote;


/// Map rust type to corresponding sql type, according to your database system.
pub trait TypeMapper {
    fn map_type(self: &Self, ty: &Type) -> &'static str;
}


/// This is the TypeMapper for postgres
pub struct PostgresTypeMapper;

impl TypeMapper for PostgresTypeMapper {
    /// Convert rust type to sql type
    fn map_type(self: &Self,ty: &Type) -> &'static str {
    match ty {
        syn::Type::Path(p) =>{
            let last_indet = p.path.segments.last().unwrap().ident.clone();
            match last_indet.to_string().as_str() {
                    // postgres's SQL types   
                "i32" | "u32" | "i64" | "u64" => "INTEGER",
                "f32" | "f64" => "REAL",
                "String" | "str" => "TEXT",
                "bool" => "BOOLEAN",
                "NaiveDate" => "DATE",          // chrono::NaiveDate
                "NaiveDateTime" => "TIMESTAMP", // chrono::NaiveDateTime
                "UuidBytes" => "BYTEA",         // reserves a name for binary data
                "Hash" => "BYTEA",              // reserves a name for binary data
                "Byte" => "BYTEA",              // reserves a name for binary data
                
                _=> "TEXT" ,
            }
        },

        _ => "TEXT",
    }
}   

}



// This is the TypeMapper for sqlite
pub struct SqliteTypeMapper;

impl TypeMapper for SqliteTypeMapper {
    fn map_type(self: &Self,ty: &Type) -> &'static str {
        match ty {
            syn::Type::Path(p) =>{
                let last_indet = p.path.segments.last().unwrap().ident.clone();
                match last_indet.to_string().as_str() {
                    // sqlite's SQL types   
                    "i32" | "u32" | "i64" | "u64" => "INTEGER",
                    "f32" | "f64" => "REAL",
                    "String" | "str" => "TEXT",
                    "bool" => "INTEGER",           // SQLite map BOOLEAN to INTEGER, 0 for false, 1 for true
                    "NaiveDate" => "TEXT",          // SQLite have no DATE type, use TEXT
                    "NaiveDateTime" => "TEXT",      // SQLite have no TIMESTAMP type, use TEXT
                    "UuidBytes" => "BLOB",          // SQLite use BLOB to store binary data
                    "Hash" => "BLOB",               // SQLite use BLOB to store binary data
                    "Byte" => "BLOB",               // SQLite use BLOB to store binary data 
                    
                    _=> "TEXT" ,
                }
            },

            _ => "TEXT",
        }
    }
}

#[cfg(feature = "only-for-postgres")]
pub fn get_mapper() -> impl TypeMapper {
    PostgresTypeMapper
}


/// This is the default mapper.
#[cfg(feature = "only-for-sqlite")]
pub fn get_mapper() -> impl TypeMapper {
    SqliteTypeMapper
}


#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_str;
    use crate::rust_type_mapper::TypeMapper;

    #[test]
    fn test_type_mapper() {
        // add all rust type name into the vec_ty_str
        let vec_ty_str = vec![
            "i32", "u32", "i64", "u64", "f32", "f64", "String", "str", 
            "bool", "NaiveDate", "NaiveDateTime", "UuidBytes", "Hash", "Byte"
        ];
        // parse the vec_ty_str to vec_ty of Type
        let test_vec_ty = vec_ty_str.iter().map(
            |s| parse_str::<Type>(*s).unwrap()
        ).collect::<Vec<_>>();

        // create a sqlite type mapper
        let sqlite_type_mapper = SqliteTypeMapper;
        // create a postgres type mapper
        let postgres_type_mapper = PostgresTypeMapper;

        // test the type mapper
        for (i, ty) in test_vec_ty.iter().enumerate() {
            let postgres_type_name = postgres_type_mapper.map_type(ty);
            let sqlite_type_name = sqlite_type_mapper.map_type(ty);
            println!("{} -> postgres: {}, sqlite: {}", vec_ty_str[i], postgres_type_name, sqlite_type_name);
        }

    }
}


