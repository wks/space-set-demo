use crate::spaces::{SimpleSpace, Space};
use mmtk_macros::HasSpaces;

pub trait HasSpaces {
    fn trace_object(&self, addr: usize) -> usize;
    fn find_space_dyn(&self, addr: usize) -> Option<&dyn Space>;
}

#[derive(HasSpaces)]
pub struct BasePlan {
    #[space_field]
    pub code_space: SimpleSpace,
    #[space_field]
    pub vm_space: SimpleSpace,
}

#[derive(HasSpaces)]
pub struct CommonPlan {
    #[space_field]
    pub immortal: SimpleSpace,
    #[space_field]
    pub los: SimpleSpace,
    #[parent_field]
    pub base: BasePlan,
}

#[derive(HasSpaces)]
pub struct GenCopy {
    #[space_field]
    pub nursery: SimpleSpace,
    #[space_field]
    pub copyspace1: SimpleSpace,
    #[space_field]
    pub copyspace2: SimpleSpace,
    #[parent_field]
    pub common: CommonPlan,
}

#[cfg(feature = "bad-examples")]
mod bad_plans {
    use crate::plans::CommonPlan;
    use crate::spaces::SimpleSpace;
    use mmtk_macros::HasSpaces;

    #[derive(HasSpaces)]
    pub struct BadPlan1 {
        // Duplicated attributes
        #[space_field]
        #[space_field]
        space1: SimpleSpace,
    }

    #[derive(HasSpaces)]
    pub struct BadPlan2 {
        #[parent_field]
        pub common1: CommonPlan,
        // Duplicated parent
        #[parent_field]
        pub common2: CommonPlan,
    }

    // Tuple-like struct
    #[derive(HasSpaces)]
    pub struct TupleStruct(i32, i64);
}
