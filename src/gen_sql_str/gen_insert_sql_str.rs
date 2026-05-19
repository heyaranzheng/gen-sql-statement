///Thi function was used to generate the insert sql string for the given table and columns.
use super::*;

use crate::database_config::DATABASE_PALCE_HOLDER;

pub fn gen_inser_sql_str(
    column_list: &Vec<(String, String)>,
    table_name: &str,
    primary_key: (String, String)
) -> String {
    
    //ret: "INSERT INTO"
    let mut ret = String::from("INSERT INTO ");

    //ret: "INSERT INTO 'table_name'"
    ret.push_str(table_name);

    //add a '(' and the primary key at the first palce.
    //ret may like this if you set id as primary key for example:
    // "INSERT INTO 'table_name' ( id, name, age) " 
    // if you not we will add an default primary key like 'id' or 'autoincrement'
    let mut str =  format!("( {}, {} )", primary_key.0, gen_field_name_str(column_list) );
    ret.push_str(&str);

    

    //add the value list
    //ret may be like this : "INSERT INTO 'table_name' (id, name, age) VALUES (?1, ?2, ?3)" for
    //sqlite.
    str = format!(" VALUES ({})", gen_value_list(column_list));
    ret.push_str(&str);

    
    ret
}

/// function to generate string like " id, name, age" with a given column list.
/// if the you have set primary_key on id for example, the string will be like 
/// " name, age" 
/// # Note
///  this will IGNORE the primary key field and the field marked with ignore.
fn gen_field_name_str(column_list: &Vec<(String, String)>) -> String {
    let mut ret = String::new();
    let len = column_list.len();
    let mut _temp = String::new();
    for i in 0 .. (len - 1) {
        _temp = format!(" {}, ", column_list[i].0);
        ret.push_str(&_temp);
    }

    //leave the last field name without comma
    ret.push_str(&column_list[len - 1].0);

    ret
}

/// function to generate string like " ?1, ?2, ?3" with a given column list, if you set
/// feature "only-for-sqlite".
/// For postgres, it will be like " $1, $2, $3"
fn gen_value_list(column_list: &Vec<(String, String)>) -> String {
    let mut ret = String::new();
    let len = column_list.len();
    let mut _temp = String::new();
    
    //we need to remain a placeholder for the primary key field. So we need to add 1 to the
    //length of the column list.
    for i in 0 .. len  {
        _temp = format!(" {}{}, ",DATABASE_PALCE_HOLDER, i + 1);
        ret.push_str(&_temp);
    }

    //leave the last field name without comma
    ret.push_str(&format!(" {}{}", DATABASE_PALCE_HOLDER, len + 1));

    ret
}


#[cfg(test)]
mod tests{
    use super::*;

    use syn::parse_quote;
  
    /// parse the test struct, and return some objects for test.
    /// # Returns
    /// * `Field_processor` - the FieldProcessor object for the test struct.
    /// * 'Vec<(String, String)>` - the column list of the test struct.
    /// * `table_name` - the table name of the test struct.
    /// * `primary_key` - the primary key of the test struct.
    fn gen_test_token_stream() 
    -> (FieldProcessor,
        Vec<(String, String)>, 
        String, 
        (String, String)
    )
        
    {
        let input = parse_quote! {
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
        };

        let field_processor = FieldProcessor::new(&input);   
        let column_list = field_processor.get_column_list();
        let table_name = field_processor.get_table_name();
        let primary_key = field_processor.get_primary_key();

        (field_processor, column_list, table_name, primary_key)
    }
    
    #[test]
    fn test_gen_field_name_str() {
        let (_, column_list, _, _) = gen_test_token_stream();
        
        let field_name_str = gen_field_name_str(&column_list);
        assert_eq!(field_name_str, " height, weight");

    }

    #[test]
    fn test_gen_insert_sql_str() {
        let (field_processor, column_list, 
            table_name, primary_key
        ) = gen_test_token_stream();

        let str = gen_inser_sql_str(&column_list, &table_name, primary_key);
        println!("{}", str);
    }
}