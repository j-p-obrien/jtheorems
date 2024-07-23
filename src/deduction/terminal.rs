use std::fmt::Display;

use crate::term::primitives::universe::UniverseLevel;

use super::{
    context_tree::ContextPtr,
    error::JError,
    judgement::{Judgement, JudgementType},
    the_domain::TheDomain,
};

pub type JResult<T> = Result<T, JError>;

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

    pub(super) fn context(&self) -> ContextPtr {
        self.judgement.context()
    }

    /// Changes the JudgementType to WellFormed in the current Context.
    ///
    /// This may always be done.
    pub fn well_formed_in_context(&mut self) {
        self.judgement.well_formed_in_context();
    }

    /// If the Judgement is a Type and the given name is not taken, we can introduce a FreeVariable
    /// with that name.
    ///
    /// If the Judgement is any other variant or the name is taken we return an error.
    pub fn context_extension(&mut self, name: String) -> JResult<()> {
        match self.judgement_type() {
            JudgementType::Type(typ) => {
                self.judgement =
                    self.domain
                        .try_context_extension_at(name, *typ, self.context())?;
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Type in order to introduce a variable.",
            )),
        }
    }

    /// Forms one of the variables in the current Context e.g. if we have a Context with variables
    /// x: A, y: B then we can construct x:A and y:B. The Judgement would look like this:
    /// x: A, y: B \vdash x: A or x: A, y: B \vdash y: B.
    ///
    /// This can be done whenever the JudgementType is WellFormed and there is a Variable with the
    /// given name in the Context. If the JudgementType is not WellFormed or there are no variables in the
    /// Context, returns an Error.
    pub fn variable_introduction(&mut self, name: &str) -> JResult<()> {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self
                    .domain
                    .try_variable_introduction_at(name, self.context())?;
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to apply the Variable Rule.",
            )),
        }
    }

    /// Forms the Natural Type.
    ///
    /// This can be done whenever the Judgement type is Well Formed. If the Judgement Type is not
    /// Well Formed, returns an Error.
    pub fn natural_formation(&mut self) -> JResult<()> {
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

    /// Forms the Natural Number 0.
    ///
    /// This can be done whenever the JudgementType is WellFormed. If the JudgementType is not WellFormed,
    /// returns an error.
    pub fn zero_introduction(&mut self) -> JResult<()> {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                self.judgement = self.domain.zero_introduction_at(self.context());
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
    pub fn universe_formation(&mut self, level: UniverseLevel) -> JResult<()> {
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

    pub fn pi_formation(&mut self) -> JResult<()> {
        match self.judgement_type() {
            JudgementType::Type(typ) => {
                self.judgement = self.domain.try_pi_formation_at(*typ, self.context())?;
                Ok(())
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Type in order to form a Pi.",
            )),
        }
    }

    pub fn lambda_introduction(&mut self) -> JResult<()> {
        match self.judgement_type() {
            JudgementType::WellFormed => {
                todo!()
            }
            _ => Err(JError::Illegal(
                "Judgement Type must be Well Formed in order to form a Lambda.",
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
        Term,
    };

    #[test]
    fn test_natural_variable_introduction() {
        let mut terminal = Terminal::new();
        let natural_judgement: JudgementType = NaturalType.into();
        let well_formed = JudgementType::well_formed();
        let x = "x".to_string();

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        assert_eq!(terminal.context_extension(x.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        matches!(terminal.context_extension(x), Err(JError::NameTaken(_)));
        assert_eq!(terminal.judgement_type(), &natural_judgement);

        assert_eq!(terminal.context_extension("y".to_string()), Ok(()));
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

        assert_eq!(terminal.context_extension(a.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.universe_formation(0), Ok(()));
        assert_eq!(terminal.judgement_type(), &universe_judgement);

        matches!(
            terminal.context_extension(a.clone()),
            Err(JError::NameTaken(_))
        );
        assert_eq!(terminal.judgement_type(), &universe_judgement);

        assert_eq!(terminal.context_extension("B".to_string()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);
    }

    #[test]
    fn test_zero_formation() {
        let mut terminal = Terminal::new();
        let zero_judgement: JudgementType = Zero.into();

        assert_eq!(terminal.zero_introduction(), Ok(()));
        assert_eq!(terminal.judgement_type(), &zero_judgement);
        matches!(terminal.zero_introduction(), Err(JError::Illegal(_)));
    }

    #[test]
    fn test_variable_rule() {
        // TODO: Make these tests more robust i.e. test that the data is correct as well.
        let mut terminal = Terminal::new();
        let well_formed = JudgementType::well_formed();
        let x = "x".to_string();
        let y = "y".to_string();

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.context_extension(x.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.natural_formation(), Ok(()));
        assert_eq!(terminal.context_extension(y.clone()), Ok(()));
        assert_eq!(terminal.judgement_type(), &well_formed);

        assert_eq!(terminal.variable_introduction(&x), Ok(()));
        matches!(
            terminal.judgement_type(),
            JudgementType::Term(Term::FreeVariable(_))
        );

        terminal.well_formed_in_context();
        assert_eq!(terminal.variable_introduction(&y), Ok(()));
        matches!(
            terminal.judgement_type(),
            JudgementType::Term(Term::FreeVariable(_))
        );
    }
}
