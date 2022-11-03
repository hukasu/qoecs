use std::fmt::Display;

use paste::paste;

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
        OptionalResources($($opt_resource_name:ident => $opt_resource_ty:ty),*)
    ) => {
        paste! { 
        pub struct $name {
            entities: Vec<$entity_id_ident>,
            $([<resource_ $resource_name>]: $resource_ty,)*
            $([<resource_ $arr_resource_name>]: [$arr_resource_ty; $arr_resource_len],)*
            $([<resource_ $opt_resource_name>]: Option<$opt_resource_ty>,)*
        }

        impl $name {
            /// Create a new instance of ECS struct
            fn new() -> $name {
                $name {
                    entities: vec![],
                    $([<resource_ $resource_name>]: $resource_ty::default(),)*
                    $([<resource_ $arr_resource_name>]: [$arr_resource_ty::default(); $arr_resource_len],)*
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

#[cfg(test)]
mod test {
    use super::*;

    type U64Map = std::collections::HashMap<u64, u64>;

    create_ecs!(
        TestECS1,
        u64,
        Resources(time => u64),
        ArrayResources(),
        OptionalResources()
    );

    create_ecs!(
        TestECS2,
        u64,
        Resources(),
        ArrayResources(seq => [u64; 10]),
        OptionalResources()
    );

    create_ecs!(
        TestECS3,
        u64,
        Resources(),
        ArrayResources(),
        OptionalResources(gravity => f32)
    );

    #[test]
    fn has_entity_on_empty_ecs() {
        let ecs = TestECS1::new();
        assert!(!ecs.has_entity(0));
    }

    #[test]
    fn required_resource_test() {
        let mut ecs = TestECS1::new();
        assert_eq!(ecs.get_resource_time(), &0);
        
        ecs.write_resource_time(100);
        assert_eq!(ecs.get_resource_time(), &100);
        
        ecs.write_resource_time(200);
        assert_eq!(ecs.get_resource_time(), &200);
    }

    #[test]
    fn array_resource_test() -> Result<(), ECSError> {
        let mut ecs = TestECS2::new();
        for i in 0..10 {
            assert_eq!(ecs.get_resource_seq(i)?, &0);
        }
        
        ecs.write_resource_seq(100, 0)?;
        assert_eq!(ecs.get_resource_seq(0)?, &100);
        for i in 1..10 {
            assert_eq!(ecs.get_resource_seq(i)?, &0);
        }

        for i in 1..10 {
            ecs.write_resource_seq(i, i as usize)?;
        }
        for i in 1..10 {
            assert_eq!(ecs.get_resource_seq(i)?, &(i as u64));
        }
        
        ecs.clear_resource_seq();
        for i in 0..10 {
            assert_eq!(ecs.get_resource_seq(i)?, &0);
        }
        Ok(())
    }

    #[test]
    fn array_resource_outofbound_test() {
        let mut ecs = TestECS2::new();
        match ecs.write_resource_seq(42, 42) {
            Err(ECSError::ArrayResourceWriteOutOfBoundsError) => (),
            _ => panic!("Should return Out of Bound Error")
        };
        match ecs.get_resource_seq(42) {
            Err(ECSError::ArrayResourceWriteOutOfBoundsError) => (),
            _ => panic!("Should return Out of Bound Error")
        };
    }

    #[test]
    fn optional_resource_test() {
        let mut ecs = TestECS3::new();
        assert_eq!(ecs.get_resource_gravity(), None);
        
        ecs.write_resource_gravity(0.42);
        assert!(
            match ecs.get_resource_gravity() {
                Some(v) => (v - 0.42) < 0.00001,
                None => false
            }
        );
        
        ecs.clear_resource_gravity();
        assert_eq!(ecs.get_resource_gravity(), None);
    }
}
