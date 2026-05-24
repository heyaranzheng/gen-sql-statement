// tests/test_macro.rs
use gen_sql_statement::SqlStatement;

#[derive(SqlStatement)]
struct User {
    #[primary_key]
    id: i32,
    name: String,
    age: i32,
}

#[derive(SqlStatement)]
struct Product {
    sku: String,
    price: f32,
    stock: i32,
}

#[test]
fn test_create_table_with_primary_key() {
    let user = User{
        id: 1,
        name: "aran".to_string(),
        age: 20,
    };
    let sql = user.create_table_sql();
    println!("User SQL: {}", sql);
    
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS"));
    assert!(sql.contains("user"));
    assert!(sql.contains("id"));
    assert!(sql.contains("PRIMARY KEY"));
    assert!(sql.contains("name"));
    assert!(sql.contains("age"));
}

#[test]
fn test_create_table_without_primary_key() {
    let product = Product{
        sku: "sku".to_string(),
        price: 100.0,
        stock: 100,
    };
    let sql = product.create_table_sql();
    println!("Product SQL: {}", sql);
    
    assert!(sql.contains("CREATE TABLE IF NOT EXISTS"));
    assert!(sql.contains("product"));
    assert!(sql.contains("id")); 
}

#[test]
fn test_get_field_names() {
    let user = User{
        id: 1,
        name: "aran".to_string(),
        age: 20,
    };
    let field_names = user.get_field_names();
    println!("Field names: {:?}", field_names);

    assert_eq!(field_names, vec!["id", "name", "age"]);

}
