# This macro generates SQL statements from rust structs

**This is a tool for my private project, so it supports Sqlite first. Some features may not be available for PostgreSQL. I do  not recommend you using this  crate if you are using PostgreSQL as your database.**

* add this crate to your project:`cargo add gen-sql-sta`
* create your structure with `#[derive(SqlSta)]`,  you can use `#[primary_key]` to mark a field as your primary key.
  The field  marked `#[ignore]` will be ignored when you generate a sql statement or something realtive to it.
* **If no field is marked with `#[primary key]` ,  macro will attempt to use  a field  name of "id" as a primary key. If "id" was already used by another field, it will fall back to "autoincrement".**
  **If neither "id" or "autoincrement" is avaliable. The macro will panic immediately.**

```
// test struct, it works if you have used generics and where clause too.
#[derive(SqlStatement)]
struct Test {
  #[primary_key]
  id: u32,
  name: String,
  age: u32,
  #[ignore]
  score: u32,
}
```

* We can use the implemented funtions to get the SQL statements if we have a object test, like this: `test.create_table_sql()`

```
let test = Test {
    id: 1,
    name: "Tom".to_string(),
    age: 10,
    score: 100, 
};

//get the SQL statement
let sql_statement = test.create_table_sql();

println!("{:?}", sql_statement);
//The output will be like this:
"CREATE TABLE IF NOT EXISTS test ( id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, age INTEGER )

```

## Function Lists

`create_table_sql(&self)`:  the sql statement to create a table.      -> something like:`CREATE TABLE IF NOT EXISTS ......`

`get_field_names(&self)`:  get your names of your structure's field.    ->`Vec<&'static str>`

,e.g: `vec!["id", "name", "age"]`

* The table name will always be the struct indent with all lowercase.
  'test' is the table name of `struct Test{ /*... */}`, not 'Test'
* If you don't use

## The Type Transformation From Rust to Sqlite

```
//Byte UuidBytes Hash, is a reserved type name, you can use the name as your individual type.
"i32" | "u32" | "i64" | "u64" => "INTEGER",
 "f32" | "f64"                 => "REAL",
 "String" | "str"              => "TEXT",
 "bool"                        => "INTEGER",       // SQLite map BOOLEAN to INTEGER, 0 for false, 1 for true
 "NaiveDate"                   => "TEXT",          // SQLite have no DATE type, use TEXT
 "NaiveDateTime"               => "TEXT",          // SQLite have no TIMESTAMP type, use TEXT
 
 "UuidBytes"                   => "BLOB",          // SQLite use BLOB to store binary data, reserved rust type name 
 "Hash"                        => "BLOB",          // SQLite use BLOB to store binary data, reserved rust type name
 "Byte"                        => "BLOB",          // SQLite use BLOB to store binary data, reserved rust type name 

//others not in the list.
 _                             => "TEXT", 
```

## The Type Transformation From Rust to PostgreSql

```
 //Byte UuidBytes Hash, is a reserved type name, you can use the name as your individual type. 
 "i32" | "u32" | "i64" | "u64" => "INTEGER",
 "f32" | "f64"                  => "REAL",
 "String" | "str"               => "TEXT",
 "bool"                         => "BOOLEAN",
 "NaiveDate"                    => "DATE",                // chrono::NaiveDate
 "NaiveDateTime"                => "TIMESTAMP",           // chrono::NaiveDateTime
 "UuidBytes"                    => "BYTEA",               // reserves a name for binary data, reserved rust type name
 "Hash"                         => "BYTEA",               // reserves a name for binary data, reserved rust type name
 "Byte"                         => "BYTEA",               // reserves a name for binary data, reserved rust type name
 
  //others not in the list.
   _                            => "TEXT" ,
```
