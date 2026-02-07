//! SQL 查询构建器
//! 提供安全的参数化查询构建功能

/// SQL 查询构建器
#[derive(Clone)]
pub struct QueryBuilder {
    sql: String,
    params: Vec<String>,
    param_counter: usize,
    where_added: bool,
}

impl QueryBuilder {
    /// 创建新的查询构建器
    pub fn new(base_sql: &str) -> Self {
        let where_added = base_sql.to_uppercase().contains("WHERE");
        Self {
            sql: base_sql.to_string(),
            params: vec![],
            param_counter: 1,
            where_added,
        }
    }

    /// 添加等于条件
    pub fn add_equals(mut self, field: &str, value: &str) -> Self {
        if !value.is_empty() {
            self.add_where_or_and();
            self.sql
                .push_str(&format!(" {} = @P{}", field, self.param_counter));
            self.params.push(value.to_string());
            self.param_counter += 1;
        }
        self
    }

    /// 添加 IN 条件
    pub fn add_in(mut self, field: &str, values: &[String]) -> Self {
        if !values.is_empty() {
            self.add_where_or_and();
            let placeholders: Vec<String> = (0..values.len())
                .map(|i| format!("@P{}", self.param_counter + i))
                .collect();
            self.sql
                .push_str(&format!(" {} IN ({})", field, placeholders.join(", ")));
            self.params.extend(values.iter().cloned());
            self.param_counter += values.len();
        }
        self
    }

    /// 添加 BETWEEN 条件
    pub fn add_between(mut self, field: &str, start: &str, end: &str) -> Self {
        if !start.is_empty() && !end.is_empty() {
            self.add_where_or_and();
            self.sql.push_str(&format!(
                " {} BETWEEN @P{} AND @P{}",
                field,
                self.param_counter,
                self.param_counter + 1
            ));
            self.params.push(start.to_string());
            self.params.push(end.to_string());
            self.param_counter += 2;
        }
        self
    }

    /// 添加原始条件
    pub fn add_raw(mut self, condition: &str) -> Self {
        self.add_where_or_and();
        self.sql.push_str(&format!(" {}", condition));
        self
    }

    /// 添加 ORDER BY
    pub fn add_order_by(mut self, fields: &[&str]) -> Self {
        if !fields.is_empty() {
            self.sql
                .push_str(&format!(" ORDER BY {}", fields.join(", ")));
        }
        self
    }

    /// 添加分页 (SQL Server 2012+)
    pub fn add_pagination(mut self, offset: usize, limit: usize) -> Self {
        self.sql.push_str(&format!(
            " OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            offset, limit
        ));
        self
    }

    /// 添加 WHERE 或 AND
    fn add_where_or_and(&mut self) {
        if !self.where_added {
            self.sql.push_str(" WHERE");
            self.where_added = true;
        } else {
            self.sql.push_str(" AND");
        }
    }

    /// 构建查询
    pub fn build(self) -> (String, Vec<String>) {
        (self.sql, self.params)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        let (sql, params) = QueryBuilder::new("SELECT * FROM table1")
            .add_raw("deleted = 0")
            .add_equals("carton_no", "CTN001")
            .add_equals("pn", "PN001")
            .add_between("create_time", "2024-01-01", "2024-12-31")
            .add_order_by(&["create_time DESC"])
            .build();

        assert!(sql.contains("WHERE"));
        assert!(sql.contains("deleted = 0"));
        assert!(sql.contains("carton_no = @P1"));
        assert!(sql.contains("pn = @P2"));
        assert!(sql.contains("create_time BETWEEN @P3 AND @P4"));
        assert_eq!(params.len(), 4);
    }
}
