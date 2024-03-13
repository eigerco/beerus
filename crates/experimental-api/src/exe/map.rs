use blockifier::execution::contract_class::{ContractClassV0, ContractClassV1};

use self::gen::DeprecatedContractClass;

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
        let hex = {
            // drop leading zeroes in order to match the regex
            let hex = hex.trim_start_matches('0');
            let hex = if hex.is_empty() { "0" } else { hex };
            format!("0x{hex}")
        };
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

impl TryFrom<gen::GetClassResult> for ContractClass {
    type Error = Error;

    fn try_from(value: gen::GetClassResult) -> Result<Self, Self::Error> {
        let json = serde_json::to_string(&value)?;
        Ok(match value {
            gen::GetClassResult::ContractClass(_) => {
                let class = ContractClassV1::try_from_json_string(&json)?;
                ContractClass::V1(class)
            }
            gen::GetClassResult::DeprecatedContractClass(class) => {
                let class = build_contract_class(class)?;
                ContractClass::V0(class)
            }
        })
    }
}

fn build_contract_class(
    class: DeprecatedContractClass,
) -> Result<ContractClassV0, Error> {
    let program = decode_program(class.program.as_ref())?;

    let mut class = serde_json::to_value(class)?;
    class["program"] = serde_json::from_str(&program)?;
    let json = serde_json::to_string_pretty(&class)?;

    let class = ContractClassV0::try_from_json_string(&json)?;
    Ok(class)
}

fn decode_program(program: &str) -> Result<String, Error> {
    let program = decode_base64(program)?;
    let program = decompress(&program)?;
    Ok(program)
}

fn decode_base64(input: &str) -> Result<Vec<u8>, Error> {
    use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
    let result = BASE64.decode(input)?;
    Ok(result)
}

fn decompress(input: &[u8]) -> Result<String, Error> {
    use flate2::read::GzDecoder;
    use std::io::prelude::*;
    let mut gz = GzDecoder::new(input);
    let mut result = String::new();
    gz.read_to_string(&mut result)?;
    Ok(result)
}
