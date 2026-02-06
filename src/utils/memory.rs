use std::collections::HashMap;
use std::sync::Arc;

/// 用于共享不可变数据的智能指针，避免不必要的克隆
pub type SharedData<T> = Arc<T>;

/// 通用数据存储优化工具
pub struct DataStore<T: Clone> {
    /// 使用 Arc 共享数据，避免克隆整个 Vec
    pub shared_data: Option<SharedData<Vec<T>>>,
    /// 缓存计算结果
    cache: HashMap<String, String>,
}

/// 专门用于 HashMap 数据的类型别名
pub type HashMapDataStore = DataStore<HashMap<String, String>>;

impl<T: Clone> DataStore<T> {
    pub fn new() -> Self {
        Self {
            shared_data: None,
            cache: HashMap::new(),
        }
    }

    /// 设置共享数据
    pub fn set_data(&mut self, data: Vec<T>) {
        self.shared_data = Some(Arc::new(data));
        self.cache.clear(); // 清除缓存
    }

    /// 获取共享数据的引用，避免克隆
    pub fn get_data(&self) -> Option<&SharedData<Vec<T>>> {
        self.shared_data.as_ref()
    }

    /// 获取数据条数，不需要克隆
    pub fn len(&self) -> usize {
        self.shared_data
            .as_ref()
            .map(|data| data.len())
            .unwrap_or(0)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 获取缓存的统计信息
    pub fn get_cached_quantity(&mut self) -> String {
        if let Some(cached) = self.cache.get("quantity") {
            cached.clone()
        } else {
            let qty = format!("总数量: {} PCS", self.len());
            self.cache.insert("quantity".to_string(), qty.clone());
            qty
        }
    }

    /// 清除数据
    pub fn clear(&mut self) {
        self.shared_data = None;
        self.cache.clear();
    }

    /// 清除缓存
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl<T: Clone> Default for DataStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 用于避免频繁字符串克隆的工具
pub struct StringInterner {
    interned: HashMap<String, String>,
}

impl StringInterner {
    pub fn new() -> Self {
        Self {
            interned: HashMap::new(),
        }
    }

    /// 获取字符串的引用，避免重复分配
    pub fn intern(&mut self, s: &str) -> &str {
        if !self.interned.contains_key(s) {
            self.interned.insert(s.to_string(), s.to_string());
        }
        self.interned.get(s).unwrap()
    }

    /// 清空内部缓存
    pub fn clear(&mut self) {
        self.interned.clear();
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}
