use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};

/// A Helper function to configure inspector plugin resources
pub trait AppInspector {
    /// Custom logic for adding resources
    fn init_inspector_resource<T>(&mut self) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static;
}

impl AppInspector for App {
    fn init_inspector_resource<T>(&mut self) -> &mut App
    where
        T: Inspectable + FromWorld + Send + Sync + 'static,
    {
        #[cfg(feature = "editor")]
        return self.add_plugin(InspectorPlugin::<T>::new().open(false));

        #[cfg(not(feature = "editor"))]
        self.init_resource::<T>()
    }
}
