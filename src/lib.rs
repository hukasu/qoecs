use std::{fmt::Display};

#[derive(Debug)]
pub enum ECSError {
    // Array Resource Errors
    ArrayResourceWriteOutOfBoundsError,
    // Archtype Errors
    EntityIdInUseError,
}

impl Display for ECSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ECSError::ArrayResourceWriteOutOfBoundsError => {
                write!(f, "Attempted to write into Array Resource with an out of bounds index.")
            },
            ECSError::EntityIdInUseError => {
                write!(f, "Attempted to create an entity with an already in use Id.")
            },
        }
    }
}

impl std::error::Error for ECSError {}

pub enum ECSEntityCreateConflictResolution {
    Error,
    Replace,
    Ignore
}

#[macro_export]
macro_rules! create_ecs {
    (
        // ECS Struct name
        $name:ident,
        // ECS Required resources
        Resources(
            $(
                $resource_name:ident => $resource_ty:ty
            ),*
        ),
        // ECS Array resources
        ArrayResources(
            $(
                $arr_resource_name:ident => [$arr_resource_ty:ty; $arr_resource_len:literal]
            ),*
        ),
        // ECS Map resources
        MapResources(
            $(
                $map_resource_name:ident => <$map_resource_key_type:ty, $map_resource_value_type:ty>
            ),*
        ),
        // ECS Optional resources
        OptionalResources(
            $(
                $opt_resource_name:ident => $opt_resource_ty:ty
            ),*
        ),
        // ECS Entity Archtypes
        Archtypes(
            $(
                Entity(
                    $entity_name:ident => $entity_id_type:ty,
                    // ECS Entity archtype Components
                    Components(
                        $(
                            $comp_name:ident => $comp_type:ty
                        ),*
                    )
                )
            ),+
        )
    ) => {
        paste::paste! { 
        mod entity {
            $(
                pub struct [<$entity_name:camel Entity>] {
                    $(pub $comp_name: Option<$comp_type>),*
                }

                pub struct [<$entity_name:camel EntityView>]<'a> {
                    $(pub $comp_name: Option<&'a $comp_type>),*
                }
            )+
        }

        struct $name {
            // Required resource properties
            $([<resource_ $resource_name>]: $resource_ty,)*
            // Array resource properties
            $([<resource_ $arr_resource_name>]: [$arr_resource_ty; $arr_resource_len],)*
            // Map resource properties
            $([<resource_ $map_resource_name>]: std::collections::HashMap<$map_resource_key_type, $map_resource_value_type>,)*
            // Optional resource properties
            $([<resource_ $opt_resource_name>]: Option<$opt_resource_ty>,)*
            // Archtype member properties
            $(
                // Entity vector member property
                $entity_name: Vec<$entity_id_type>,
                // Entity components member properties
                $(
                    [<$entity_name _ $comp_name>]: Vec<($entity_id_type, $comp_type)>,
                )*
            )+
        }

        impl $name {
            /// Create a new instance of ECS struct
            fn new() -> $name {
                $name {
                    // Required resources default initialization
                    $([<resource_ $resource_name>]: $resource_ty::default(),)*
                    // Array resources default list initialization
                    $([<resource_ $arr_resource_name>]: [$arr_resource_ty::default(); $arr_resource_len],)*
                    // Map resources empty map initialization
                    $([<resource_ $map_resource_name>]: std::collections::HashMap::new(),)*
                    // Optional resources None initialization
                    $([<resource_ $opt_resource_name>]: None,)*
                    // Archtype storage initialization
                    $(
                        // Entity empty list initialization
                        $entity_name: vec![],
                        // Entity components  empty list initialization
                        $(
                            [<$entity_name _ $comp_name>]: vec![],
                        )*
                    )+
                }
            }

            // Creating Resource methods
            $(
            /// Write a value to Resource
            fn [<write_resource_ $resource_name>](&mut self, $resource_name: $resource_ty) {
                self.[<resource_ $resource_name>] = $resource_name;
            }

            /// Get the value of Resource
            fn [<get_resource_ $resource_name>](&self) -> &$resource_ty {
                &self.[<resource_ $resource_name>]
            }
            )*

            // Creating Array Resource methods
            $(
            /// Write a value to Array Resource at index
            fn [<write_resource_ $arr_resource_name>](
                &mut self,
                $arr_resource_name: $arr_resource_ty,
                index: usize
            ) -> Result<(), ECSError> {
                if index < self.[<resource_ $arr_resource_name>].len() {
                    self.[<resource_ $arr_resource_name>][index] = $arr_resource_name;
                    Ok(())
                } else {
                    Err(ECSError::ArrayResourceWriteOutOfBoundsError)
                }
            }

            /// Clear all values of Array Resource
            fn [<clear_resource_ $arr_resource_name>](&mut self) {
                self.[<resource_ $arr_resource_name>] = [$arr_resource_ty::default(); $arr_resource_len];
            }

            /// Get the value of Array Resource at index
            fn [<get_resource_ $arr_resource_name>](
                &self,
                index: usize
            ) -> Result<&$arr_resource_ty, ECSError> {
                if index < self.[<resource_ $arr_resource_name>].len() {
                    let v = &self.[<resource_ $arr_resource_name>][index];
                    Ok(&v)
                } else {
                    Err(ECSError::ArrayResourceWriteOutOfBoundsError)
                }
            }
            )*

            // Creating Array Resource methods
            $(
            /// Write a value to Map Resource for key
            fn [<write_resource_ $map_resource_name>](
                &mut self,
                [<$map_resource_name _key>]: $map_resource_key_type,
                [<$map_resource_name _value>]: $map_resource_value_type
            ) -> Option<$map_resource_value_type> {
                self.[<resource_ $map_resource_name>].insert(
                    [<$map_resource_name _key>],
                    [<$map_resource_name _value>]
                )
            }

            /// Clear all values of Array Resource
            fn [<clear_resource_ $map_resource_name>](&mut self) {
                self.[<resource_ $map_resource_name>].clear();
            }

            /// Get the value of Map Resource for key
            fn [<get_resource_ $map_resource_name>](
                &self,
                [<$map_resource_name _key>]: $map_resource_key_type
            ) -> Option<&$map_resource_value_type> {
                self.[<resource_ $map_resource_name>].get(&[<$map_resource_name _key>])
            }
            )*

            // Creating Optional Resource methods
            $(
            /// Writes a value to Optional Resource
            fn [<write_resource_ $opt_resource_name>](&mut self, $opt_resource_name: $opt_resource_ty) {
                self.[<resource_ $opt_resource_name>] = Some($opt_resource_name);
            }

            /// Clears value of Optiomal Resource
            fn [<clear_resource_ $opt_resource_name>](&mut self) {
                self.[<resource_ $opt_resource_name>] = None;
            }

            /// Gets value of Optional Resource
            fn [<get_resource_ $opt_resource_name>](&self) -> Option<&$opt_resource_ty> {
                match &self.[<resource_ $opt_resource_name>] {
                    Some(v) => Some(&v),
                    None => None
                }
            }
            )*

            // Creating Entity Archtype methods
            $(
                /// Checks if ID has valid entity
                fn [<has_ $entity_name>](&self, [<$entity_name _id>]: $entity_id_type) -> bool {
                    match self.[<$entity_name>].binary_search(&[<$entity_name _id>]) {
                        Ok(_) => true,
                        Err(_) => false
                    }
                }

                $(
                    /// Adds a component to a Entity
                    fn [<add_ $comp_name _to_ $entity_name>](
                        &mut self,
                        [<$entity_name _id>]: &$entity_id_type,
                        $comp_name: $comp_type
                    ) {
                        let pos = self.[<$entity_name _ $comp_name>].binary_search_by_key(
                            [<$entity_name _id>],
                            |kv| kv.0
                        );
                        match pos {
                            Ok(ind) => self.[<$entity_name _ $comp_name>][ind].1 = $comp_name,
                            Err(ind) => self.[<$entity_name _ $comp_name>].insert(
                                ind,
                                (*[<$entity_name _id>], $comp_name)
                            )
                        };
                    }
                )*

                /// Creates a new entity with given Id and Components
                fn [<_create_ $entity_name>](
                    &mut self,
                    [<$entity_name _id>]: $entity_id_type,
                    $entity_name: entity::[<$entity_name:camel Entity>]
                ) -> $entity_id_type{
                    match self.[<$entity_name>].binary_search(&[<$entity_name _id>]) {
                        Ok(ind) => (),
                        Err(ind) => self.[<$entity_name>].insert(ind, [<$entity_name _id>])
                    }

                    let entity::[< $entity_name:camel Entity>] {
                        $($comp_name: $comp_name),*
                    } = $entity_name;

                    $(
                        if let Some(comp) = $comp_name {
                            self.[<add_ $comp_name _to_ $entity_name>](&[<$entity_name _id>], comp);
                        }
                    )*
                    [<$entity_name _id>]
                }

                /// Creates a new entity with given Id and Components
                /// after verifying for conflict.
                /// 
                /// The resolution of conflict is as follows, based on 
                /// the value passed by `conflict_resolution`:  
                /// 
                /// | *conflict_resolution* | Resolution |  
                /// |---------|---------|  
                /// | Error | Returns `ECSError::EntityIdInUseError` error. |  
                /// | Ignore | Returns Id without modifying existing entity. |  
                /// | Replace | Returns Id replacing existing entity. |  
                fn [<create_ $entity_name>](
                    &mut self,
                    [<$entity_name _id>]: $entity_id_type,
                    $entity_name: entity::[<$entity_name:camel Entity>],
                    conflict_resolution: ECSEntityCreateConflictResolution
                ) -> Result<$entity_id_type, ECSError> {
                    let exists = self.[<has_ $entity_name>]([<$entity_name _id>]);
                    match (conflict_resolution, exists) {
                        (ECSEntityCreateConflictResolution::Error, true) => Err(ECSError::EntityIdInUseError),
                        (ECSEntityCreateConflictResolution::Ignore, true) => Ok([<$entity_name _id>]),
                        (_) => Ok(self.[<_create_ $entity_name>]([<$entity_name _id>], $entity_name))
                    }
                }

                // Entity components methods
                fn [<get_ $entity_name>]<'a>(&'a self, [<$entity_name _id>]: $entity_id_type) -> Option<entity::[<$entity_name:camel EntityView>]> {
                    match self.[<$entity_name>].binary_search(&[<$entity_name _id>]) {
                        Ok(ind) => {
                            Some(
                                entity::[<$entity_name:camel EntityView>] {
                                    $($comp_name: self.[<get_ $comp_name _of_ $entity_name>]([<$entity_name _id>])),*
                                }
                            )
                        },
                        Err(_) => None
                    }
                }

                $(
                    /// Gets the Component from the entity of Id
                    fn [<get_ $comp_name _of_ $entity_name>]<'a>(&'a self, [<$entity_name _id>]: $entity_id_type) -> Option<&'a $comp_type> {
                        match self.[<$entity_name _ $comp_name>].binary_search_by_key(
                            &[<$entity_name _id>], |kc| { kc.0 }
                        ) {
                            Ok(real_index) => {
                                match self.[<$entity_name _ $comp_name>].get(real_index) {
                                    Some(kc) => Some(&kc.1),
                                    None => None
                                }
                            },
                            Err(_) => None
                        }
                    }
                )*
            )+ // for each archtype end
        } // impl $ecs end
        } // paste! end
    };
} // macro_rules end