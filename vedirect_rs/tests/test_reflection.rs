use bevy_reflect::Struct;
use vedirect_rs::structs::VEDirectBlock;

#[cfg(test)]
#[test]
fn test_field_reflection_good() {
    let foo = VEDirectBlock::default();
    let bv;
    let dr;
     match foo.field("battery_volts"){
        Some(v) => {
            bv = v;
            match bv.downcast_ref::<f32>() {
                Some(s) => {
                    dr = s;
                    assert_eq!(*dr, 0.0f32);
                },
                None => { assert!(false,"field should be donwcastable to f32")}
            };
        },
        None => { assert!(false,"value should exist but doesnt")}
    };
}
#[test]
fn test_field_reflection_bad_name() {
    let foo = VEDirectBlock::default();
    match foo.field("foo_bar"){
        Some(_) => { assert!(false,"field name should not have existed")},
        None => { assert!(true,"foo_bar should not exist and does not")}
    };
}
