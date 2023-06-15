use nohash_hasher::IntSet;

use re_data_store::{EntityPath, EntityProperties, EntityPropertyMap, TimeInt, Timeline};

pub struct SceneQuery<'s> {
    /// The root of the space in which context the query happens.
    pub space_origin: &'s EntityPath,

    /// All queried entities.
    ///
    /// Contains also invisible objects, use `iter_entities` to iterate over visible ones.
    entity_paths: &'s IntSet<EntityPath>,

    /// The timeline we're on.
    pub timeline: Timeline,

    /// The time on the timeline we're currently at.
    pub latest_at: TimeInt,

    /// The entity properties for all queried entities.
    /// TODO(jleibs/wumpf): This will be replaced by blueprint queries.
    pub entity_props_map: &'s EntityPropertyMap,
}

impl<'s> SceneQuery<'s> {
    pub fn new(
        space_origin: &'s EntityPath,
        entity_paths: &'s IntSet<EntityPath>,
        timeline: Timeline,
        latest_at: TimeInt,
        entity_props_map: &'s EntityPropertyMap,
    ) -> Self {
        Self {
            space_origin,
            entity_paths,
            timeline,
            latest_at,
            entity_props_map,
        }
    }

    /// Iter over all of the currently visible [`EntityPath`]s in the [`SceneQuery`].
    ///
    /// Also includes the corresponding [`EntityProperties`].
    pub fn iter_entities(&self) -> impl Iterator<Item = (&EntityPath, EntityProperties)> {
        self.entity_paths
            .iter()
            .map(|entity_path| (entity_path, self.entity_props_map.get(entity_path)))
            .filter(|(_entity_path, props)| props.visible)
    }
}
