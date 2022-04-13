use crate::{
    plans::{BasePlan, CommonPlan, GenCopy, HasSpaces},
    spaces::SimpleSpace,
};

mod plans;
mod spaces;

fn main() {
    let plan = GenCopy {
        nursery: SimpleSpace {
            name: "nursery".to_string(),
            start: 0x10000,
            length: 0x10000,
        },
        copyspace1: SimpleSpace {
            name: "copyspace1".to_string(),
            start: 0x20000,
            length: 0x10000,
        },
        copyspace2: SimpleSpace {
            name: "copyspace2".to_string(),
            start: 0x30000,
            length: 0x10000,
        },
        common: CommonPlan {
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
        },
    };

    println!("Trace objects:");

    for objref in [
        0x10010, 0x20010, 0x30010, 0x40010, 0x50010, 0x60010, 0x70010,
    ] {
        plan.trace_object(objref);
    }

    println!("The following should panic:");

    std::panic::catch_unwind(|| {
        plan.trace_object(0x80010);
    })
    .expect_err("0x80010 should not be in any space.");
}
