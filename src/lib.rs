mod type_mapper;
mod field_processor;
mod sql_str_creator;

use quote::quote;
    use syn::{ Data, DeriveInput, Field, Fields};
/* 
//For the macro we shared must use the signature of proc_macro::TokenStream, but in the test we 
//can't use the input as the proc_macro::TokenStre. So, we wrap the core function, which has
//the proc_macro2::TokkenStream, into another function which has an proc_macro::TokenStream input.
#[proc_macro_derive(TableCreator, attributes(table_ignore, primary_key))]
pub fn derive_table_creator( input :proc_macro::TokenStream ) -> proc_macro::TokenStream {
    let input: DeriveInput = syn::parse(input as proc_macro::TokenStream).expect("No input");
    //this is the name of the struct.
    let name = input.ident;

    //This is the name of the table,  with the lowercase.
    // struct Abcd => table_name: abcd.
    let name_lower = name.to_string().to_lowercase();


    let generics = input.generics;

    let fields = match input.data {
        Data::Struct(ref data) =>  match &data.fields {
            Fields::Named(fileds) => &fileds.named,
            _ => panic!("Only named fields are supported"),
        },

        _ => panic!("Only Struct are supported"),
    };


    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    
    //The primary should be the unique one, so we declare primary_info like this is reasonalble.
    //If the primary_key is not exsited, we just use a default one, named it with "id".
    let mut primary_info : Option<(String, syn::Type)> = None;
    
    //We will filter the fields with the arttubute #[table_ignore], and find the primary 
    let fields: Vec<&Field> = fields.iter()
        .filter(|f| {
            //This two is the flags to mark this field.
            let mut is_primary = false;
            let mut is_ignore = false;
            //This is the counter for the attribute #[primary_key], if the counter != 1,
            //just panic!
            let mut counter = 0; 
   

            //check there is or not an attribute: #[table_ignor]
            let _ = f.attrs.iter().for_each(|a|{
                //check the attributes
                if let Some(last) = a.path.segments.last() {
                    if last.ident == "table_ignore" {
                        is_ignore = true;
                    }else if last.ident == "primary_key" {
                        is_primary = true;
                        counter += 1;
                        eprintln!("find the attr primary_key in {}", &f.ident.as_ref().unwrap().to_string());
                    }
                }
              
            });
            //if this field have attribute #[primary_key], set the primary_info once.
            if is_primary {
                let ident = f.ident.as_ref().unwrap().to_string();
                let ty = f.ty.clone();
                //check counter of the primary key.
                if counter != 1 {
                    panic!("Attribute error, can't have more #[primary_key] than once");
                }
                primary_info = Some((ident, ty));
            }
            //if have the attribute #[table_ignore], is_ignore will set with true, 
            //that means we must skip this field, so we return the NOT value of is_ignore 
            //to the filter, beacuse of the itor with true value would be skiped.
            !is_ignore

                
        })
        //check there is or not an attribute: #[primary_key].
        //If it's tru, we will set the element as the primary key, or use a default "id" as the primary key instead.      
    .collect();
    
    //store the ident of the column into the vector.  quote will pase it as the column name.
    let column_ident: Vec<_>  = fields.iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect();
    //store the transformed type into the vector. 
    let fields_create: Vec<String> = fields.iter().map( |f|{
        let field_name = f.ident.as_ref().unwrap().to_string();
        
        
        let ty = match &f.ty {
            syn::Type::Path(p) =>{
                let last_indet = p.path.segments.last().unwrap().ident.clone();
                match last_indet.to_string().as_str() {
                    //sqlite's sql types   
                    "i32" | "u32" | "i64" | "u64" => "INTEGER",
                    "f32" | "f64" => "REAL",
                    "String" | "str" => "TEXT",
                    "bool" => "BOOLEAN",
                    "NaiveDate" => "DATE",          // chrono::NaiveDate
                    "NaiveDateTime" => "TIMESTAMP", // chrono::NaiveDateTime
                    "UuidBytes" => "BYTEA",         //[u8;16]
                    "Hash" => "BYTEA",              //[u8;32]
                    
                    _=> "TEXT" ,
                }
            }, 
            _=> "TEXT",
        };
        //if the primary_info is not none.
        if let Some(primary_info) = primary_info.clone() {
            //if we have a primary key, add the string "PRIMARY KEY" after the field name.
            //at here, we can unwrap it safely.
            if field_name == primary_info.clone().0 {
                //need some space before the "P"
                return  format!("{} {} PRIMARY KEY", field_name, ty);
            }
        }
        return format!("{} {}", field_name, ty);
        
    })
    .collect(); 

    //sotre the field name into the column_names.
    //why we collect the filed name at hear? Because we need to know if there is a primary key.
    let mut column_names:Vec<String> = Vec::new();
    
    fields.iter().for_each(|f|{
        let field_name = f.ident.as_ref().unwrap().to_string();
        column_names.push(field_name.clone());
    });

    //store the state of the primary_info.
    let have_primary_key = primary_info.is_some();

    //Note: the default primary key named "id" at the LAST position of the column_names.
    if !have_primary_key {
        // have no primary key, so we use the default "id" as the primary key.
        // We must delete the this element below if we want to use the column_names as the real column names.
        // The id is a serial primary key, so we don't need to set it in the insert sql.
        column_names.push(" id SERIAL PRIMARY KEY ".to_string());
    }

    //this is the sql to create a tale we will return.
    let create_sql: String;

    //this is the sql to insert a row into the table.
    let insert_sql: String;

    //if we have a primary key.
    if have_primary_key {
        create_sql = format!("CREATE TABLE IF NOT EXISTS  {} ( {});", 
            name_lower, fields_create.join(", "));
        
        //question mark is the placeholder for the values wich will be inserted.
        //insertsql will be like this: "INSERT INTO A ( pid, hash, uuid ) VALUES ( $1, $2, $3 )", 
        let mut  vec_palceholder = Vec::new();
        for i in 1 ..= column_names.len() {
            let i_str = i.to_string();
            let placeholder = format!("${}", i_str);
            vec_palceholder.push(placeholder);
        }
        insert_sql = format!("INSERT INTO {} ( {} ) VALUES ( {} )", 
            name, column_names.join(", "), vec_palceholder.join(", "));
        
    }else {//have no primary key, but have the default "id" as the primary key.

        //we need to know the "id" is the last element of the column_names.
        //we must delet the "id" before we create a insert sql, for the id is a serial primary key, 
        //so we don't need to set it in the insert sql.
        create_sql = format!("CREATE TABLE IF NOT EXISTS {}  ( {} );",
            name_lower, fields_create.join(", "));

        //delete the "id" from the column_names.
        column_names.pop();
        
        //question mark is the placeholder for the values wich will be inserted.
        //insertsql will be like this: "INSERT INTO a ( pid, hash, uuid ) VALUES ( ?, ?, ? )", 
        let vec_question_mark = vec!["?"; fields.len()];
        insert_sql = format!("INSERT INTO {} ( {} ) VALUES ( {} )", 
            name_lower, column_names.join(", "), vec_question_mark.join(", "));
    }
    //Note: at here, column_names is the real column names, not include the "id" at the end.

    println!("create_sql: {}", create_sql);
    println!("insert_sql: {}", insert_sql);
   
    
    //the name of method
    let expanded = quote! {

        impl #impl_generics #name  #ty_generics #where_clause{
            ///this is the sql to create a tale.
            pub fn sql_create() -> String {
                #create_sql.to_string()
            }
            
            ///this is the sql to insert a row into the table.
            pub fn sql_insert() -> String {
                #insert_sql.to_string()
            }          
        
            ///function to generate the arguments for sqlx query.
            ///Note: this function only valid for the feature "sqlx-support".
            /// 
            ///If your struct A have derived the TableCreator, you can use it to generate the arguments 
            ///for sqlx query like this:
            /// A a = A {.....  }
            /// let args = a.to_sqlx_args();
            #[cfg(feature = "sqlx-support")]
            pub fn to_sqlx_args(&self) ->sqlx::postgres::PgArguments {
                let mut args = sqlx::postgres::PgArguments::default();
                
                // We need to import the trait `Arguments` to use the `add` method, it is required by sqlx for
                // "add" method to work.
                use sqlx::Arguments; 
                #(args.add(&self.#column_ident);)*
                args
            }
        } 
        
    };

    proc_macro::TokenStream::from(expanded)
}
*/

/*
#[cfg(test)]
mod test{

    // 在宏crate的tests/debug_test.rs
    use super::*;
    use syn::{parse_quote, DeriveInput};

    #[test]
    fn debug_macro() {
        let input: DeriveInput= parse_quote! {
           #[derive(TableCreator,Debug)]
            struct A {
                #[primary_key]
                pid: i32,
                #[table_ignore]
                hash: Hash,
                uuid: UuidBytes,
            }
         
        };
     
        let tokens = derive_table_creator_impl(proc_macro2::TokenStream::from( quote! { #input } ).into() ); 
        

        println!("{}", tokens);
    }
}
*/