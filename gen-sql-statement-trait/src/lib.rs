enum Error{
    InvalidInput(String),
    SqlError(String),      
}


pub trait SqlStatementTrait {
    fn create_table_sql(&self) -> String;
    fn get_feild_names(&self) -> Vec<&'static str>;
   
}