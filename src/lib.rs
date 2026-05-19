mod type_mapper;
mod field_processor;
mod database_config;
mod gen_sql_str;

use quote::{ToTokens, quote};
use syn::{ Data, DeriveInput, Field, Fields, parse_macro_input, Type };


use field_processor::{FieldInfo, FieldProcessor};
use type_mapper::{TypeMapper};
use database_config::{get_mapper};

use gen_sql_str::{
    gen_create_table_sql_str::create_table_sql_str, 
};



/// A trait that provides the "CREATE_TABLE_SQL" constant for generating the SQL CREATE TABLE statement
/// 
/// The SQL string is generated at **compile time**, no runtime overhead.
/// 
/// # Example
/// ```rust
/// use sql_str_creator::CreateTable;
/// 
/// #[derive(CreateTable)]
/// struct User {
///     #[primary_key]
///     id: i32,
///     #[ignore]
///     name: String,
///     age: i32,
/// }
/// 
/// // SQL is already generated at compile time!
/// println!("{}", User::create_table_sql());
/// ```
/// The output may be: CREATE TABLE IF NOT EXISTS user (id INTEGER PRIMARY KEY, age INTEGER);
/// 
/// # Note
/// * 1.The table name will be "user" instead of "User", we will convert it into all lower case.
/// * 2.It will provides a primary key "id" or "autoincrement",  if there is no primary_key attribute was
/// marked.
/// * 3. The real Sql string is dependent on the database you are using, you can use different
/// feature flags to switch between different databases.


/// The entry funtion of derive macro
#[proc_macro_derive(CreateTable, attributes(primary_key, ignore))]
pub fn derive_create_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //get the token stream, and parse it into a DeriveInput struct.
    let input = parse_macro_input!(input as DeriveInput);
    let field_processor = FieldProcessor::new(&input);
    
    //the ident we used to expand the macro
    let struct_ident = &input.ident;
    
    //prepare the arguments for the create_table_sql_str function
    let column_list = field_processor.get_column_list();
    let table_name = field_processor.get_table_name();
    let primary_key = field_processor.get_primary_key();

    //generate the sql string
    let sql_str = create_table_sql_str(&column_list, &table_name, &primary_key);

    //generate the code to implement the CreateTable trait
    let expand = quote!{
        //implement a function to return the sql string
        impl #struct_ident {
            pub fn create_table_sql(&self) -> &'static str {
                #sql_str
            }
        }

        //implement a function to insert sql data to the database
        impl #struct_ident {
           
        }
    };

    expand.into()


    
}