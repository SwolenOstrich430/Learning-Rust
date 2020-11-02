use mysql::PooledConn;
use super::db_column::Column;
use super::column_types::ColumnType;
use std::collections::HashMap;


pub fn get_columns(query: &str, db_conn: &mut PooledConn) -> Vec<Column> {
    let columns: Vec<Column> = db_conn.prep_exec(query, ()).map(|result| { 
        result.map(|x| x.unwrap()).map(|row| {
            let (table_name, column_name, ordinal_position, is_nullable_string, data_type, column_key): 
                (String, String, i32, String, String, String) = mysql::from_row(row);
            let is_primary: bool = column_key.eq(&String::from("PRI"));
            let is_nullable: bool = is_nullable_string.eq(&String::from("NO"));
            let column_type: ColumnType = ColumnType::from_str(&data_type[..]).unwrap();

            Column::new(column_name, ordinal_position, is_nullable, column_type, is_primary, table_name)
        }).collect()
    }).unwrap();

    return columns;
}

pub fn get_table_map(columns: Vec<Column>) -> HashMap<String, Vec<Column>> {
    let mut table_map: HashMap<String, Vec<Column>> = HashMap::new();

    for col in columns {
        let table_name_copy: &String = &String::from(&col.table_name);

        if table_map.contains_key(&col.table_name) {
            let curr_columns: &mut Vec<Column> = table_map.get_mut(table_name_copy).unwrap();
            curr_columns.push(col);
        } else {
            let mut new_col_vec: Vec<Column> = Vec::new();
            new_col_vec.push(col);
            table_map.insert(String::from(table_name_copy), new_col_vec);
        }
    }

    return table_map;
}