use super::StateSet;

pub struct DFA<'a> {
    pub start: StateSet,
    pub accepts: StateSet,
    pub transition: Box<dyn (Fn(&StateSet, String) -> StateSet) + 'a>,
}
