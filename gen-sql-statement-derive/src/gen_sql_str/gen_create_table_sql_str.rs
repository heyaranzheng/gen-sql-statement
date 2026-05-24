/// functions to generate SQL strings


use crate::database_config::{DATABASE_PALCE_HOLDER, DEFAULT_PRIMARY_KEY_SQL_STR};

pub fn create_table_sql_str( 
    column_list: &Vec<(String, String)>,
    table_name: &String, 
    primary_key: &(String, String),
)-> String 
{
    let mut part1 = format!("CREATE TABLE IF NOT EXISTS {} ", table_name);
    
    //part2: "("
    let mut part2 = "(".to_string();

    
    //part2: "( id INTEGER PRIMARY KEY AUTOINCREMENT, " or "( autoincrement INTEGER PRIMARY 
    //KEY AUTOINCREMENT, " for sqlite, the similar for postgres
    let str = format!("{} {}," , primary_key.0, primary_key.1);
    part2.push_str(&str);

    //part2: "( id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, age INTEGER "
    //part3, the similar string like "name TEXT, age INTEGER "
    let part3 = sql_str_field_type(column_list);

    //push part3 to part2. 
    //part2: "( id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, age INTEGER "
    part2.push_str(&part3);

    //add the last ")"
    part2.push_str(" )");

    //add it to the part1
    part1.push_str(&part2);
    
    part1
}

/// a helper to find if the "id" or "autoincrement" field is in vector of String
#[inline(always)]
fn check_id_or_autoincrement(column_list: &Vec<(String, String)>) -> (bool, bool) {
    let mut has_id = false;
    let mut has_autoincrement = false;
    for (name, _) in column_list {
        if name == "id" {
            has_id = true;
        } else if name == "autoincrement" {
            has_autoincrement = true;
        }
    }
    (has_id, has_autoincrement)

}

/// a helper to generate a suitable primary key string
/// like: id INTEGER PRIMARY KEY AUTOINCREMENT, or autoincrement INTEGER PRIMARY KEY AUTOINCREMENT for sqlite
/// or id SERIAL PRIMARY KEY or autoincrement SERIAL PRIMARY KEY for postgres
#[inline(always)]
fn get_primary_key_sql_str(
    column_list: &Vec<(String, String)>,
    primary_key: &Option<(String, String)>
) -> String {
    let part_primary_key_sql_str; 
    if let Some((str, sql_type)) = &primary_key {

        //like: "id INTEGER PRIMARY KEY " if your id is used like i32, i64 in sqlite,
        //the same for postgres. This only hanppens when you set up a primary key in your struct.
        part_primary_key_sql_str = format!("{} {} PRIMARY KEY, ", str, sql_type);  

    } else {
        //check if we have an same name of id, if it is we use autoincrement as the primary key
        //otherwise we will panic if we have a field named autoincrement too.
        let (has_id, has_autoincrement) = check_id_or_autoincrement(column_list);
        if has_id && has_autoincrement {
            panic!("your struct has both id and autoincrement fields, so I can't use a default 
                primary key for you, please specify a primary key in your struct"
            );
        }else if has_id {
            //at here, we have a field named id, so we use autoincrement as the primary key
            part_primary_key_sql_str = format!("autoincrement  {}, ", DEFAULT_PRIMARY_KEY_SQL_STR);
        }else {
            //we have no field named id, so we use id as the primary key
            part_primary_key_sql_str = format!("id {}, ", DEFAULT_PRIMARY_KEY_SQL_STR);
        }
    }
    part_primary_key_sql_str
}

/// a helper funtion to construct a string like " $1, $2, $3 " for postgres,
/// or " ?1, ?2, ?3 " for sqlite
#[inline(always)]
fn palce_holder_and_number_str(totol:  usize) -> String {
    let mut  str = String::new();
     // construct like: $1, $2, $3 for postgres, or ?1, ?2, ?3 for sqlite, and so on.
    for index in 0 .. totol {
            if index != totol - 1 {
                str.push_str(DATABASE_PALCE_HOLDER);
                str.push_str(&index.to_string());
                str.push_str(", ");
            } else {
                str.push_str(DATABASE_PALCE_HOLDER);
                str.push_str(&index.to_string());
            }
    }

    str

}

/// a helper function to generate a string like "name TEXT, age INTEGER "
/// # Arguments
/// * `column_list` - the vector of tuple consists of (field_name, sql_type_name).
#[inline(always)]
fn sql_str_field_type(column_list: &Vec<(String, String)>) -> String {
    let len = column_list.len();
    let mut ret = String::new();
    let mut _temp = String::new();

    //we leave the last field type to the last
    for i in 0..(len - 1) {
        _temp = format!("{} {}, ", column_list[i].0, column_list[i].1);
        ret.push_str(&_temp);
    }
    let temp = format!("{} {}", column_list[len - 1].0, column_list[len - 1].1);
    ret.push_str(&temp);
    
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;
    use syn::DeriveInput;
    use crate::field_processor::FieldProcessor;

    fn gen_test_token_stream() -> DeriveInput {
        parse_quote! {
            struct TestStruct {
                #[primary_key]
                id: i32,
                #[ignore]
                name: String,
                #[ignore]
                age: i32,
                height: f32,
                weight: f32,
                #[ignore]
                score: u32,
            }
        }
    }
    

    fn gen_test_token_stream_without_primary_key() -> DeriveInput {
        parse_quote! {
            struct TestStruct2 {
                id: i32,
                #[ignore]
                name: String,
                #[ignore]
                age: i32,
                height: f32,
                #[ignore]

            }
        }

    }
    #[test]
    fn sql_str_field_type_test() {
        let input = gen_test_token_stream();
        let field_processor = FieldProcessor::new(&input);
        let column_list = field_processor.get_column_list();

        println!("{:?}", column_list);
    }

    use crate::database_config::{DEFAULT_DATABASE_NAME};
    #[test]
    fn test_create_table_sql_str() {
        let input = gen_test_token_stream();
        let field_processor = FieldProcessor::new(&input);

     
        let table_name = field_processor.get_table_name();
        let primary_key = field_processor.get_primary_key();
        let column_list = field_processor.get_column_list();

        let sql_str = create_table_sql_str(
            &column_list,
            &table_name,
            &primary_key,
        );
        
        println!("{}", sql_str);
        if DEFAULT_DATABASE_NAME == "sqlite" {
            assert_eq!(sql_str,
                "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY AUTOINCREMENT, height REAL, weight REAL, score INTEGER )"
            );
        }else if DEFAULT_DATABASE_NAME == "postgres" {
            assert_eq!(sql_str,
                "CREATE TABLE IF NOT EXISTS test_table (id INTEGER PRIMARY KEY, height INTEGER, weight REAL )"
            )
        }

    }

    



}