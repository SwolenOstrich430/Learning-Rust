use std::fs::File;
use std::path::Path;
use std::collections::HashMap;
use super::db_column::Column;
use std::io::prelude::*;
use super::graphql_types::GraphqlType;
use super::column_types::ColumnType;


pub fn write_schema(output_path: &str, scehma_content: String) {
    let path = Path::new(output_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(scehma_content.as_bytes()) {
        Ok(_) => println!("file written successfully"), 
        Err(why) => println!("couldn't write file: {}", why)
    };
}

pub fn get_schema_content(table_map: &HashMap<String, Vec<Column>>) -> String {
    let mut schema_content: String = String::from("");

    for (table_name, columns) in table_map {
        schema_content.push_str("type ");
        schema_content.push_str(&table_name[..]);
        schema_content.push_str("\n\n");
        schema_content.push_str(&table_name[..]);
        schema_content.push_str(" {\n");

        for col in columns {
            let field: String = get_graphql_field(col);
            schema_content.push_str(&field[..]);
        }

        schema_content.push_str("}\n\n");
    }

    
    return schema_content;
}

fn get_graphql_field(column: &Column) -> String {
    let column_type: ColumnType = column.data_type;
    let graphql_type: GraphqlType = GraphqlType::from_db_type(column_type, column.is_primary, None);

    let is_nullable_string: String = match column.is_nullable {
        true => String::from(""), 
        false => String::from("!")
    };

    let graphql_field: String = format!("\t{}: {}{}\n", column.column_name, graphql_type.value(), is_nullable_string);
    return graphql_field;
}