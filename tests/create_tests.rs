use qoecs::*;

#[derive(PartialEq, Clone, Debug)]
pub struct PositionComponent {
    x: f64,
    y: f64
}

#[derive(PartialEq, Clone, Debug)]
pub struct VelocityComponent {
    vector: (f64, f64)
}

#[derive(PartialEq, Clone, Debug)]
pub struct WeaponComponent {
    attack: i32
}

#[derive(PartialEq, Clone, Debug)]
pub struct ArmorComponent {
    defense: i32
}

#[derive(PartialEq, Clone, Debug)]
pub struct UseComponent {
    health: i32
}

create_ecs!(
    TestECS,
    Resources(time => u64, step => u64),
    ArrayResources(seq => [u64; 10], offset => [u64; 10]),
    MapResources(name_conv => <u64, String>, text_conv => <u64, String>),
    OptionalResources(gravity => f32),
    Archtypes(
        Entity(
            mob => u64,
            Components(
                position => crate::PositionComponent,
                clone_position => crate::PositionComponent,
                velocity => crate::VelocityComponent
            )
        ),
        Entity(
            item => u64,
            Components(
                weapon => crate::WeaponComponent,
                armor => crate::ArmorComponent,
                useable => crate::UseComponent
            )
        )
    )
);

#[test]
fn required_resource_test() {
    let mut ecs = TestECS::new();
    assert_eq!(ecs.get_resource_time(), &0);
    
    ecs.write_resource_time(100);
    assert_eq!(ecs.get_resource_time(), &100);
    
    ecs.write_resource_time(200);
    assert_eq!(ecs.get_resource_time(), &200);
}

#[test]
fn array_resource_test() -> Result<(), ECSError> {
    let mut ecs = TestECS::new();
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
fn map_resource_test() -> Result<(), ECSError> {
    let mut ecs = TestECS::new();
    for i in 0..100 {
        assert_eq!(ecs.get_resource_name_conv(i), None);
    }

    assert_eq!(ecs.write_resource_name_conv(0, "A".to_owned()), None);
    assert_eq!(ecs.write_resource_name_conv(0, "B".to_owned()), Some("A".to_owned()));
    assert_eq!(ecs.write_resource_name_conv(0, "C".to_owned()), Some("B".to_owned()));
    assert_eq!(ecs.write_resource_name_conv(0, "A".to_owned()), Some("C".to_owned()));
    for i in 1..100 {
        assert_eq!(ecs.get_resource_name_conv(i), None);
        ecs.write_resource_name_conv(i, "A".to_owned());
    }
    for i in 0..100 {
        assert_eq!(ecs.get_resource_name_conv(i), Some(&"A".to_owned()));
    }
    for i in 100..110 {
        assert_eq!(ecs.get_resource_name_conv(i), None);
    }

    ecs.clear_resource_name_conv();
    for i in 0..100 {
        assert_eq!(ecs.get_resource_name_conv(i), None);
    }

    Ok(())
}

#[test]
fn array_resource_outofbound_test() {
    let mut ecs = TestECS::new();
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
    let mut ecs = TestECS::new();
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

#[test]
fn archtype_test() {
    let mut ecs = TestECS::new();
    assert!(!ecs.has_mob(0));

    let mob_pos = Some(PositionComponent{x: 0., y: 0.});
    let mob_clone_pos: Option<PositionComponent> = None;
    let mob_vel = Some(VelocityComponent{ vector: (1., 0.)});

    let mob = entity::MobEntity {
        position: mob_pos.clone(),
        clone_position: mob_clone_pos.clone(),
        velocity: mob_vel.clone()
    };
    let mob_id = ecs.create_mob(
        0,
        mob,
        ECSEntityCreateConflictResolution::Error
    ).unwrap();

    let mob = entity::MobEntity {
        position: mob_pos.clone(),
        clone_position: mob_clone_pos.clone(),
        velocity: mob_vel.clone()
    };
    if let Ok(_) = ecs.create_mob(
        mob_id,
        mob,
        ECSEntityCreateConflictResolution::Error
    ) {
        panic!("Should error on duplicate entity.")
    }

    assert_eq!(mob_pos.as_ref(), ecs.get_position_of_mob(mob_id));
    assert_eq!(mob_clone_pos.as_ref(), ecs.get_clone_position_of_mob(mob_id));
    assert_eq!(mob_vel.as_ref(), ecs.get_velocity_of_mob(mob_id));

    if let Some(
        entity::MobEntityView {
            position: pos_view,
            clone_position: clone_pos_view,
            velocity: vel_view,
        }
    ) = ecs.get_mob(mob_id) {
        assert_eq!(mob_pos.as_ref(), pos_view);
        assert_eq!(mob_clone_pos.as_ref(), clone_pos_view);
        assert_eq!(mob_vel.as_ref(), vel_view);
    } else {
        panic!("Entity should exists.")
    }
    
    todo!("Add entities")
}