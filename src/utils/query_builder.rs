//! SQL 查询构建器
//! 提供安全的参数化查询功能，优化版本减少内存分配

use std::fmt::Write;

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
        // 预分配容量避免重复扩容
        Self {
            sql: String::with_capacity(base_sql.len() + 256),
            params: Vec::with_capacity(16),
            param_counter: 1,
            where_added,
        }
    }

    /// 添加等于条件
    pub fn add_equals(mut self, field: &str, value: &str) -> Self {
        if !value.is_empty() {
            self.add_where_or_and();
            // 使用 write! 替代 format! 减少临时分配
            let _ = write!(self.sql, " {} = @P{}", field, self.param_counter);
            self.params.push(value.to_string());
            self.param_counter += 1;
        }
        self
    }

    /// 添加 IN 条件
    pub fn add_in(mut self, field: &str, values: &[String]) -> Self {
        if !values.is_empty() {
            self.add_where_or_and();
            // 手动构建 placeholders 字符串，避免多次分配
            let _ = write!(self.sql, " {} IN (", field);
            for (i, _) in values.iter().enumerate() {
                if i > 0 {
                    self.sql.push_str(", ");
                }
                let _ = write!(self.sql, "@P{}", self.param_counter + i);
            }
            self.sql.push(')');

            self.params.extend(values.iter().cloned());
            self.param_counter += values.len();
        }
        self
    }

    /// 添加 BETWEEN 条件
    pub fn add_between(mut self, field: &str, start: &str, end: &str) -> Self {
        if !start.is_empty() && !end.is_empty() {
            self.add_where_or_and();
            let _ = write!(
                self.sql,
                " {} BETWEEN @P{} AND @P{}",
                field,
                self.param_counter,
                self.param_counter + 1
            );
            self.params.push(start.to_string());
            self.params.push(end.to_string());
            self.param_counter += 2;
        }
        self
    }

    /// 添加原始条件
    pub fn add_raw(mut self, condition: &str) -> Self {
        self.add_where_or_and();
        self.sql.push(' ');
        self.sql.push_str(condition);
        self
    }

    /// 添加 ORDER BY
    pub fn add_order_by(mut self, fields: &[&str]) -> Self {
        if !fields.is_empty() {
            self.sql.push_str(" ORDER BY ");
            for (i, field) in fields.iter().enumerate() {
                if i > 0 {
                    self.sql.push_str(", ");
                }
                self.sql.push_str(field);
            }
        }
        self
    }

    /// 添加分页 (SQL Server 2012+)
    pub fn add_pagination(mut self, offset: usize, limit: usize) -> Self {
        let _ = write!(
            self.sql,
            " OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            offset, limit
        );
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
