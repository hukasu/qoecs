# QOECS
Quite Ok Entity-Component-System

ECS with knowledge of Resources, Components and Systems at compile time without the need of Register pattern.

## Disclamer
All versions will contain breaking changes for a good while.

## Example
The creating of a ECS is entirely done by a macro.
```rust
create_ecs!(
    MyECS, // ECS name
    Resources(time => u64, step => u64), // Single value resources
    ArrayResources(seq => [u64; 10], offset => [u64; 10]), // Fixed-size array resources
    MapResources(name_conv => <u64, String>, text_conv => <u64, String>), // Map resources
    OptionalResources(gravity => f32), // Optional resources
    Archtypes( // Entity archtypes
        Entity( // First entity archtype
            mob => u64,
            Components(
                position => crate::PositionComponent,
                clone_position => crate::PositionComponent,
                velocity => crate::VelocityComponent
            )
        ),
        Entity( // Second entity archtype
            item => u64,
            Components(
                weapon => crate::WeaponComponent,
                armor => crate::ArmorComponent,
                useable => crate::UseComponent
            )
        )
    )
    todo!()
);

let ecs = MyEcs::new();
```

## Todo
A list of TODOs.  

| TODO | Description |
| --- | --- |
| Update entity | Create method to update entities |
| Delete entity | Create method to delete entities |
| Remove component | Create method to remove component from entities |
| Entity iterator | Create iterator to iterate over entities |
| Generational ID | Create struct to hold generational ID |
| Systems | Create systems to update entities |

## Changelog
### 2022-11-22
* Create methods to create entities
* Create methods to recover entities
* Create methods to create component to entity
* Create methods to recover component to entity