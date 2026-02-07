//! 缓存工具模块
//! 提供全局缓存功能

use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::RwLock;
use std::time::{Duration, Instant};

/// 带过期时间的缓存项
pub struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
    ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            timestamp: Instant::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        self.timestamp.elapsed() > self.ttl
    }

    pub fn get(&self) -> &T {
        &self.data
    }
}

/// 通用缓存结构
pub struct TimedCache<T> {
    entry: RwLock<Option<CacheEntry<T>>>,
}

impl<T: Clone> TimedCache<T> {
    pub const fn new() -> Self {
        Self {
            entry: RwLock::new(None),
        }
    }

    /// 获取缓存值，如果过期或不存在则调用工厂函数
    pub fn get_or_refresh<F>(&self, factory: F, ttl: Duration) -> Option<T>
    where
        F: FnOnce() -> Option<T>,
    {
        // 先尝试读取缓存
        {
            let read_guard = self.entry.read().ok()?;
            if let Some(entry) = read_guard.as_ref() {
                if !entry.is_expired() {
                    return Some(entry.get().clone());
                }
            }
        }

        // 缓存过期或不存在，刷新缓存
        let mut write_guard = self.entry.write().ok()?;

        // 双重检查，防止其他线程已经刷新
        if let Some(entry) = write_guard.as_ref() {
            if !entry.is_expired() {
                return Some(entry.get().clone());
            }
        }

        // 调用工厂函数获取新数据
        if let Some(data) = factory() {
            *write_guard = Some(CacheEntry::new(data.clone(), ttl));
            Some(data)
        } else {
            None
        }
    }

    /// 清除缓存
    pub fn clear(&self) {
        if let Ok(mut guard) = self.entry.write() {
            *guard = None;
        }
    }
}

// 全局表列表缓存（5分钟过期）
static TABLES_CACHE: TimedCache<Vec<String>> = TimedCache::new();

/// 获取缓存的表列表
pub fn get_cached_tables<F>(factory: F) -> Option<Vec<String>>
where
    F: FnOnce() -> Option<Vec<String>>,
{
    TABLES_CACHE.get_or_refresh(factory, Duration::from_secs(300))
}

/// 清除表列表缓存
pub fn clear_tables_cache() {
    TABLES_CACHE.clear();
}

/// 全局查询结果缓存（用于重复查询）
pub type QueryCache = HashMap<String, CacheEntry<Vec<HashMap<String, String>>>>;

static QUERY_CACHE: OnceLock<RwLock<QueryCache>> = OnceLock::new();

/// 获取查询缓存
pub fn get_query_cache() -> &'static RwLock<QueryCache> {
    QUERY_CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// 从缓存获取查询结果
pub fn get_cached_query(key: &str) -> Option<Vec<HashMap<String, String>>> {
    let cache = get_query_cache().read().ok()?;
    let entry = cache.get(key)?;

    if entry.is_expired() {
        drop(cache);
        let mut cache = get_query_cache().write().ok()?;
        cache.remove(key);
        None
    } else {
        Some(entry.get().clone())
    }
}

/// 设置查询缓存
pub fn set_cached_query(key: String, data: Vec<HashMap<String, String>>, ttl_secs: u64) {
    if let Ok(mut cache) = get_query_cache().write() {
        let entry = CacheEntry::new(data, Duration::from_secs(ttl_secs));
        cache.insert(key, entry);
    }
}

/// 清除所有查询缓存
pub fn clear_query_cache() {
    if let Ok(mut cache) = get_query_cache().write() {
        cache.clear();
    }
}
