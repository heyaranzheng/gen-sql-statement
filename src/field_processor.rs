use syn::{DeriveInput, Field, Data};
use std::any::Any;
use std::sync::Arc;

use crate::database_config::get_mapper; 
use crate::TypeMapper;


/// Field Processor, including the information of the all the fields in a struct 
#[derive(Clone)]
pub struct FieldProcessor {
    //all the fields info in the struct
    fields: Vec<FieldInfo>,
    //the name of the struct
    struct_ident: String,
    //the name of the primary key field, if we have not set it, it will create a primary key.
    primary_key: (String, String),    
    //ignore the fields that are not used in the table or sql statement
    ignore_fields_name: Vec<String>,
    //parmeters for the database connection
    //parms: Vec<Arc<dyn Any + Send + Sync>>,
}



///The field information in a struct
#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty_name: String,
    pub is_primary_key: bool,
    pub is_ignore: bool,
}

impl FieldInfo {
    // Create a new FieldInfo derectly from Field
    fn new(name: String, ty_name: String, is_primary_key: bool, is_ignore: bool) -> Self {
        Self {
            name,
            ty_name,
            is_primary_key,
            is_ignore,
        }
    }
}

///
impl FieldProcessor {
    /// Create a new FieldProcessor derectly from DeriveInput
    /// # Note: Only struct can be used with this function
    pub fn new(input: &DeriveInput) -> Self {
        let struct_ident = input.ident.to_string();
        let mut  primary_key_opt: Option<(String, String)> = None;
        let mut ignore_fields_name: Vec<String> = Vec::new();

        //this used to check if we have already have a field name with id or autoincrement
        //in the current struct.
        let mut have_field_name_with_id = false;
        let mut have_field_name_with_auto_increment = false;
        
        //this is the mapper to map the rust type into sql type for your specific database
        let mapper = get_mapper();


        //------------------iter the fields to figure out the primary key field------------
        let fields = match & input.data {
            Data::Struct(s) => {
                let mut is_ignore = false;
                let mut is_primary_key = false;
                

                //iter the fields of the struct
                s.fields.iter().map(
                    |f:&Field | {
                        //reset the flags
                        is_ignore = false;
                        is_primary_key = false;

                        
                        let field_name = f.ident.as_ref().unwrap().to_string();
                
                        let field_ty_name = mapper.map_type(&f.ty).to_string();

                        //check if the field name is id or autoincrement, record it.
                        if field_name == "id" {
                           have_field_name_with_id = true;
                        }else if field_name == "autoincrement" {
                            have_field_name_with_auto_increment = true;
                        }

                        //iter the attributes of the field
                        f.attrs.iter().for_each(
                            //check if the field is primary key or need to be ignored in
                            // the table or sql statement
                            |a| {
                                let attr_ident = a.path.get_ident().expect("error in attr of fields in a struct");
                                if attr_ident == "ignore" {
                                    ignore_fields_name.push(field_name.clone());
                                    is_ignore = true;
                                }else if attr_ident == "primary_key" {
                                    //check if we have already set the primary key field, if it is, that's
                                    //an error, store the primary 
                                    if primary_key_opt.is_none() {
                                        primary_key_opt = Some((field_name.clone(), field_ty_name.clone()));
                                        is_primary_key = true;
                                    }else{
                                        //we can't mark two fields as primary key in the same sql table
                                        panic!("Error: multiple primary key fields found in struct {}", struct_ident);
                                    }
                                }

                            }
                        );

                        FieldInfo::new(field_name, field_ty_name, is_primary_key, is_ignore)
                    }
                
                //collect the fields info in the struct    
                ).collect::<Vec<FieldInfo>>()
            
            },


            _=> panic!("Error: only struct can be used to create a table"),
        };


        let mut primary_key = (String::new(), String::new());   
        //check if the user has set the primay key field, if not, we will give a default primary
        //key, it will be named as id or autoincrement.
        //If the user has not set the primary key field, and the name of "id" and "autoincrement"
        //was occupied by other fields, we will panic.
        if let Some(pk) = primary_key_opt {
            primary_key = pk;
        }else{
            //the user has not set the primary key field, we will give a default primary key
            if have_field_name_with_id && have_field_name_with_auto_increment {
                //the name of "id" and "autoincrement" was both occupied by other fields  
                panic!("you should set your 'id' or 'autoincrement' field as primary key, I 
                    can't give a default primary key by name of 'id' or 'autoincrement'."
            );
            }else if have_field_name_with_id {
                //id was occupied by other fields
                primary_key.0 = "autoincrement".to_string();
            }else{
                //autoincrement was occupied by other fields
                primary_key.0 = "id".to_string();
            }
        }

        //get a suitable sql type for the primary key field
        primary_key.1 = crate::database_config::DEFAULT_PRIMARY_KEY_SQL_STR.to_string();



        Self {
            fields,
            struct_ident,
            primary_key,
            ignore_fields_name,
        }
        
    }


