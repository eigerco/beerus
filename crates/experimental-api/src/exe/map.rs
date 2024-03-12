use blockifier::execution::contract_class::{ContractClassV0, ContractClassV1};

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
        let json = serde_json::to_string(&value).unwrap();
        tracing::trace!("Trying to build ContractClass from: {json}");

        Ok(match value {
            gen::GetClassResult::ContractClass(_) => {
                let class = ContractClassV1::try_from_json_string(&json)?;
                ContractClass::V1(class)
            }
            gen::GetClassResult::DeprecatedContractClass(class) => {
                // TODO: clean this up!
                let program = {
                    let program = class.program.as_ref();

                    use base64::{
                        engine::general_purpose::STANDARD as BASE64,
                        Engine as _,
                    };
                    let program = BASE64.decode(program).unwrap();

                    use flate2::read::GzDecoder;
                    use std::io::prelude::*;
                    let mut gz = GzDecoder::<&[u8]>::new(program.as_ref());
                    let mut result = String::new();
                    gz.read_to_string(&mut result).unwrap();

                    // println!("result={result}");
                    result
                };

                let mut class = serde_json::to_value(class).unwrap();
                class["program"] = serde_json::from_str(&program).unwrap();
                let json = serde_json::to_string_pretty(&class).unwrap();
                // println!("class={json}");

                let class = ContractClassV0::try_from_json_string(&json)?;
                ContractClass::V0(class)
            }
        })
    }
}
