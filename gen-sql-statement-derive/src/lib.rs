mod type_mapper;
mod field_processor;
mod database_config;
mod gen_sql_str;

use quote::{ToTokens, quote};
use syn::{ Data, DeriveInput, Field, Fields, parse_macro_input, Type };


use field_processor::{FieldInfo, FieldProcessor};
use type_mapper::{TypeMapper};

use gen_sql_str::{
    gen_create_table_sql_str::create_table_sql_str, 
};



/// A trait that provides the "CREATE_TABLE_SQL" constant for generating the SQL CREATE TABLE statement
/// 
/// The SQL string is generated at **compile time**, no runtime overhead.
/// 
/// # Example
/// ```rust
/// use sql_str_creator::SqlStatement;
/// 
/// #[derive(SqlStatement)]
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
#[proc_macro_derive(SqlStatementTrait, attributes(primary_key, ignore))]
pub fn derive_create_table(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    //get the token stream, and parse it into a DeriveInput struct.
    let input = parse_macro_input!(input as DeriveInput);
    let field_processor = FieldProcessor::new(&input);
    
    //the ident we used to expand the macro
    let struct_ident = &input.ident;

    //get the generic parameters of the struct
    let generics = &input.generics;
    let (impl_generics, ty_generics, where_clausre) 
        = generics.split_for_impl();
    
    //prepare the arguments for the create_table_sql_str function
    let column_list = field_processor.get_column_list();
    let table_name = field_processor.get_table_name();
    let primary_key = field_processor.get_primary_key();

    //prepare a field names list for the get_field_names function
    let field_names = field_processor.get_field_names();

    //generate the sql string
    let sql_str = create_table_sql_str(&column_list, &table_name, &primary_key);

    //generate the code to implement the CreateTable trait
    let expand = quote!{
        use gen-sql-statement-trait::SqlStatementTrait;

        
        impl #impl_generics SqlStatementTrait for #struct_ident #ty_generics #where_clausre {
            /// Return the sql string of the create table statement
            pub fn create_table_sql(&self) -> &'static str {
                #sql_str
            }
            
            /// Return the all the field names of the struct
            /// # Note 
            /// the primary key's field name will be the first one in the list.
            fn get_field_names(&self) -> Vec<&'static str> {
                vec![#(#field_names), *]
            } 
        }

      
    };

    expand.into()


    
}