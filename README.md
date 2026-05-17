# This pre-macro is used to create a specific sql when we have a structure.

for example:

```
#[derive(TableCreator)]
struct Mystruct <'somelifetime, SomeType>{
  #[table_ignore]
  a: i32,
  b: String,
  c: &'Somelifetime SomeType
  #[primary_key]
  d: String,
}
printfln!("{}", Mystruct::<'somelifetime, SomeType>::sql_create();
printfln!("{}", Mystruct::<'somelifetime, SomeType>::sql_insert();
```

the output is : 

* CREATE TABLE IF NOT EXISTS  mystruct (b TEXT , c SomeMatchedSqlType, d  TEXT PRIMARY KEY )
* INSERT INTO TABLE mystruct (b, c, d) VALUES ( \$1, \$2 , \$3 \)

Note:   

The elements , with #[table_ignore], will be ignored, and will be dealed as a key if #[primary_key].

And the table name always the lower case. If the struct name is "Abc", then "abc" will be the nameof the table. 

You can also use the default "SERIAL PRIMARY KEY" named "id",  without marking the field with atribute [primary_key].

For example:

```
#[derive(TableCreator)]
struct Mystruct <'somelifetime, SomeType>{
  #[table_ignore]
  a: i32,
  b: String,
  c: &'somelifetime, SomeType,
  //#[primary_key]
  d: String,
}
printfln!("{}", Mystruct::<'somelifetime, SomeType>::sql_create();
printfln!("{}", Mystruct::<'somelifetime, SomeType>::sql_insert();
```

the output is:

* CREATE TABLE IF NOT EXISTS  mystruct (b TEXT , c SomeMatchedSqlType, d  TEXT, id SERIAL PRIMARY KEY)
* INSERT INTO TABLE mystruct (b, c, d) VALUES ( \$1, \$2, \$3 \)

This crate also provided a way to generate the arguments to insert a row for ***postgres** *if you use ***sqlx**.*

Add this in your** *Cargo.toml*** firstly:

```
[features]
sqlx-support = ["table-creator-macro2/sqlx-support"]
default = ["sqlx-support"]
```

Then you can create args for postgres like this :

```
let mystruct = MyStruct { .....}
let args = mystruct.to_sql_value();
```

This args will ignore the element which marked with #[table_ignore].

It just support some sql types:

*  i32| u32 | i64 | u64 => INTEGER
* f32 => REAL
* f64 => REAL
* String | str => TEXT
* bool => BOOLEAN
* NaiveDate => DATE
* NaiveDateTime => TIMESTAMP
* UuidByte=>BYTEA              //UuidByte can't be used, unless you have defined one by your own.
* Hash=>BYTEA                    //Hash can't be used, unless you have defined one by your own.

```

```
