use std::str::FromStr;
use std::fmt;
use std::fmt::Display;
#[derive(Debug, Clone)]
pub struct InvalidColumnTypeError;

impl fmt::Display for InvalidColumnTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid column type")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ColumnType {
    CHAR, 
    VARCHAR, 
    TINYTEXT, 
    TEXT, 
    BLOB, 
    MEDIUMTEXT, 
    MEDIUMBLOB, 
    LONGTEXT, 
    LONGBLOB, 
    TINYINT, 
    SMALLINT, 
    MEDIUMINT, 
    INT, 
    BIGINT, 
    FLOAT, 
    DOUBLE, 
    DECIMAL, 
    DATE, 
    DATETIME, 
    TIMESTAMP, 
    TIME, 
    SET, 
    BOOLEAN, 
    ENUM
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value())
    }
}

impl ColumnType {
    pub fn value(&self) -> String {
        match self {
            ColumnType::CHAR => String::from("char"), 
            ColumnType::VARCHAR => String::from("varchar"), 
            ColumnType::TINYTEXT => String::from("tinytext"), 
            ColumnType::TEXT => String::from("text"), 
            ColumnType::BLOB => String::from("blob"), 
            ColumnType::MEDIUMTEXT => String::from("mediumtext"), 
            ColumnType::MEDIUMBLOB => String::from("mediumblob"), 
            ColumnType::LONGTEXT => String::from("longtext"), 
            ColumnType::LONGBLOB => String::from("longblob"), 
            ColumnType::TINYINT => String::from("tinyint"), 
            ColumnType::SMALLINT => String::from("smallint"), 
            ColumnType::MEDIUMINT => String::from("mediumint"), 
            ColumnType::INT => String::from("int"), 
            ColumnType::BIGINT => String::from("bigint"), 
            ColumnType::FLOAT => String::from("float"), 
            ColumnType::DOUBLE => String::from("double"), 
            ColumnType::DECIMAL => String::from("decimal"), 
            ColumnType::DATE => String::from("date"), 
            ColumnType::DATETIME => String::from("datetime"), 
            ColumnType::TIMESTAMP => String::from("timestamp"), 
            ColumnType::TIME => String::from("time"), 
            ColumnType::SET => String::from("set"), 
            ColumnType::BOOLEAN => String::from("boolean"), 
            ColumnType::ENUM => String::from("enum")
        }
    }

    pub fn from_str(s: &str) -> Result<Self, InvalidColumnTypeError> {
        match s {
            "char" => Ok(ColumnType::CHAR), 
            "varchar" => Ok(ColumnType::VARCHAR), 
            "tinytext" => Ok(ColumnType::TINYTEXT), 
            "text" => Ok(ColumnType::TEXT), 
            "blob" => Ok(ColumnType::BLOB), 
            "mediumtext" => Ok(ColumnType::MEDIUMTEXT), 
            "mediumblob" => Ok(ColumnType::MEDIUMBLOB), 
            "longtext"  => Ok(ColumnType::LONGTEXT),
            "longblob" => Ok(ColumnType::LONGBLOB), 
            "tinyint" => Ok(ColumnType::TINYINT), 
            "smallint" => Ok(ColumnType::SMALLINT), 
            "mediumint" => Ok(ColumnType::MEDIUMINT), 
            "int" => Ok(ColumnType::INT), 
            "bigint" => Ok(ColumnType::BIGINT), 
            "float" => Ok(ColumnType::FLOAT), 
            "double" => Ok(ColumnType::DOUBLE), 
            "decimal" => Ok(ColumnType::DECIMAL), 
            "date" => Ok(ColumnType::DATE), 
            "datetime" => Ok(ColumnType::DATETIME), 
            "timestamp" => Ok(ColumnType::TIMESTAMP), 
            "time" => Ok(ColumnType::TIME), 
            "set" => Ok(ColumnType::SET), 
            "boolean" => Ok(ColumnType::BOOLEAN),
            "enum" => Ok(ColumnType::ENUM),
            _ => Err(InvalidColumnTypeError)
        }
    }
}