    //get all feilds info in the struct
    pub fn get_fields(&self) -> &Vec<FieldInfo> {
        &self.fields
    }

    ///get the name of the struct, we will convert it into all lower case
    /// # Note: 
    ///  It will return the name of the struct in lower case, it  is a string, not a
    /// ident type of token stream. It is the table name. 
    pub fn get_table_name(&self) -> String{
        self.struct_ident.to_lowercase()
    }

    //get the name of the primary key field
    pub fn get_primary_key(&self) -> (String, String) {
        self.primary_key.clone()
    }

    pub fn get_ignore_fields_name(&self) -> &Vec<String> {
        &self.ignore_fields_name
    }

    /// This function get the list of the column names of the table, attached with its
    /// type name in SQL.
    /// # Arguments
    /// * field_processor: &FieldProcessor, the reference of the FieldProcessor struct that 
    /// contains all the information of the struct.
    /// 
    /// # Return
    /// * column_list: Vec<(String, String)>, the column name is the first element of the tuple,
    /// the type name in SQL is the second element of the tuple.
    /// ```
    /// //after using this function, the column_list may's data may look like the vector below:
    /// let list_without_primary_key_may_be = [("id", "INTEGER"), ("name", "TEXT"), ("age", "INTEGER")]
    /// //if you use a primary_key marked on the id field,
    /// //the column_list may's data may look like the vector below:
    /// let column_list_may_be = [("name", "TEXT"), ("age", "INTEGER")]
    /// ```
    /// 
    /// # Note: 
    /// It will filter out the fields that are marked as ignore or primary key.
    /// So the primary key will NOT be included in the column list.
    pub fn get_column_list(&self) -> Vec<(String, String)> 
    {
        self.get_fields().iter().filter(|f| !f.is_primary_key && !f.is_ignore)
            .map(|f| (f.name.clone(), f.ty_name.clone()))
            .collect::<Vec<(String, String)>>()
    }

}



#[cfg(test)]
mod tests {
   

    use super::*;
    use syn::parse_quote;

    ///a function to create a struct, and parse it into a DeriveInput
    /// # Note:
    /// This function should not be changed for the test structure we create inside 
    /// will INFLUENCE the test result of the other tests DIRECTLY.
    #[inline(always)]
    fn gen_test_token_stream() -> DeriveInput {
        parse_quote! {
            struct TestStruct {
                #[primary_key]
                id: i32,
                name: String,
                #[ignore]
                age: i32,
            }
        }
    }

    #[test]
    fn test_field_processor() {
        let input = gen_test_token_stream();

        let field_processor = FieldProcessor::new(&input);
        let fields = field_processor.get_fields();
        let table_name = field_processor.get_table_name();
        let primary_key = field_processor.get_primary_key();
        let ignore_fields_name = field_processor.get_ignore_fields_name();

        assert_eq!(fields.len(), 3);

        //the ident will be converted to lower case
        assert_eq!(table_name, "test_struct");    

        assert_eq!(primary_key.0, "id".to_string());
        assert_eq!(ignore_fields_name.len(), 2);
        assert_eq!(ignore_fields_name[0], "name");
        assert_eq!(ignore_fields_name[1], "age");

    }

    #[test]
    fn test_get_column_list() {
        let input = gen_test_token_stream();

        let field_processor = FieldProcessor::new(&input);
        let column_list = field_processor.get_column_list();

        assert_eq!(column_list.len(), 1);
        assert_eq!(column_list[0].0, "name");
        assert_eq!(column_list[0].1, "TEXT");

    }
  
  

}