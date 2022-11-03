# QOECS
Quite Ok Entity-Component-System

ECS with knowledge of Resources, Components and Systems at compile time without the need of Register pattern.

## Example
The creating of a ECS is entirely done by a macro.
```rust
create_ecs!(
    MyECS, // ECS name
    u64, // Entity ID type
    Resources( // Required resources
        res_a => u64,
        res_b => f32
    ),
    ArrayResources( // Fixed lenght array resources
        res_c => [u64; 10],
        res_d => [u64; 10]
    ),
    OptionalResources( // Optional resources, defaults to None and can be cleared
        res_e => u64,
        res_e => f32
    )
    todo!()
);

let ecs = MyEcs::new();
```