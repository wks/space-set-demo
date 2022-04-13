use space_set_demo::{
    plans::{BasePlan, CommonPlan, HasSpaces},
    spaces::SimpleSpace,
};

fn main() {
    let plan = CommonPlan {
        immortal: SimpleSpace {
            name: "immortal_space".to_string(),
            start: 0x40000,
            length: 0x10000,
        },
        los: SimpleSpace {
            name: "los".to_string(),
            start: 0x50000,
            length: 0x10000,
        },
        base: BasePlan {
            code_space: SimpleSpace {
                name: "code_space".to_string(),
                start: 0x60000,
                length: 0x10000,
            },
            vm_space: SimpleSpace {
                name: "vm_space".to_string(),
                start: 0x70000,
                length: 0x10000,
            },
        },
    };

    println!("Find spaces of the object:");

    for objref in [0x40010, 0x50010, 0x60010, 0x70010] {
        let space = plan
            .find_space_dyn(objref)
            .expect("should be in some space");
        println!("0x{:x} is in {}", objref, space.name());
    }

    println!("The following should not find any space:");

    assert!(plan.find_space_dyn(0x80010).is_none());
}
