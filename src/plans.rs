use crate::spaces::SimpleSpace;
use mmtk_macros::HasSpaces;

pub trait HasSpaces {
    //fn find_space_dyn(&self, addr: usize) -> Option<&dyn Space>;
    fn trace_object(&self, addr: usize) -> usize;
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

#[derive(HasSpaces)]
pub struct ABadSpace {
    #[parent_field]
    pub common1: CommonPlan,
    // The following should produce a nice error message
    //#[parent_field]
    //pub common2: CommonPlan,
}
