// src/query_builder.rs

use super::traits::dao::{
    DaoFilter, LikeOptions, LikeValueOptions, OrderBy, OrderOptions, PaginationOptions,
    WhereOptions,
};

/// A parameterized SQL query builder for PostgreSQL.
#[derive(Debug)]
pub struct QueryBuilder {
    pub query: String,
    pub params: Vec<String>,
}

impl QueryBuilder {
    /// Builds a parameterized SELECT query for PostgreSQL.
    ///
    /// - `table`: the table name.
    /// - `columns`: list of columns to select (empty means "*").
    /// - `dao_filter`: optional filter conditions.
    pub fn build_select(table: &str, columns: &[&str], dao_filter: Option<&DaoFilter>) -> Self {
        let columns_str = if columns.is_empty() {
            "*".to_string()
        } else {
            columns.join(", ")
        };

        let mut query = format!("SELECT {} FROM {}", columns_str, table);
        let mut conditions = Vec::new();
        let mut params: Vec<String> = Vec::new();

        if let Some(filter) = dao_filter {
            // Process WHERE conditions.
            for (col, op, value) in &filter.where_condition {
                match op {
                    WhereOptions::EQ => {
                        conditions.push(format!(r#""{}" = ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::NotEQ => {
                        conditions.push(format!(r#""{}" != ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::GT => {
                        conditions.push(format!(r#""{}" > ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::GTE => {
                        conditions.push(format!(r#""{}" >= ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::LT => {
                        conditions.push(format!(r#""{}" < ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::LTE => {
                        conditions.push(format!(r#""{}" <= ${}"#, col, params.len() + 1));
                        params.push(value.clone());
                    }
                    WhereOptions::In => {
                        // Assume `value` is a comma-separated list, e.g., "val1,val2,val3"
                        let items: Vec<&str> = value
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect();
                        if !items.is_empty() {
                            let mut placeholders = Vec::new();
                            for item in items.iter() {
                                placeholders.push(format!("${}", params.len() + 1));
                                params.push(item.to_string());
                            }
                            conditions.push(format!(
                                r#""{}" IN ({})"#,
                                col,
                                placeholders.join(", ")
                            ));
                        }
                    }
                    WhereOptions::NotIn => {
                        let items: Vec<&str> = value
                            .split(',')
                            .map(|s| s.trim())
                            .filter(|s| !s.is_empty())
                            .collect();
                        if !items.is_empty() {
                            let mut placeholders = Vec::new();
                            for item in items.iter() {
                                placeholders.push(format!("${}", params.len() + 1));
                                params.push(item.to_string());
                            }
                            conditions.push(format!(
                                r#""{}" NOT IN ({})"#,
                                col,
                                placeholders.join(", ")
                            ));
                        }
                    }
                    WhereOptions::Between => {
                        // Assume `value` contains two comma-separated values.
                        let items: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
                        if items.len() == 2 {
                            let placeholder1 = format!("${}", params.len() + 1);
                            params.push(items[0].to_string());
                            let placeholder2 = format!("${}", params.len() + 1);
                            params.push(items[1].to_string());
                            conditions.push(format!(
                                r#""{}" BETWEEN {} AND {}"#,
                                col, placeholder1, placeholder2
                            ));
                        }
                    }
                    WhereOptions::NotBetween => {
                        let items: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
                        if items.len() == 2 {
                            let placeholder1 = format!("${}", params.len() + 1);
                            params.push(items[0].to_string());
                            let placeholder2 = format!("${}", params.len() + 1);
                            params.push(items[1].to_string());
                            conditions.push(format!(
                                r#""{}" NOT BETWEEN {} AND {}"#,
                                col, placeholder1, placeholder2
                            ));
                        }
                    }
                }
            }

            // Process LIKE conditions.
            for (col, like_op, value) in &filter.like_condition {
                let like_value = Self::build_like_value(like_op, value);
                let placeholder = format!("${}", params.len() + 1);
                let cond = match like_op {
                    LikeOptions::Like(_) => format!(r#""{}" LIKE {}"#, col, placeholder),
                    LikeOptions::NotLike(_) => format!(r#""{}" NOT LIKE {}"#, col, placeholder),
                    LikeOptions::ILike(_) => format!(r#""{}" ILIKE {}"#, col, placeholder),
                    LikeOptions::INotLike(_) => format!(r#""{}" NOT ILIKE {}"#, col, placeholder),
                    LikeOptions::Fulltext => {
                        // For full-text search, using Postgres functions.
                        format!(
                            r#"to_tsvector("{}") @@ plainto_tsquery({})"#,
                            col, placeholder
                        )
                    }
                };
                conditions.push(cond);
                params.push(like_value);
            }

            // Append WHERE clause if any conditions exist.
            if !conditions.is_empty() {
                query.push_str(" WHERE ");
                query.push_str(&conditions.join(" AND "));
            }

            // Add ORDER BY clause.
            if let Some(OrderOptions {
                order_column,
                order_by,
            }) = &filter.order_options
            {
                let order = match order_by {
                    OrderBy::ASC => "ASC",
                    OrderBy::DESC => "DESC",
                };
                query.push_str(&format!(r#" ORDER BY "{}" {}"#, order_column, order));
            }

            // Add pagination (LIMIT and OFFSET).
            if let Some(PaginationOptions { page, per_page }) = &filter.pagination {
                let limit = per_page;
                let offset = (page - 1) * per_page;
                query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));
            }
        }

        Self { query, params }
    }

    /// Build the LIKE pattern based on the specified LikeOptions.
    fn build_like_value(like_op: &LikeOptions, value: &str) -> String {
        match like_op {
            // For these variants, we apply a wildcard pattern.
            LikeOptions::Like(kind)
            | LikeOptions::NotLike(kind)
            | LikeOptions::ILike(kind)
            | LikeOptions::INotLike(kind) => match kind {
                LikeValueOptions::StartWiths => format!("{}%", value),
                LikeValueOptions::EndWiths => format!("%{}", value),
                LikeValueOptions::Contains => format!("%{}%", value),
            },
            // For full-text, we simply pass the value to the tsquery function.
            LikeOptions::Fulltext => value.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::traits::dao::{
        DaoFilter, LikeOptions, LikeValueOptions, OrderBy, OrderOptions, PaginationOptions,
        WhereOptions,
    };
    use super::*;

    #[test]
    fn test_select_no_filter() {
        let qb = QueryBuilder::build_select("users", &["id", "name"], None);

        println!("{:?}", qb);

        assert_eq!(qb.query, "SELECT id, name FROM users");
        assert!(qb.params.is_empty());
    }

    #[test]
    fn test_select_with_where_eq() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![("name".to_string(), WhereOptions::EQ, "Alice".to_string())],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &["id", "name"], Some(&filter));

        println!("{:?}", qb);

        assert_eq!(qb.query, "SELECT id, name FROM users WHERE \"name\" = $1");
        assert_eq!(qb.params, vec!["Alice".to_string()]);
    }

    #[test]
    fn test_select_with_where_gt() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![("age".to_string(), WhereOptions::GT, "18".to_string())],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        // Empty columns means selecting all (*)
        assert_eq!(qb.query, "SELECT * FROM users WHERE \"age\" > $1");
        assert_eq!(qb.params, vec!["18".to_string()]);
    }

    #[test]
    fn test_select_with_where_in() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![(
                "role".to_string(),
                WhereOptions::In,
                "admin,guest".to_string(),
            )],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        assert_eq!(qb.query, "SELECT * FROM users WHERE \"role\" IN ($1, $2)");
        assert_eq!(qb.params, vec!["admin".to_string(), "guest".to_string()]);
    }

    #[test]
    fn test_select_with_where_not_in() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![(
                "role".to_string(),
                WhereOptions::NotIn,
                "admin,guest".to_string(),
            )],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        assert_eq!(
            qb.query,
            "SELECT * FROM users WHERE \"role\" NOT IN ($1, $2)"
        );
        assert_eq!(qb.params, vec!["admin".to_string(), "guest".to_string()]);
    }

    #[test]
    fn test_select_with_where_between() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![(
                "created_at".to_string(),
                WhereOptions::Between,
                "2020-01-01,2020-12-31".to_string(),
            )],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        assert_eq!(
            qb.query,
            "SELECT * FROM users WHERE \"created_at\" BETWEEN $1 AND $2"
        );
        assert_eq!(
            qb.params,
            vec!["2020-01-01".to_string(), "2020-12-31".to_string()]
        );
    }

    #[test]
    fn test_select_with_where_not_between() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![(
                "created_at".to_string(),
                WhereOptions::NotBetween,
                "2020-01-01,2020-12-31".to_string(),
            )],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        assert_eq!(
            qb.query,
            "SELECT * FROM users WHERE \"created_at\" NOT BETWEEN $1 AND $2"
        );
        assert_eq!(
            qb.params,
            vec!["2020-01-01".to_string(), "2020-12-31".to_string()]
        );
    }

    #[test]
    fn test_select_with_like_like() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![],
            like_condition: vec![(
                "name".to_string(),
                LikeOptions::Like(LikeValueOptions::Contains),
                "John".to_string(),
            )],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT * FROM users WHERE "name" LIKE $1, parameter: "%John%"
        assert_eq!(qb.query, "SELECT * FROM users WHERE \"name\" LIKE $1");
        assert_eq!(qb.params, vec!["%John%".to_string()]);
    }

    #[test]
    fn test_select_with_like_ilike() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![],
            like_condition: vec![(
                "name".to_string(),
                LikeOptions::ILike(LikeValueOptions::StartWiths),
                "John".to_string(),
            )],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT * FROM users WHERE "name" ILIKE $1, parameter: "John%"
        assert_eq!(qb.query, "SELECT * FROM users WHERE \"name\" ILIKE $1");
        assert_eq!(qb.params, vec!["John%".to_string()]);
    }

    #[test]
    fn test_select_with_like_not_like() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![],
            like_condition: vec![(
                "description".to_string(),
                LikeOptions::NotLike(LikeValueOptions::EndWiths),
                "test".to_string(),
            )],
        };
        let qb = QueryBuilder::build_select("products", &[], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT * FROM products WHERE "description" NOT LIKE $1, parameter: "%test"
        assert_eq!(
            qb.query,
            "SELECT * FROM products WHERE \"description\" NOT LIKE $1"
        );
        assert_eq!(qb.params, vec!["%test".to_string()]);
    }

    #[test]
    fn test_select_with_like_fulltext() {
        let filter = DaoFilter {
            pagination: None,
            order_options: None,
            where_condition: vec![],
            like_condition: vec![(
                "content".to_string(),
                LikeOptions::Fulltext,
                "search_term".to_string(),
            )],
        };
        let qb = QueryBuilder::build_select("articles", &[], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT * FROM articles WHERE to_tsvector("content") @@ plainto_tsquery($1)
        assert_eq!(
            qb.query,
            r#"SELECT * FROM articles WHERE to_tsvector("content") @@ plainto_tsquery($1)"#
        );
        assert_eq!(qb.params, vec!["search_term".to_string()]);
    }

    #[test]
    fn test_select_with_order() {
        let filter = DaoFilter {
            pagination: None,
            order_options: Some(OrderOptions {
                order_column: "name".to_string(),
                order_by: OrderBy::DESC,
            }),
            where_condition: vec![],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &["id", "name"], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT id, name FROM users ORDER BY "name" DESC
        assert_eq!(
            qb.query,
            r#"SELECT id, name FROM users ORDER BY "name" DESC"#
        );
        assert!(qb.params.is_empty());
    }

    #[test]
    fn test_select_with_pagination() {
        let filter = DaoFilter {
            pagination: Some(PaginationOptions {
                page: 2,
                per_page: 10,
            }),
            order_options: None,
            where_condition: vec![],
            like_condition: vec![],
        };
        let qb = QueryBuilder::build_select("users", &[], Some(&filter));

        println!("{:?}", qb);

        // Expected: SELECT * FROM users LIMIT 10 OFFSET 10
        assert_eq!(qb.query, "SELECT * FROM users LIMIT 10 OFFSET 10");
        assert!(qb.params.is_empty());
    }

    #[test]
    fn test_select_combined_filters() {
        let filter = DaoFilter {
            pagination: Some(PaginationOptions {
                page: 3,
                per_page: 5,
            }),
            order_options: Some(OrderOptions {
                order_column: "created_at".to_string(),
                order_by: OrderBy::ASC,
            }),
            where_condition: vec![
                ("age".to_string(), WhereOptions::GTE, "18".to_string()),
                ("status".to_string(), WhereOptions::EQ, "active".to_string()),
            ],
            like_condition: vec![(
                "name".to_string(),
                LikeOptions::ILike(LikeValueOptions::Contains),
                "Doe".to_string(),
            )],
        };
        let qb = QueryBuilder::build_select("users", &["id", "name", "age"], Some(&filter));

        println!("{:?}", qb);

        // Check that the query includes each clause.
        assert!(qb.query.contains("SELECT id, name, age FROM users"));
        assert!(qb.query.contains("WHERE"));
        assert!(qb.query.contains("\"age\" >= $1"));
        // Expected parameter order: ["18", "active", "%Doe%"]
        assert_eq!(
            qb.params,
            vec!["18".to_string(), "active".to_string(), "%Doe%".to_string()]
        );
        assert!(qb.query.contains("ORDER BY \"created_at\" ASC"));
        // Pagination: page 3, per_page 5 gives LIMIT 5 OFFSET 10.
        assert!(qb.query.contains("LIMIT 5 OFFSET 10"));
    }
}
