use super::column_types::ColumnType;

pub enum GraphqlType {
    ID, 
    STR, 
    INT, 
    FLOAT, 
    BOOLEAN, 
    ENUM(Vec<String>),
    SCALAR(String)
}

impl GraphqlType {
    pub fn value(&self) -> String {
        match self {
            GraphqlType::ID => String::from("ID"),
            GraphqlType::STR => String::from("String"),
            GraphqlType::INT => String::from("Int"),
            GraphqlType::FLOAT => String::from("Float"), 
            GraphqlType::BOOLEAN => String::from("Boolean"), 
            GraphqlType::ENUM(vals) => self.get_enum_def(vals),
            GraphqlType::SCALAR(val) => String::from(val)
        }
    }

    pub fn from_db_type(db_type: ColumnType, is_primary: bool, enum_options: Option<Vec<String>>) -> GraphqlType {
        if is_primary {
            return GraphqlType::ID;
        }

        match db_type {
            ColumnType::VARCHAR => GraphqlType::STR,
            ColumnType::CHAR => GraphqlType::STR,
            ColumnType::TEXT => GraphqlType::STR, 
            ColumnType::TINYTEXT => GraphqlType::STR, 
            ColumnType::MEDIUMTEXT => GraphqlType::STR, 
            ColumnType::LONGTEXT => GraphqlType::STR, 
            ColumnType::FLOAT => GraphqlType::FLOAT, 
            ColumnType::DECIMAL => GraphqlType::FLOAT, 
            ColumnType::DOUBLE => GraphqlType::FLOAT, 
            ColumnType::INT => GraphqlType::INT, 
            ColumnType::TINYINT => GraphqlType::INT, 
            ColumnType::MEDIUMINT => GraphqlType::INT,
            ColumnType::ENUM => GraphqlType::ENUM(enum_options.unwrap()),
            ColumnType::BOOLEAN => GraphqlType::BOOLEAN,
            ColumnType::TINYINT => GraphqlType::BOOLEAN,
            _ => GraphqlType::SCALAR(db_type.value())
        }
    }

    pub fn get_enum_def(&self, vals: &Vec<String>) -> String {
        let mut enum_vals = String::from("");

        for val in vals {
            enum_vals.push_str("\t");
            enum_vals.push_str(&val[..]);
            enum_vals.push_str("\n");
        }

        return enum_vals;
    }
}