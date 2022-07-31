use bevy::{
    prelude::{debug, default, Entity, Query, ResMut, With, info},
    render::view::{ExtractedView, VisibleEntities},
};

use crate::instancing::material::{
    plugin::InstanceViewMeta, specialized_instanced_material::SpecializedInstancedMaterial,
};

pub fn system<M: SpecializedInstancedMaterial>(
    query_views: Query<Entity, (With<ExtractedView>, With<VisibleEntities>)>,
    mut instance_view_meta: ResMut<InstanceViewMeta<M>>,
) {
    info!("{}", std::any::type_name::<M>());
    instance_view_meta.clear();
    for view_entity in query_views.iter() {
        info!("\tView {view_entity:?}");
        instance_view_meta.insert(view_entity, default());
    }
}