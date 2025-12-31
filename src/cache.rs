use anyhow::{Context, Result};
use dashmap::DashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::SystemTime;

use crate::parser;
use crate::theme::Theme;

/// Thread-safe cache for parsed themes with file modification time tracking
pub struct ThemeCache {
    cache: Arc<DashMap<PathBuf, (Theme, SystemTime)>>,
}

impl ThemeCache {
    /// Create a new empty theme cache
    pub fn new() -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
        }
    }

    /// Get a theme from cache or parse it if not cached or file has changed
    pub fn get_or_parse<P: AsRef<Path>>(&self, path: P) -> Result<Theme> {
        let path = path.as_ref().to_path_buf();
        
        // Get file modification time
        let metadata = fs::metadata(&path)
            .with_context(|| format!("Failed to read metadata for: {:?}", path))?;
        let file_mtime = metadata
            .modified()
            .with_context(|| format!("Failed to get modification time for: {:?}", path))?;

        // Check cache
        if let Some(entry) = self.cache.get(&path) {
            let (cached_theme, cached_mtime) = entry.value();
            if *cached_mtime == file_mtime {
                // Cache hit - file hasn't changed
                return Ok(cached_theme.clone());
            }
            // File has changed, remove stale entry
            self.cache.remove(&path);
        }

        // Cache miss or stale - parse file
        let theme = parser::parse_theme_file(&path)?;
        
        // Store in cache
        self.cache.insert(path, (theme.clone(), file_mtime));
        
        Ok(theme)
    }

    /// Invalidate a specific cache entry
    pub fn invalidate<P: AsRef<Path>>(&self, path: P) {
        let path = path.as_ref().to_path_buf();
        self.cache.remove(&path);
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        self.cache.clear();
    }

    /// Get the number of cached entries
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}

impl Default for ThemeCache {
    fn default() -> Self {
        Self::new()
    }
}

/// Global theme cache instance
static GLOBAL_CACHE: once_cell::sync::Lazy<ThemeCache> = once_cell::sync::Lazy::new(ThemeCache::new);

/// Get the global theme cache
pub fn global_cache() -> &'static ThemeCache {
    &GLOBAL_CACHE
}
