use std::fmt::Display;

use crate::term::primitives::universe::UniverseLevel;

use super::{
    context_tree::Context,
    error::JError,
    judgement::{Judgement, JudgementType},
    the_domain::TheDomain,
};

pub type JResult = Result<(), JError>;

#[derive(Debug, Clone, PartialEq, Eq)]
/// A Terminal is a window into The Domain, the central repository of all of terms and their
/// associated Context.
///
/// You interact with The Domain through your Terminal. It contains the current judgement and
/// a pointer into the data contained in The Domain.
pub struct Terminal {
    domain: TheDomain,
    judgement: Judgement,
}

impl Display for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Terminal {
    /// Creates a new Terminal containing a new Domain and a pointer to the root of the
    /// ContextTree, with a Judgement Type that is WellFormed.
    pub fn new() -> Self {
        Self {
            domain: TheDomain::new(),
            judgement: Judgement::well_formed_empty_context(),
        }
    }

    pub fn judgement_type(&self) -> &JudgementType {
        self.judgement.judgement_type()
    }

    pub(super) fn context(&self) -> Context {
        self.judgement.context()
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant or the name is taken we return an error.
    pub fn variable_introduction(&mut self, name: String) -> JResult {
        match self.judgement_type() {
            JudgementType::Type(typ) => {
                self.judgement =
                    self.domain
                        .try_variable_introduction_at(name, *typ, self.context())?;
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Type in order to introduce a variable.",
            )),
        }
    }

    pub fn variable_rule(&mut self) -> JResult {
        todo!("Variable Rule")
    }

    /// Forms the Natural Type.
    ///
    /// This can be done whenever the Judgement type is Well Formed. If the Judgement Type is not
    /// Well Formed, returns an Error.
    pub fn natural_formation(&mut self) -> JResult {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self.domain.natural_formation_at(self.context());
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to form Natural Type.",
            )),
        }
    }

    pub fn zero_formation(&mut self) -> JResult {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self.domain.zero_formation_at(self.context());
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to form Zero.",
            )),
        }
    }

    /// Forms the Universe at the given level.
    ///
    /// This can be done whenever the Judgement type is Well Formed. If the Judgement Type is not
    /// Well Formed, returns an Error.
    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self.domain.universe_formation_at(level, self.context());
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to form a Universe.",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::term::{
        self,
        primitives::{
            naturals::{NaturalType, Zero},
            universe::Universe,
        },
    };

    #[test]
    fn test_natural_variable_introduction() {
        let mut terminal = Terminal::new();
        let natural_judgement: JudgementType = NaturalType.into();
        let well_formed = JudgementType::well_formed();
        let x = "x".to_string();

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        assert_eq!(terminal.variable_introduction(x.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        matches!(terminal.variable_introduction(x), Err(JError::NameTaken(_)));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        assert_eq!(terminal.variable_introduction("y".to_string()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);
    }

    #[test]
    fn test_universe_variable_introduction() {
        let mut terminal = Terminal::new();
        let universe_judgement: JudgementType = Universe::new(0).into();
        let well_formed = JudgementType::well_formed();
        let a = "A".to_string();

        assert_eq!(terminal.universe_formation(0), Ok(()));
        assert_eq!(terminal.judgement_type(), &universe_judgement);

        assert_eq!(terminal.variable_introduction(a.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.universe_formation(0), Ok(()));
        assert_eq!(terminal.judgement_type(), &universe_judgement);

        matches!(
            terminal.variable_introduction(a.clone()),
            Err(JError::NameTaken(_))
        );
        assert_eq!(terminal.judgement_type(), &universe_judgement);

        assert_eq!(terminal.variable_introduction("B".to_string()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);
    }

    #[test]
    fn test_zero_formation() {
        let mut terminal = Terminal::new();
        let zero_judgement: JudgementType = Zero.into();
        let well_formed = JudgementType::well_formed();

        assert_eq!(terminal.zero_formation(), Ok(()));
        assert_eq!(terminal.judgement_type(), &zero_judgement);
        matches!(terminal.zero_formation(), Err(JError::Illegal(_)));
    }
}
