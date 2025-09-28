use std::collections::hash_map::Iter;
use std::collections::hash_map::Keys;
use std::collections::HashMap;

use log::LevelFilter;

/// LevelConfig stores the relation target->LevelFilter in a hash table.
///
/// The table supports copying from the logger system LevelConfig to
/// a widget's LevelConfig. In order to detect changes, the generation
/// of the hash table is compared with any previous copied table.
/// On every change the generation is incremented.
#[derive(Default)]
pub struct LevelConfig {
    config: HashMap<String, LevelFilter>,
    generation: u64,
    origin_generation: u64,
    default_display_level: Option<LevelFilter>,
}
impl LevelConfig {
    /// Create an empty LevelConfig.
    pub fn new() -> LevelConfig {
        LevelConfig {
            config: HashMap::new(),
            generation: 0,
            origin_generation: 0,
            default_display_level: None,
        }
    }
    /// Set for a given target the LevelFilter in the table and update the generation.
    pub fn set(&mut self, target: &str, level: LevelFilter) {
        if let Some(lev) = self.config.get_mut(target) {
            if *lev != level {
                *lev = level;
                self.generation += 1;
            }
            return;
        }
        self.config.insert(target.to_string(), level);
        self.generation += 1;
    }
    /// Set default display level filter for new targets - independent from recording
    pub fn set_default_display_level(&mut self, level: LevelFilter) {
        self.default_display_level = Some(level);
    }
    /// Get default display level filter for new targets - independent from recording
    pub fn get_default_display_level(&self) -> Option<LevelFilter> {
        self.default_display_level
    }
    /// Retrieve an iter for all the targets stored in the hash table.
    pub fn keys(&self) -> Keys<'_, String, LevelFilter> {
        self.config.keys()
    }
    /// Get the levelfilter for a given target.
    pub fn get(&self, target: &str) -> Option<LevelFilter> {
        self.config.get(target).cloned()
    }
    /// Retrieve an iterator through all entries of the table.
    pub fn iter(&self) -> Iter<'_, String, LevelFilter> {
        self.config.iter()
    }
    /// Merge an origin LevelConfig into this one.
    ///
    /// The origin table defines the maximum levelfilter.
    /// If this table has a higher levelfilter, then it will be reduced.
    /// Unknown targets will be copied to this table.
    pub(crate) fn merge(&mut self, origin: &LevelConfig) {
        if self.origin_generation != origin.generation {
            for (target, origin_levelfilter) in origin.iter() {
                if let Some(levelfilter) = self.get(target) {
                    if levelfilter <= *origin_levelfilter {
                        continue;
                    }
                }
                let levelfilter = self
                    .default_display_level
                    .map(|lvl| {
                        if lvl > *origin_levelfilter {
                            *origin_levelfilter
                        } else {
                            lvl
                        }
                    })
                    .unwrap_or(*origin_levelfilter);
                self.set(target, levelfilter);
            }
            self.generation = origin.generation;
        }
    }
}
