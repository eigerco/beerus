use super::*;

impl TryFrom<gen::Felt> for StarkFelt {
    type Error = Error;
    fn try_from(felt: gen::Felt) -> Result<Self, Self::Error> {
        let felt = felt.as_ref().as_str();
        let felt = StarkFelt::try_from(felt)?;
        Ok(felt)
    }
}
