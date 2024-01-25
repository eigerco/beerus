use beerus_core::storage_proofs::StorageProof;
use rstest::{fixture, rstest};
use starknet::macros::felt;
use starknet_crypto::FieldElement;

const TESTING_STATE_ROOT: FieldElement = felt!("0x11d7289401f12bdbbfcf890cf531dd13e215d68fa700b82b08220dc75c24f54");
const TESTING_CONTRACT_ADDR: FieldElement = felt!("0x49d36570d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7");
const TESTING_STORAGE_KEY: FieldElement = felt!("0xd4daccb5bc077d40279ee559dc950ff0e5a7d1e139b3e3ab7e1b8dd8b997a7");
const TESTING_BALANCE: FieldElement = felt!("0x17e3b52ef2aa6a");

struct ProofData {
    root: FieldElement,
    addr: FieldElement,
    key: FieldElement,
    value: FieldElement,
}

#[fixture]
fn proof_data() -> ProofData {
    ProofData {
        root: TESTING_STATE_ROOT,
        addr: TESTING_CONTRACT_ADDR,
        key: TESTING_STORAGE_KEY,
        value: TESTING_BALANCE,
    }
}

#[rstest]
fn verify_valid_storage_proof(proof_data: ProofData) {
    let mut proof: StorageProof = serde_json::from_str(PROOF).unwrap();

    let res = proof.verify(proof_data.root, proof_data.addr, proof_data.key, proof_data.value);
    assert!(res.is_ok());
}

