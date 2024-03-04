use super::*;

impl TryFrom<gen::Felt> for StarkFelt {
    type Error = Error;
    fn try_from(felt: gen::Felt) -> Result<Self, Self::Error> {
        let felt = felt.as_ref().as_str();
        let felt = StarkFelt::try_from(felt)?;
        Ok(felt)
    }
}

impl TryFrom<&StarkFelt> for gen::Felt {
    type Error = Error;
    fn try_from(felt: &StarkFelt) -> Result<Self, Self::Error> {
        let hex = hex::encode(felt.bytes());
        let felt = gen::Felt::try_new(&hex)?;
        Ok(felt)
    }
}

impl TryFrom<StarkFelt> for gen::Felt {
    type Error = Error;
    fn try_from(felt: StarkFelt) -> Result<Self, Self::Error> {
        let felt = &felt;
        felt.try_into()
    }
}
