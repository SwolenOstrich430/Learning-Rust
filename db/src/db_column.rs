use std::fmt;
use super::column_types::ColumnType;

#[derive(Debug)]
pub struct Column {
    pub column_name: String, 
    pub ordinal_position: i32, 
    pub is_nullable: bool, 
    pub data_type: ColumnType, 
    pub is_primary: bool, 
    pub table_name: String
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Column (\n  column_name: {}\n  position: {}\n  nullable: {}\n  type: {}\n  primary: {}\n)", 
            self.column_name, self.ordinal_position, self.is_nullable, self.data_type, self.is_primary)
    }
}

impl Column {
    pub fn new(column_name: String, ordinal_position: i32, is_nullable: bool, data_type: ColumnType, is_primary: bool, 
                    table_name: String) -> Column {
        Column {
            column_name: column_name, 
            ordinal_position: ordinal_position, 
            is_nullable: is_nullable, 
            data_type: data_type, 
            is_primary: is_primary, 
            table_name: table_name
        }
    }
}