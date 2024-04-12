use std::any::TypeId;

#[derive(Clone)]
pub struct TermType {
    pub term_type: TypeId,
}

#[derive(Clone)]
pub struct Term {
    pub term_type: TermType,
    pub literal: String,
}

pub trait FTerm {
    fn call(&self) -> Term;
}

impl FTerm for Term {
    fn call(&self) -> Term {
        self.clone()
    }
}

pub fn for_all<T>(term: T) -> Term
where
    T: FTerm,
{
    term.call()
}

// for_all(A, B) -> A -> B -> bool

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let term = Term {
            term_type: TermType {
                term_type: TypeId::of::<bool>(),
            },
            literal: String::from("false"),
        };

        let result = for_all(term);

        assert_eq!(result.term_type.term_type, TypeId::of::<bool>());
    }
}
