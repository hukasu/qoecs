use qoecs::*;

type U64Map = std::collections::HashMap<u64, u64>;

create_ecs!(
    TestECS,
    u64,
    Resources(time => u64),
    ArrayResources(seq => [u64; 10]),
    MapResources(map => <u64, String>),
    OptionalResources(gravity => f32)
);

#[test]
fn has_entity_on_empty_ecs() {
    let ecs = TestECS::new();
    assert!(!ecs.has_entity(0));
}

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
        assert_eq!(ecs.get_resource_map(i), None);
    }

    assert_eq!(ecs.write_resource_map(0, "A".to_owned()), None);
    assert_eq!(ecs.write_resource_map(0, "B".to_owned()), Some("A".to_owned()));
    assert_eq!(ecs.write_resource_map(0, "C".to_owned()), Some("B".to_owned()));
    assert_eq!(ecs.write_resource_map(0, "A".to_owned()), Some("C".to_owned()));
    for i in 1..100 {
        assert_eq!(ecs.get_resource_map(i), None);
        ecs.write_resource_map(i, "A".to_owned());
    }
    for i in 0..100 {
        assert_eq!(ecs.get_resource_map(i), Some(&"A".to_owned()));
    }
    for i in 100..110 {
        assert_eq!(ecs.get_resource_map(i), None);
    }

    ecs.clear_resource_map();
    for i in 0..100 {
        assert_eq!(ecs.get_resource_map(i), None);
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