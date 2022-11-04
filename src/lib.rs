use std::fmt::Display;

#[derive(Debug)]
pub enum ECSError {
    ArrayResourceWriteOutOfBoundsError
}

impl Display for ECSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ECSError::ArrayResourceWriteOutOfBoundsError => {
                write!(f, "Attempted to write into Array Resource with an out of bounds index.")
            }
        }
    }
}

#[macro_export]
macro_rules! create_ecs {
    (
        $name:ident,
        $entity_id_ident:ident,
        Resources($($resource_name:ident => $resource_ty:ty),*),
        ArrayResources($($arr_resource_name:ident => [$arr_resource_ty:ty; $arr_resource_len:literal]),*),
        MapResources($($map_resource_name:ident => <$map_resource_key_type:ty, $map_resource_value_type:ty>),*),
        OptionalResources($($opt_resource_name:ident => $opt_resource_ty:ty),*)
    ) => {
        paste::paste! { 
        pub struct $name {
            entities: Vec<$entity_id_ident>,
            $([<resource_ $resource_name>]: $resource_ty,)*
            $([<resource_ $arr_resource_name>]: [$arr_resource_ty; $arr_resource_len],)*
            $([<resource_ $map_resource_name>]: std::collections::HashMap<$map_resource_key_type, $map_resource_value_type>,)*
            $([<resource_ $opt_resource_name>]: Option<$opt_resource_ty>,)*
        }

        impl $name {
            /// Create a new instance of ECS struct
            fn new() -> $name {
                $name {
                    entities: vec![],
                    $([<resource_ $resource_name>]: $resource_ty::default(),)*
                    $([<resource_ $arr_resource_name>]: [$arr_resource_ty::default(); $arr_resource_len],)*
                    $([<resource_ $map_resource_name>]: std::collections::HashMap::new(),)*
                    $([<resource_ $opt_resource_name>]: None,)*
                }
            }

            /// Checks if entity with `entity_id` exists in ECS
            fn has_entity(&self, entity_id: $entity_id_ident) -> bool {
                match self.entities.binary_search(&entity_id) {
                    Ok(_) => true,
                    Err(_) => false
                }
            }

            // Creating Resource methods
            $(
            /// Write a value to Resource
            fn [<write_resource_ $resource_name>](&mut self, $resource_name: $resource_ty) {
                self.[<resource_ $resource_name>] = $resource_name;
            }

            /// Get the value of Resource
            #[cfg(test)]
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
            #[cfg(test)]
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
            #[cfg(test)]
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
            #[cfg(test)]
            fn [<get_resource_ $opt_resource_name>](&self) -> Option<&$opt_resource_ty> {
                match &self.[<resource_ $opt_resource_name>] {
                    Some(v) => Some(&v),
                    None => None
                }
            }
            )*
        }
        } // paste! end
    };
}