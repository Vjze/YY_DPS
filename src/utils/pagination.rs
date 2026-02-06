use std::collections::HashMap;

/// 分页查询参数
#[derive(Debug, Clone)]
pub struct PaginationParams {
    pub page: usize,
    pub page_size: usize,
    pub offset: usize,
}

impl PaginationParams {
    pub fn new(page: usize, page_size: usize) -> Self {
        Self {
            page,
            page_size,
            offset: (page - 1) * page_size,
        }
    }

    pub fn limit(&self) -> usize {
        self.page_size
    }

    pub fn offset_sql(&self) -> String {
        format!(
            "OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            self.offset, self.page_size
        )
    }
}

/// 分页结果
#[derive(Debug, Clone)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total_count: usize,
    pub page: usize,
    pub page_size: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_prev: bool,
}

impl<T> PaginatedResult<T> {
    pub fn new(data: Vec<T>, total_count: usize, params: &PaginationParams) -> Self {
        let total_pages = (total_count + params.page_size - 1) / params.page_size;
        Self {
            data,
            total_count,
            page: params.page,
            page_size: params.page_size,
            total_pages,
            has_next: params.page < total_pages,
            has_prev: params.page > 1,
        }
    }

    pub fn empty(params: &PaginationParams) -> Self {
        Self::new(Vec::new(), 0, params)
    }
}

/// 为 HashMap 数据添加分页支持的类型别名
pub type PaginatedQueryResult = PaginatedResult<HashMap<String, String>>;

/// 构建带分页的 SQL 查询
pub fn build_paginated_query(base_query: &str, params: &PaginationParams) -> String {
    if params.page == 1 && params.page_size == usize::MAX {
        // 第一页且无限制时，不需要分页
        base_query.to_string()
    } else {
        // SQL Server 分页语法
        format!(
            "{} ORDER BY testtime DESC OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            base_query, params.offset, params.page_size
        )
    }
}

/// 获取总数的 SQL 查询
pub fn build_count_query(base_query: &str) -> String {
    format!(
        "SELECT COUNT(*) as total_count FROM ({}) AS count_query",
        base_query
    )
}
