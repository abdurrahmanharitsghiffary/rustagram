pub struct InsertTemplate {
    pub query: String,
}

impl InsertTemplate {
    pub fn new(table_name: &str, insertable_columns: &[&str]) -> InsertTemplate {
        let query_template = format!("INSERT INTO {} (", table_name);

        let columns = insertable_columns.join(", ");
        let placeholders = insertable_columns
            .iter()
            .map(|_| "?")
            .collect::<Vec<&str>>()
            .join(", ");

        let query = format!("{} {}) VALUES ({})", query_template, columns, placeholders);

        InsertTemplate { query }
    }
}

pub struct UpdateTemplate {
    pub query: String,
}

impl UpdateTemplate {
    pub fn new(id_column: &str, table_name: &str, update_columns: &[&str]) -> UpdateTemplate {
        let query_template = format!("UPDATE {} SET ", table_name);

        let assignments = update_columns
            .iter()
            .map(|column| format!("{} = ?", column))
            .collect::<Vec<String>>()
            .join(", ");

        let query = format!("{} {} WHERE {} = ?", query_template, assignments, id_column);

        UpdateTemplate { query }
    }
}
