use blockifier::execution::contract_class::ContractClassV0;
use cairo_lang_starknet_classes::casm_contract_class::CasmContractClass;
use cairo_lang_starknet_classes::contract_class::ContractClass as CairoContractClass;

use self::gen::DeprecatedContractClass;

use super::*;

impl TryFrom<gen::Felt> for StarkFelt {
    type Error = Error;
    fn try_from(felt: gen::Felt) -> Result<Self, Self::Error> {
        let felt = felt.as_ref().as_str();
        let felt = StarkFelt::from_hex_unchecked(felt);
        Ok(felt)
    }
}

impl TryFrom<&StarkFelt> for gen::Felt {
    type Error = Error;
    fn try_from(felt: &StarkFelt) -> Result<Self, Self::Error> {
        let hex = hex::encode(felt.to_bytes_be());
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
        Ok(match value {
            gen::GetClassResult::ContractClass(ref class) => {
                let mut json = serde_json::to_value(&value)?;
                if let Some(abi) = class.abi.as_ref() {
                    let abi: serde_json::Value = serde_json::from_str(abi)?;
                    json["abi"] = abi;
                }
                let contract_class: CairoContractClass =
                    serde_json::from_value(json)?;
                let casm_contract_class =
                    CasmContractClass::from_contract_class(
                        contract_class,
                        /*add_pythonic_hints=*/ false,
                        /*max_bytecode_size=*/ u16::MAX as usize,
                    )?;
                let class = casm_contract_class
                    .try_into()
                    .map_err(|e| Error::Program(format!("{e}")))?;

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
    let json = serde_json::to_string(&class)?;

    let class = ContractClassV0::try_from_json_string(&json)
        .map_err(|e| Error::Program(format!("{e}")))?;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_from_starknet_felt_into_gen_felt() {
        let stark_felt = StarkFelt::from_hex("0x1").unwrap();
        let gen_felt: gen::Felt = stark_felt.try_into().unwrap();
        assert_eq!(gen_felt.as_ref(), "0x1");
    }

    #[test]
    fn test_decode_base64_returns_error_for_invalid_input() {
        let result = decode_base64("}");
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_decode_base64_returns_decoded_text_for_valid_input() {
        let result = decode_base64("ZWlnZXI=").unwrap();
        assert_eq!(String::from_utf8(result).unwrap(), "eiger");
    }

    #[test]
    fn test_decompress_for_valid_input() {
        let text = decode_base64("H4sIAAAAAAAAA0vNTE8tAgD+5cc6BQAAAA==").unwrap();
        let result = decompress(&text).unwrap();
        assert_eq!(result, "eiger");
    }

    #[test]
    fn test_decompress_for_invalid_input() {
        let input = b"\xFF\xFF";
        let result = decompress(input);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_decode_program() {
        assert!(matches!(decode_program("invalid_base_64}").unwrap_err(), Error::Base64(..)));
        // "//8=" is base64 encoded \xFF\xFF which is invalid UTF-8
        assert!(matches!(decode_program("//8=").unwrap_err(), Error::Io(..)));
        assert_eq!(decode_program("H4sIAAAAAAAAA0vNTE8tAgD+5cc6BQAAAA==").unwrap(), "eiger");
    }

    #[test]
    fn test_try_from_get_class_result_for_contract_class() {
        let contract_class = r#"{
            "program": "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",
            "entry_points_by_type": {
              "CONSTRUCTOR": [
                {
                  "offset": "0xA1",
                  "selector": "0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194"
                }
              ],
              "EXTERNAL": [
                {
                  "offset": "0xD0",
                  "selector": "0x0"
                }
              ],
              "L1_HANDLER": [
                {
                  "offset": "0xE9",
                  "selector": "0x0"
                }
              ]
            },
            "abi": [
              {
                "type": "event",
                "name": "Upgraded",
                "keys": [],
                "data": [
                  {
                    "name": "implementation",
                    "type": "felt"
                  }
                ]
              },
              {
                "type": "event",
                "name": "AdminChanged",
                "keys": [],
                "data": [
                  {
                    "name": "previousAdmin",
                    "type": "felt"
                  },
                  {
                    "name": "newAdmin",
                    "type": "felt"
                  }
                ]
              },
              {
                "type": "constructor",
                "name": "constructor",
                "inputs": [
                  {
                    "name": "implementation_hash",
                    "type": "felt"
                  },
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_len",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": []
              },
              {
                "type": "function",
                "name": "__default__",
                "inputs": [
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_size",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": [
                  {
                    "name": "retdata_size",
                    "type": "felt"
                  },
                  {
                    "name": "retdata",
                    "type": "felt*"
                  }
                ]
              },
              {
                "type": "l1_handler",
                "name": "__l1_default__",
                "inputs": [
                  {
                    "name": "selector",
                    "type": "felt"
                  },
                  {
                    "name": "calldata_size",
                    "type": "felt"
                  },
                  {
                    "name": "calldata",
                    "type": "felt*"
                  }
                ],
                "outputs": []
                }]
              }"#;

              assert!(matches!(
                ContractClass::try_from(
                        gen::GetClassResult::DeprecatedContractClass(serde_json::from_str(contract_class).unwrap())
                    ).unwrap(),
                ContractClass::V0(..))
            );

              let sierra = r#"
{
    "abi": "[{\"type\": \"impl\", \"name\": \"HelloStarknetImpl\", \"interface_name\": \"starknet_call_self_function::IHelloStarknet\"}, {\"type\": \"interface\", \"name\": \"starknet_call_self_function::IHelloStarknet\", \"items\": [{\"type\": \"function\", \"name\": \"increase_balance\", \"inputs\": [{\"name\": \"amount\", \"type\": \"core::felt252\"}], \"outputs\": [], \"state_mutability\": \"external\"}, {\"type\": \"function\", \"name\": \"get_balance\", \"inputs\": [], \"outputs\": [{\"type\": \"core::felt252\"}], \"state_mutability\": \"view\"}]}, {\"type\": \"event\", \"name\": \"starknet_call_self_function::HelloStarknet::Event\", \"kind\": \"enum\", \"variants\": []}]",
    "contract_class_version": "0.1.0",
    "entry_points_by_type": {
      "CONSTRUCTOR": [],
      "EXTERNAL": [
        {
          "function_idx": 0,
          "selector": "0x362398bec32bc0ebb411203221a35a0301193a96f317ebe5e40be9f60d15320"
        },
        {
          "function_idx": 1,
          "selector": "0x39e11d48192e4333233c7eb19d10ad67c362bb28580c604d67884c85da39695"
        }
      ],
      "L1_HANDLER": []
    },
    "sierra_program": [
      "0x1",
      "0x6",
      "0x0",
      "0x2",
      "0x8",
      "0x5",
      "0xaa",
      "0x56",
      "0x1a",
      "0x52616e6765436865636b",
      "0x800000000000000100000000000000000000000000000000",
      "0x53746f726167654261736541646472657373",
      "0x800000000000000700000000000000000000000000000000",
      "0x537472756374",
      "0x800000000000000700000000000000000000000000000002",
      "0x0",
      "0x145cc613954179acf89d43c94ed0e091828cbddcca83f5b408785785036d36d",
      "0x1",
      "0x436f6e7374",
      "0x800000000000000000000000000000000000000000000002",
      "0xe",
      "0x2",
      "0x4661696c656420746f20646573657269616c697a6520706172616d202331",
      "0x4f7574206f6620676173",
      "0x4172726179",
      "0x800000000000000300000000000000000000000000000001",
      "0x536e617073686f74",
      "0x800000000000000700000000000000000000000000000001",
      "0x5",
      "0x1baeba72e79e9db2587cf44fedb2f3700b2075a5e8e39a562584862c4b71f62",
      "0x6",
      "0x2ee1e2b1b89f8c495f200e4956278a4d47395fe262f27b52e5865c9524c08c3",
      "0x7",
      "0xa",
      "0x753332",
      "0x53746f7261676541646472657373",
      "0x31448060506164e4d1df7635613bacfbea8af9c3dc85ea9a55935292a4acddc",
      "0x416d6f756e742063616e6e6f742062652030",
      "0x66656c74323532",
      "0x4e6f6e5a65726f",
      "0x4275696c74696e436f737473",
      "0x53797374656d",
      "0x800000000000000f00000000000000000000000000000001",
      "0x16a4c8d7c05909052238a862d8cc3e7975bf05a07b3a69c6b28951083a6d672",
      "0x800000000000000300000000000000000000000000000003",
      "0x12",
      "0x456e756d",
      "0x9931c641b913035ae674b400b61a51476d506bbe8bba2ff8a6272790aba9e6",
      "0x8",
      "0x13",
      "0x496e70757420746f6f206c6f6e6720666f7220617267756d656e7473",
      "0x800000000000000700000000000000000000000000000003",
      "0x11c6d8087e00642489f92d2821ad6ebd6532ad1a3b6d12833da6d6810391511",
      "0x16",
      "0x426f78",
      "0x4761734275696c74696e",
      "0x40",
      "0x7265766f6b655f61705f747261636b696e67",
      "0x77697468647261775f676173",
      "0x6272616e63685f616c69676e",
      "0x7374727563745f6465636f6e737472756374",
      "0x656e61626c655f61705f747261636b696e67",
      "0x73746f72655f74656d70",
      "0x61727261795f736e617073686f745f706f705f66726f6e74",
      "0x756e626f78",
      "0x72656e616d65",
      "0x656e756d5f696e6974",
      "0x17",
      "0x6a756d70",
      "0x7374727563745f636f6e737472756374",
      "0x656e756d5f6d61746368",
      "0x64697361626c655f61705f747261636b696e67",
      "0x64726f70",
      "0x18",
      "0x61727261795f6e6577",
      "0x636f6e73745f61735f696d6d656469617465",
      "0x15",
      "0x61727261795f617070656e64",
      "0x14",
      "0x19",
      "0x11",
      "0x6765745f6275696c74696e5f636f737473",
      "0x10",
      "0x77697468647261775f6761735f616c6c",
      "0x647570",
      "0x66656c743235325f69735f7a65726f",
      "0xd",
      "0xf",
      "0x73746f726167655f626173655f616464726573735f636f6e7374",
      "0x206f38f7e4f15e87567361213c28f235cccdaa1d7fd34c9db1dfe9489c6a091",
      "0xc",
      "0x736e617073686f745f74616b65",
      "0x73746f726167655f616464726573735f66726f6d5f62617365",
      "0x9",
      "0xb",
      "0x73746f726167655f726561645f73797363616c6c",
      "0x66656c743235325f616464",
      "0x73746f726167655f77726974655f73797363616c6c",
      "0x4",
      "0x3",
      "0xf9",
      "0xffffffffffffffff",
      "0x92",
      "0x82",
      "0x27",
      "0x1b",
      "0x1c",
      "0x1d",
      "0x1e",
      "0x1f",
      "0x20",
      "0x74",
      "0x21",
      "0x22",
      "0x23",
      "0x3c",
      "0x24",
      "0x25",
      "0x26",
      "0x28",
      "0x29",
      "0x2a",
      "0x6b",
      "0x2b",
      "0x2c",
      "0x2d",
      "0x2e",
      "0x2f",
      "0x30",
      "0x31",
      "0x32",
      "0x33",
      "0x34",
      "0x35",
      "0x66",
      "0x36",
      "0x37",
      "0x38",
      "0x39",
      "0x3a",
      "0x3b",
      "0x3d",
      "0x3e",
      "0x61",
      "0x3f",
      "0x41",
      "0x42",
      "0x43",
      "0x44",
      "0x45",
      "0x46",
      "0x47",
      "0x48",
      "0x49",
      "0x4a",
      "0x4b",
      "0x4c",
      "0x4d",
      "0x4e",
      "0x4f",
      "0x50",
      "0x51",
      "0x52",
      "0x53",
      "0x54",
      "0x55",
      "0x56",
      "0x57",
      "0x58",
      "0x59",
      "0x5a",
      "0x5b",
      "0x5c",
      "0xeb",
      "0xb5",
      "0xde",
      "0xd5",
      "0xa0",
      "0x91a",
      "0x140913120c0911100f0d0c090b0a0e0d0c090b0a0909080706050403020100",
      "0x90b0a09091c070605041b041a070d19090b0a180917070605160915070605",
      "0x280927072426140925091707240523072205022104200c09131f041e1d0d0c",
      "0x732073130022f0c09132e2d090c092c072b26170722052a0d0c090b0a2909",
      "0x36070d3b090d3a0c0909390c0909380c090937070909360735180909340733",
      "0x94016090940073f3b09093e090d3b090d3a2d09093d073c3b090936160909",
      "0x3a2909093d2509093d0c0909450c090936440909430c0909420c0909404109",
      "0x74d0c09094c074b4a0909360749460909364809093647090936090d46090d",
      "0x9093452090940520909535209093d510d09504f090940140909364e090943",
      "0x9401409095307590758075756090936190909365509094307540909093952",
      "0x909405b0909432d0909405a090943070d46090d3a2809093d1809093d1409",
      "0x5d0d09070d0907075d090707075c0d0909340d0909400d0909530d09093d18",
      "0x5a091407075d09075a0728095d095b095b07075d09070d0718160d5e145a0d",
      "0x752095d0919091807075d09070d0756090c19550d5d0d280916075a095d09",
      "0x4a095d090c0956074f095d09550919070c095d094e0955074e095d09520928",
      "0x5609190725095d0948090c0748095d09074e07075d09070d07072909075207",
      "0x1607075d09070d0746095f29095d0d4a094f074a095d09250956074f095d09",
      "0x2507075d0944094807075d09074a07075d09070d073b09602d440d5d0d4f09",
      "0x947092d0747095d0907440741095d09074607075d0929092907075d092d09",
      "0x9000762095d0900610d470761095d0907410700095d0947410d3b0747095d",
      "0x63070d095d090d09620714095d09140961075a095d095a09140763095d0962",
      "0x75d093b094807075d09074a07075d09070d07630d145a5a0963095d096309",
      "0x769680d6766650d5d0d64145a5b660764095d096409650764095d09076407",
      "0x9690765095d09650914076a290d5d0929096807075d09075a07075d09070d",
      "0x9076a076d095d09074607075d0929092907075d09070d076c096b075d0d6a",
      "0x962076f095d096609610760095d096e6d0d3b076e095d096e092d076e095d",
      "0x5d096c096d07075d09070d0707720907520771095d0960096c0770095d090d",
      "0x5d097509700776750d5d0974096f0774095d097309600773095d09076e0707",
      "0x95d0907750779095d097809740778095d097709730777095d097609710707",
      "0x7e7d7c7b5b5d0d797a0d665a780779095d09790977077a095d097a0976077a",
      "0x5d097d092d0783095d098209740782095d09076e07075d09070d0781807f5b",
      "0x96b0976077b095d097b0961076b095d0907750784095d09297d0d79077d09",
      "0x850d5d0d84836b7c7b147a0784095d0984092d0783095d09830977076b095d",
      "0x5d098a097b078a095d09074607075d09074a07075d09070d078930885b8786",
      "0x98e0980078e095d098d097f078d095d098c097d07075d098b097c078c8b0d",
      "0x8f09630786095d098609620785095d098509610765095d09650914078f095d",
      "0x95d09300962076f095d0988096107075d09070d078f8685655a098f095d09",
      "0x96107075d0929092907075d09070d0707720907520771095d0989096c0770",
      "0x9074107075d09074a0771095d0981096c0770095d09800962076f095d097f",
      "0x9610765095d096509140792095d099109000791095d0971900d470790095d",
      "0x9070d0792706f655a0992095d099209630770095d09700962076f095d096f",
      "0x94095d0994092d0794095d0907810793095d09074607075d0929092907075d",
      "0x5d099709000797095d0995960d470796095d0907410795095d0994930d3b07",
      "0x9980963070d095d090d09620769095d096909610768095d09680914079809",
      "0x94807075d0946098207075d09074a07075d09070d07980d69685a0998095d",
      "0x9a990d3b079a095d099a092d079a095d0907830799095d09074607075d094f",
      "0x914079e095d099d0900079d095d099b9c0d47079c095d090741079b095d09",
      "0x5a099e095d099e0963070d095d090d09620714095d09140961075a095d095a",
      "0xa0095d090781079f095d09074607075d095b098407075d09070d079e0d145a",
      "0x5d09a1a20d4707a2095d09074107a1095d09a09f0d3b07a0095d09a0092d07",
      "0x90d09620718095d091809610716095d0916091407a4095d09a3090007a309",
      "0xd5d0d09070d0907075d09070707a40d18165a09a4095d09a40963070d095d",
      "0x916075a095d095a09140728095d095b095b07075d09070d0718160da5145a",
      "0x4607075d0919092507075d0955094807075d09070d075609a619550d5d0d28",
      "0x41070c095d094e520d3b074e095d094e092d074e095d0907440752095d0907",
      "0x75a095d095a09140748095d094a0900074a095d090c4f0d47074f095d0907",
      "0xd07480d145a5a0948095d09480963070d095d090d09620714095d09140961",
      "0x145a5b660725095d092509650725095d09076407075d0956094807075d0907",
      "0x41095d093b096b073b095d09076e07075d09070d072d440da746290d5d0d25",
      "0x95d096109730761095d0900098807075d094709860700470d5d0941098507",
      "0x95d096309770764095d096409760764095d0907750763095d096209740762",
      "0x9070d076c6a695ba86866655b5d0d63640d465a780729095d092909140763",
      "0x96e097b076e095d09686d0d3b0768095d0968092d076d095d09074607075d",
      "0x7109800771095d0970097f0770095d096f097d07075d0960097c076f600d5d",
      "0x9630766095d096609620765095d096509610729095d092909140773095d09",
      "0x5d096c740d470774095d09074107075d09070d07736665295a0973095d0973",
      "0x96a09620769095d096909610729095d092909140776095d09750900077509",
      "0x777095d09074607075d09070d07766a69295a0976095d09760963076a095d",
      "0x77a095d0907410779095d0978770d3b0778095d0978092d0778095d090781",
      "0x95d092d09610744095d09440914077c095d097b0900077b095d09797a0d47",
      "0x8407075d09070d077c0d2d445a097c095d097c0963070d095d090d0962072d",
      "0x7d0d3b077f095d097f092d077f095d090781077d095d09074607075d095b09",
      "0x140783095d098209000782095d0980810d470781095d0907410780095d097f",
      "0x983095d09830963070d095d090d09620718095d091809610716095d091609",
      "0x464847075a184847075a075b0d0907464847075a184847075a0d830d18165a",
      "0xa95b0d0907"
    ]
  }
              "#;
              let ret: gen::GetClassResult = serde_json::from_str(sierra).unwrap();
              assert!(matches!(ContractClass::try_from(ret).unwrap(), ContractClass::V1(..)));



    }

    #[test]
    fn test_build_contract_class() {
        let contract_class = r#"{
    "program": "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",
    "entry_points_by_type": {
      "CONSTRUCTOR": [
        {
          "offset": "0xA1",
          "selector": "0x28ffe4ff0f226a9107253e17a904099aa4f63a02a5621de0576e5aa71bc5194"
        }
      ],
      "EXTERNAL": [
        {
          "offset": "0xD0",
          "selector": "0x0"
        }
      ],
      "L1_HANDLER": [
        {
          "offset": "0xE9",
          "selector": "0x0"
        }
      ]
    },
    "abi": [
      {
        "type": "event",
        "name": "Upgraded",
        "keys": [],
        "data": [
          {
            "name": "implementation",
            "type": "felt"
          }
        ]
      },
      {
        "type": "event",
        "name": "AdminChanged",
        "keys": [],
        "data": [
          {
            "name": "previousAdmin",
            "type": "felt"
          },
          {
            "name": "newAdmin",
            "type": "felt"
          }
        ]
      },
      {
        "type": "constructor",
        "name": "constructor",
        "inputs": [
          {
            "name": "implementation_hash",
            "type": "felt"
          },
          {
            "name": "selector",
            "type": "felt"
          },
          {
            "name": "calldata_len",
            "type": "felt"
          },
          {
            "name": "calldata",
            "type": "felt*"
          }
        ],
        "outputs": []
      },
      {
        "type": "function",
        "name": "__default__",
        "inputs": [
          {
            "name": "selector",
            "type": "felt"
          },
          {
            "name": "calldata_size",
            "type": "felt"
          },
          {
            "name": "calldata",
            "type": "felt*"
          }
        ],
        "outputs": [
          {
            "name": "retdata_size",
            "type": "felt"
          },
          {
            "name": "retdata",
            "type": "felt*"
          }
        ]
      },
      {
        "type": "l1_handler",
        "name": "__l1_default__",
        "inputs": [
          {
            "name": "selector",
            "type": "felt"
          },
          {
            "name": "calldata_size",
            "type": "felt"
          },
          {
            "name": "calldata",
            "type": "felt*"
          }
        ],
        "outputs": []
        }]
      }"#;
      let class: DeprecatedContractClass = serde_json::from_str(contract_class).unwrap();

      let result = build_contract_class(class);

      assert!(result.is_ok())

    }

}