#[rstest]
fn invalid_value_storage_proof(proof_data: ProofData) {
    let mut proof: StorageProof = serde_json::from_str(PROOF).unwrap();

    let bad_value = proof_data.value + FieldElement::ONE;
    let res = proof.verify(proof_data.root, proof_data.addr, proof_data.key, bad_value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_key_storage_proof(proof_data: ProofData) {
    let mut proof: StorageProof = serde_json::from_str(PROOF).unwrap();

    let bad_key = proof_data.key + FieldElement::ONE;
    let res = proof.verify(proof_data.root, proof_data.addr, bad_key, proof_data.value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_addr_storage_proof(proof_data: ProofData) {
    let mut proof: StorageProof = serde_json::from_str(PROOF).unwrap();

    let bad_addr = proof_data.addr + FieldElement::ONE;
    let res = proof.verify(proof_data.root, bad_addr, proof_data.key, proof_data.value);
    assert!(res.is_err());
}

#[rstest]
fn invalid_root_storage_proof(proof_data: ProofData) {
    let mut proof: StorageProof = serde_json::from_str(PROOF).unwrap();

    let bad_root = proof_data.root + FieldElement::ONE;
    let res = proof.verify(bad_root, proof_data.addr, proof_data.key, proof_data.value);
    assert!(res.is_err());
}

const PROOF: &str = r###"
{
    "class_commitment": "0x1c77bc2aaa0f0b18d477a417934193907e3413cefef1f140fc7de79a1e156e7",
    "contract_data": {
        "class_hash": "0xd0e183745e9dae3e4e78a8ffedcce0903fc4900beace4e0abf192d4c202da3",
        "contract_state_hash_version": "0x0",
        "nonce": "0x0",
        "root": "0x83d3395c0eb1b0976fb45a84d2509bc00d4845abaf879a65906ebb8caf0f8e",
        "storage_proofs": [
            [
                {
                    "binary": {
                        "left": "0x3b4c2674bc40f568dbae60fa428abfced83ddeaf8c20e18af27381b0ac2dc52",
                        "right": "0x504d036832daebcf9617826611b55c0650463a1287e672d90eb2eb4979aae6f"
                    }
                },
                {
                    "binary": {
                        "left": "0x66197253670c3c29e82356877cda22f37e95825ae61ae87612a695ada93d014",
                        "right": "0x52723c3011de99a217b0b13264800c23b397744a05e0d89a1dcaa29d25080af"
                    }
                },
                {
                    "binary": {
                        "left": "0x694c3e3bd76652c0e256029684e3bcadd7b6369193b7dcc5ed3512b83c83dd8",
                        "right": "0x61e88ab8ed9d0f2ad01f847f5b6aa57f58198b8a8af2d05f8255b85daabd873"
                    }
                },
                {
                    "binary": {
                        "left": "0x68e6ac119bb599708e0eb88c86cfa7030ed7ad420dfe7026a7fb3ab107751db",
                        "right": "0x5c4f68c49a25cad515574c64b14497f13fffe1e0468755bdbac8351ab82fd13"
                    }
                },
                {
                    "binary": {
                        "left": "0x6490d49899fa3bb254b07fff0b252b95344f6f6e0775aed45ca3ae325eb3869",
                        "right": "0x726a89839da2b35c487cd236a9ddbec6aaef6673f7fe7c6a310c2b7bdd401a"
                    }
                },
                {
                    "binary": {
                        "left": "0x1c0d141b806e782c3f640ea4c1c51b02829ad850773cf690ec060b82b9bb08e",
                        "right": "0x6af03b82561c81f5513fb26b8ab45d27574d7b03e7b256f2958403864dcb85d"
                    }
                },
                {
                    "binary": {
                        "left": "0x70538aab469898adddfc3f9e594dcfce7a9a70f2eb888d2a0da7a2bff98b33d",
                        "right": "0x22f1ae3668ff76e1b2a3f688db121f33c48854597526491091535dd2787376e"
                    }
                },
                {
                    "binary": {
                        "left": "0x54425759a6cbcebc76c16c3577992352fe6ed5999c77fe386bcf9a92f5fa07a",
                        "right": "0x49a5f76a79ab1d62d3790c9728d8a05d9855587170eb2c300264330dcb8e1ba"
                    }
                },
                {
                    "binary": {
                        "left": "0x5aa18afc99215b70f4d92dcae306374e9914f6878b90dd390c562ce0640b73e",
                        "right": "0x28bc3a26bfd3b0bf72499648da986619d06bb12ebdd5715acb0e8a92c376f6c"
                    }
                },
                {
                    "binary": {
                        "left": "0x4618cd40579132ec033621582bfba781dd82f58bc2c6514c2738dde15b60a8",
                        "right": "0x14f88e2864f81792a90ceda6199bb0bbc04de60014545546ea7be40b32112c4"
                    }
                },
                {
                    "binary": {
                        "left": "0x2b4dd64ae9f2b5b4cdc4746b00c8f4d1cdf3e0aed40f8f3d71d9b1de7f13585",
                        "right": "0x736f3da00ab9c033d85852dd02099e74f3735ba0f514834e67c35df25bd2520"
                    }
                },
                {
                    "binary": {
                        "left": "0x2f2ae562d9c6277c8ea31e4dbde7ccf05dcc6a6c8883f5428257f0e711399ee",
                        "right": "0x7d85f71d8880270060f779212c0665d81db4d5bd563e763cdfc9c667fe7e3a2"
                    }
                },
                {
                    "binary": {
                        "left": "0x3cc9ff9365a925f0b31b31eef09f999c2878dbffa9d218ab0de407f60f0098",
                        "right": "0x6eafe2a505b0a6db99e024676711f889e11fa5d787acfade66549a56fe36f03"
                    }
                },
                {
                    "binary": {
                        "left": "0x63df9d02184edb8a53376e16bb3dfd7db89705ccbaef6b11d6784b397cb7c3b",
                        "right": "0x55a17515de7f65409a30515b2a160cc78f1d8d492b4549e7700cadda36127b4"
                    }
                },
                {
                    "binary": {
                        "left": "0x28b2d6e68eafab17e6b2af3c711fff86a91165da3aaf1977fc0cf6274f2491e",
                        "right": "0x1da1f8b801efa45608523f254c6fa283223a3ebfb9bf44f5811f760c9ad741"
                    }
                },
                {
                    "binary": {
                        "left": "0x370032893b9c6bcfa11c117a27a8b3e352f59029ec5efa41c0ee1c7f3c0d2fb",
                        "right": "0x2cbfb86007a463c6ce4e7865caad3264aaf2a8cb53ef5a3a94c0e5c8eed50dd"
                    }
                },
                {
                    "binary": {
                        "left": "0x41e70a365a75ab6b15489d50a2e0dd042871f5081ae31fe1a967f5197cfb76c",
                        "right": "0x7772b902f79f83aba1501bcc63a834775c3072b298bda5d9d16faa60c5f5b26"
                    }
                },
                {
                    "binary": {
                        "left": "0x4167ad67664dbffce92ee9be7d3ca5280416bce7b33e95c6ea3ca4d282ed4d0",
                        "right": "0x563c3ff40404ac1d0c49c1bd02dfdf9998cad9272bb0bb2dc4a0e86a3cb004e"
                    }
                },
                {
                    "binary": {
                        "left": "0x3737331544acd8c1e0c6bf275ff7fd504f83123ecec9f2ecbe2c557929234b6",
                        "right": "0x18f982b67afd7e18417a44e2bd570da0de2e1add2aefcdb7569d2b6c3915ce1"
                    }
                },
                {
                    "binary": {
                        "left": "0x20f75e88b21ac732dcd2e412cdfc91a7fff645808f7302ade84db5cc20fa32e",
                        "right": "0x50a740757e0386cb0173ecc648db9c313642d62ccf0e77373aec02ab54899e0"
                    }
                },
                {
                    "edge": {
                        "child": "0x4704742f41c251c4c3bf76b7b0805b8207b196fcc2aadccf3fa5025d7af86e1",
                        "path": {
                            "len": 1,
                            "value": "0x1"
                        }
                    }
                },
                {
                    "binary": {
                        "left": "0x2b46e7468e41585bb9c93ae68c1d39d5d4640e52cf6e916cd9a9a0471fce1b4",
                        "right": "0x119cb8facf960980502cc38e31b7df76b8e534edfa43e1256dfc05fd810a95e"
                    }
                },
                {
                    "edge": {
                        "child": "0x4201319390b8cf22498e190bf98d9822574d05130b4e053b24d308ecaf97f1d",
                        "path": {
                            "len": 1,
                            "value": "0x0"
                        }
                    }
                },
                {
                    "binary": {
                        "left": "0x25c814d3ba44a2c4e3a187cf608e9dba69e6f93a04499d6b4b3bbf056f0ba50",
                        "right": "0x312f75949185badac6930d885a2700889e3d3339095b0cab4934ee46029110d"
                    }
                },
                {
                    "binary": {
                        "left": "0x10c6b434ccd46cfcafee9886f9f937b32c50a57eb1e5aab212eb48e2f5fc05a",
                        "right": "0x52900e65d22d98fa40216ffdd1d172e0e2536cc62a462684520fa037f779ca3"
                    }
                },
                {
                    "binary": {
                        "left": "0x36056d0732ef5ec2fe60266518c7ca9cbb286fb5adf624379ad35e2ccafc0b5",
                        "right": "0x60b674f786ea7b35d589fe9acfd4509cecb738f59148837fe79c4a14117dbd2"
                    }
                },
                {
                    "edge": {
                        "child": "0x17e3b52ef2aa6a",
                        "path": {
                            "len": 225,
                            "value": "0xb5bc077d40279ee559dc950ff0e5a7d1e139b3e3ab7e1b8dd8b997a7"
                        }
                    }
                }
            ]
        ]
    },
    "contract_proof": [
        {
            "binary": {
                "left": "0x5059c49a7b1a43abf16ea6f498f8da14c2431c3266b6b6bf0532b2067f108dc",
                "right": "0x5d55d90c9f1b71672adc074dd4b0e6a14190ea3bde514cd40e6bb9881c14d09"
            }
        },
        {
            "binary": {
                "left": "0x3baee5ac6e6d3f9748b147426f82c5e7be950f495c095494102c854b6c52e9c",
                "right": "0x6f14a6c1025fe9ae7ecac6c50efa2059470508c726ff384b0f89bdbb00f7eb5"
            }
        },
        {
            "binary": {
                "left": "0x1976a308b98471e67121f3a0c676ee09c371f24fe2cf428c130516f798572be",
                "right": "0x596abd8b6cbef909be6d863b853f0bfecfb9d84ed840bee6124937e43f3258e"
            }
        },
        {
            "binary": {
                "left": "0x4504906c47cafcbcfccd6a14e1499d6e8875af9e336bfc505bc242d23d4d6dd",
                "right": "0x1010b8084da28a53647a73cf07ce64989ec4698d6cc72ef139450e514d90432"
            }
        },
        {
            "binary": {
                "left": "0x13426ebe62dd2881205b039274bc6128573074fed9fa86359031c2a0ea03516",
                "right": "0x6561cc426f5f1d1b3278c880d4736e6c82a01f0fad38a99a8e7c0c9558788e2"
            }
        },
        {
            "binary": {
                "left": "0x3e3a818855cd65c528d18d2ce4a0cd508415350839d6cde79cebccb03152dd9",
                "right": "0x79263e48b9e11a0daaa307f4a2805c8cb74a9248ddf2046651579ae2ac016f"
            }
        },
        {
            "binary": {
                "left": "0x1fda237ebef52317c6231cefa4795fd363d15d9933893a556446270a80a51e0",
                "right": "0x41a89df8545ac205ccd543a191a6150a72e079f88063da2be01c770a287be3c"
            }
        },
        {
            "binary": {
                "left": "0x5b1a7cfff7bf479f08486d2476c2b9c0af9b98dc9f713f5296c1e19c4afde0b",
                "right": "0x70c276f4f92df9cc2c8029f34fd7fff037edf574bdb32d6a32bc30bc612c0b2"
            }
        },
        {
            "binary": {
                "left": "0x4904140cb36753fc3cb278e7ac3e70c1f5cc353b3bdf20650f422afc846e77",
                "right": "0x76f31e1062e3765231a7a475fdbf11474615e6696574add1ea55c61bafeb48f"
            }
        },
        {
            "binary": {
                "left": "0x4212f9fab035f14467cea0b04e3f9840e9a2593752d080cde41cf4ed33350a5",
                "right": "0x7af023b1e46a1bf838d34b1ec9095bdf1962e7a0e5a17843a5def4c758eaf6"
            }
        },
        {
            "binary": {
                "left": "0x112434b3f3a5a1bc07ab2712e04e78806d1a1e5759598db1c70a7eb908c9305",
                "right": "0x35276989deb73569a58a77ebe5124d89d85de6770c30db489e49b25445b2260"
            }
        },
        {
            "binary": {
                "left": "0x5a6eb2280d1f5037edab4c118a0f6b78477163a9a36abe5c0dbdacf7cc95bb0",
                "right": "0x5027d219f59a682fed69f0d9bd14c52f9a34c45cf9015c99c687115945fa8e2"
            }
        },
        {
            "binary": {
                "left": "0x32b79d2be79ac006f3c08dbd7e5b7f70b645d9dff8ca18c3090843ab3e49a1d",
                "right": "0xc265071b40c5099ea627266812931a54fed51b9d2ea7aeca9afc6abe6f93c4"
            }
        },
        {
            "binary": {
                "left": "0x36af3f809294ccd01eb37744b52c7ba303a61a2384b8db4fb7bcfa9776d5b97",
                "right": "0x19356d7d86192cf5ddd65d1a38a986c49827b37037c1576c641d11bc1ed1da1"
            }
        },
        {
            "binary": {
                "left": "0x6be721046fab959e6a9ed5a60a90e578e72bb79f011f40eb68e02c8f0c2b3a7",
                "right": "0x14aefd8d338e682b43d2bbca75b6bc99697334bfa1bf9789dbdc41c59ed5261"
            }
        },
        {
            "binary": {
                "left": "0x4dd59fd6e0482d322226e60374cb2cc41de2f1fe296da05526fe6deb28bbe3b",
                "right": "0x3c5edd966866336ee05b18534a00aab9f78d81eeb600320483e68b0b1666ec"
            }
        },
        {
            "binary": {
                "left": "0x7078856c6be2b96e7da0334307644fee3dd2c1c061bf79ded36de1b09619959",
                "right": "0x63b52d81acc9d5cff0839e50d0ac1755bc812775515a4ee9679cc4a7c5eb6c"
            }
        },
        {
            "binary": {
                "left": "0x3be1f608feb9d358a0cad74dd8eadfbe9169b16f30e386a527ae9f5ddaaecbb",
                "right": "0x1f01d83ed3e0722f5e9c5ef41827f8d7bf97ec3422ede3966fd0d1ffea862b9"
            }
        },
        {
            "binary": {
                "left": "0x634e6343b58e9aaf9fdf9934a3a0029e55fee04740445ea37ba62487e7a378d",
                "right": "0x55bc73901e48a005706bb19aeb4b62fd5ad30b8e2d22ee154560e4fc46ee71d"
            }
        },
        {
            "binary": {
                "left": "0x3a87d4c5a8d4a55a3518c0714f01c7b7febd76d895e7d6742595eba052477c4",
                "right": "0x1b20e7165783037b301a0151d26a7c2336502a402aa0b2de7c9d903cec351ca"
            }
        },
        {
            "binary": {
                "left": "0x17477b2e441571dc4e1ced1b015ab8c2c66f51cfb0f89b1668b827e485fab4",
                "right": "0x3f630886b0c6915c388a16511e8dc86c6b70580e43dead0428c213be057d713"
            }
        },
        {
            "binary": {
                "left": "0x58aca47a46764400304a59b4988c2ca69aee0328d3626de2bb439540a4f80d5",
                "right": "0x4c4dffc5358ffbc896d09168cf2d1900f3df56c8b8f2f913cd08465dceec232"
            }
        },
        {
            "edge": {
                "child": "0x2df98a8a908e7045fca123b7dfb86e7c632c79c50b1e45610932ebdd9dd7787",
                "path": {
                    "len": 229,
                    "value": "0x170d4e46f48e99674bd3fcc84644ddd6b96f7c741b1562b82f9e004dc7"
                }
            }
        }
    ],
    "state_commitment": "0x11d7289401f12bdbbfcf890cf531dd13e215d68fa700b82b08220dc75c24f54"
}
"###;