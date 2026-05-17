use syn::{DeriveInput, Field, Type, Data};


/// Field Processor, including the information of the all the fields in a struct 
#[derive(Clone)]
pub struct FieldProcessor {
    //all the fields info in the struct
    fields: Vec<FieldInfo>,
    //the name of the struct
    struct_ident: String,
    //the name of the primary key field, if we have not set it, it will create a primary key.
    primary_key: Option<String>,    
    //ignore the fields that are not used in the table or sql statement
    ignore_fields_name: Vec<String>,
}



///The field information in a struct
#[derive(Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: Type,
    pub is_primary_key: bool,
    pub is_ignore: bool,
}

impl FieldInfo {
    // Create a new FieldInfo derectly from Field
    fn new(name: String, ty: Type, is_primary_key: bool, is_ignore: bool) -> Self {
        Self {
            name,
            ty,
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
        let mut  primary_key: Option<String> = None;
        let mut ignore_fields_name: Vec<String> = Vec::new();
        let fields = match & input.data {
            Data::Struct(s) => {
                let mut _field_name: String = String::new();
                let mut is_ignore = false;
                let mut is_primary_key = false;
                

                //iter the fields of the struct
                s.fields.iter().map(
                    |f:&Field | {
                        _field_name = f.ident.as_ref().unwrap().to_string();
                        let field_ty = f.ty.clone();

                        //iter the attributes of the field
                        f.attrs.iter().for_each(
                            //check if the field is primary key or need to be ignored in
                            // the table or sql statement
                            |a| {
                                let attr_ident = a.path.get_ident().expect("error in attr of fields in a struct");

                                
                                if attr_ident == "ignore" {
                                    ignore_fields_name.push(_field_name.clone());
                                    is_ignore = true;
                                }else if attr_ident == "primary_key" {
                                    //check if we have already set the primary key field, if it is, that's
                                    //an error
                                    if primary_key.is_none() {
                                        primary_key = Some(_field_name.clone());
                                        is_primary_key = true;
                                    }else{
                                        //we can't mark two fields as primary key in the same sql table
                                        panic!("Error: multiple primary key fields found in struct {}", struct_ident);
                                    }
                                }

                            }
                        );

                        FieldInfo::new(_field_name.clone(), field_ty, is_primary_key, is_ignore)
                    }
                
                //collect the fields info in the struct    
                ).collect::<Vec<FieldInfo>>()
            
            },


            _=> panic!("Error: only struct can be used to create a table"),
        };

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

    //get the name of the struct
    pub fn get_struct_ident(&self) -> &str {
        &self.struct_ident
    }

    //get the name of the primary key field
    pub fn get_primary_key(&self) -> Option<&str> {
        self.primary_key.as_deref()
    }

    pub fn get_ignore_fields_name(&self) -> &Vec<String> {
        &self.ignore_fields_name
    }


}



#[cfg(test)]
mod tests {
    use super::*;
    use syn::parse_quote;

    #[test]
    fn test_field_processor() {
        //create a testing struct 
        let input: DeriveInput = parse_quote! {
            struct TestStruct {
                #[primary_key]
                id: i32,
                #[ignore]
                name: String,
                #[ignore]
                age: i32,
            }
        };


        let field_processor = FieldProcessor::new(&input);
        let fields = field_processor.get_fields();
        let struct_ident = field_processor.get_struct_ident();
        let primary_key = field_processor.get_primary_key();
        let ignore_fields_name = field_processor.get_ignore_fields_name();

        assert_eq!(fields.len(), 3);
        assert_eq!(struct_ident, "TestStruct");
        assert_eq!(primary_key, Some("id"));
        assert_eq!(ignore_fields_name.len(), 2);
        assert_eq!(ignore_fields_name[0], "name");
        assert_eq!(ignore_fields_name[1], "age");

    }

}