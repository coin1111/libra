pub use bridge_escrow_mod::*;
mod bridge_escrow_mod {
    #![allow(dead_code)]
    #![allow(unused_imports)]
    use ethers::{
        contract::{
            builders::{ContractCall, Event},
            Contract, Lazy,
        },
        core::{
            abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
            types::*,
        },
        providers::JsonRpcClient,
        signers::{Client, Signer},
    };
    pub static BRIDGEESCROW_ABI: Lazy<Abi> = Lazy::new(|| {
        serde_json :: from_str ( "[\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"olTokenAddr\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address\",\n        \"name\": \"executorAddr\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"constructor\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"_to\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"_value\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"_data\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"name\": \"call\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes\",\n        \"name\": \"\",\n        \"type\": \"bytes\"\n      }\n    ],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"closeTransferAccountSender\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"receiver_address\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccount\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"receiver_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccountAux\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"amount\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"createTransferAccountThis\",\n    \"outputs\": [],\n    \"stateMutability\": \"payable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"executor\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transferId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"getLockedAccountInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"sender_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"sender_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"receiver_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"receiver_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"balance\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"transfer_id\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locked_idx\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"is_closed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct BridgeEscrow.AccountInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"getLockedLength\",\n    \"outputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"start\",\n        \"type\": \"uint256\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"n\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"name\": \"getNextTransferId\",\n    \"outputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"uint256\",\n        \"name\": \"\",\n        \"type\": \"uint256\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transferId\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"getUnlockedAccountInfo\",\n    \"outputs\": [\n      {\n        \"components\": [\n          {\n            \"internalType\": \"address\",\n            \"name\": \"sender_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"sender_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"address payable\",\n            \"name\": \"receiver_this\",\n            \"type\": \"address\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"receiver_other\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint64\",\n            \"name\": \"balance\",\n            \"type\": \"uint64\"\n          },\n          {\n            \"internalType\": \"bytes16\",\n            \"name\": \"transfer_id\",\n            \"type\": \"bytes16\"\n          },\n          {\n            \"internalType\": \"uint256\",\n            \"name\": \"locked_idx\",\n            \"type\": \"uint256\"\n          },\n          {\n            \"internalType\": \"bool\",\n            \"name\": \"is_closed\",\n            \"type\": \"bool\"\n          }\n        ],\n        \"internalType\": \"struct BridgeEscrow.AccountInfo\",\n        \"name\": \"\",\n        \"type\": \"tuple\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [],\n    \"name\": \"owner\",\n    \"outputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"\",\n        \"type\": \"address\"\n      }\n    ],\n    \"stateMutability\": \"view\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_address\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrow\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"sender_other\",\n        \"type\": \"bytes16\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_this\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrowAux\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  },\n  {\n    \"inputs\": [\n      {\n        \"internalType\": \"address\",\n        \"name\": \"sender_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"address payable\",\n        \"name\": \"receiver_address\",\n        \"type\": \"address\"\n      },\n      {\n        \"internalType\": \"uint64\",\n        \"name\": \"balance\",\n        \"type\": \"uint64\"\n      },\n      {\n        \"internalType\": \"bytes16\",\n        \"name\": \"transfer_id\",\n        \"type\": \"bytes16\"\n      }\n    ],\n    \"name\": \"withdrawFromEscrowThis\",\n    \"outputs\": [],\n    \"stateMutability\": \"nonpayable\",\n    \"type\": \"function\"\n  }\n]\n" ) . expect ( "invalid abi" )
    });
    #[derive(Clone)]
    pub struct BridgeEscrow<'a, P, S>(Contract<'a, P, S>);
    impl<'a, P, S> std::ops::Deref for BridgeEscrow<'a, P, S> {
        type Target = Contract<'a, P, S>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> std::fmt::Debug for BridgeEscrow<'a, P, S> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(BridgeEscrow))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, P: JsonRpcClient, S: Signer> BridgeEscrow<'a, P, S> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<Address>>(address: T, client: &'a Client<P, S>) -> Self {
            let contract = Contract::new(address.into(), BRIDGEESCROW_ABI.clone(), client);
            Self(contract)
        }
        #[doc = "Calls the contract's `createTransferAccountAux` (0xfac60217) function"]
        pub fn create_transfer_account_aux(
            &self,
            sender_this: Address,
            sender_other: [u8; 16],
            receiver_this: Address,
            receiver_other: [u8; 16],
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash(
                    [250, 198, 2, 23],
                    (
                        sender_this,
                        sender_other,
                        receiver_this,
                        receiver_other,
                        amount,
                        transfer_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createTransferAccountThis` (0x3e60e5dd) function"]
        pub fn create_transfer_account_this(
            &self,
            receiver_address: Address,
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash([62, 96, 229, 221], (receiver_address, amount, transfer_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `call` (0x6dbf2fa0) function"]
        pub fn call(
            &self,
            to: Address,
            value: U256,
            data: Vec<u8>,
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash([109, 191, 47, 160], (to, value, data))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLockedLength` (0x1593d0f6) function"]
        pub fn get_locked_length(&self) -> ContractCall<'a, P, S, U256> {
            self.0
                .method_hash([21, 147, 208, 246], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLockedAccountInfo` (0x6ef902d8) function"]
        pub fn get_locked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, Token> {
            self.0
                .method_hash([110, 249, 2, 216], (transfer_id,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getNextTransferId` (0x27ac1453) function"]
        pub fn get_next_transfer_id(
            &self,
            start: U256,
            n: U256,
        ) -> ContractCall<'a, P, S, ([u8; 16], U256)> {
            self.0
                .method_hash([39, 172, 20, 83], (start, n))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrow` (0xdb4d7e05) function"]
        pub fn withdraw_from_escrow(
            &self,
            sender_address: [u8; 16],
            receiver_address: Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash(
                    [219, 77, 126, 5],
                    (sender_address, receiver_address, balance, transfer_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrowAux` (0x703b7597) function"]
        pub fn withdraw_from_escrow_aux(
            &self,
            sender_this: Address,
            sender_other: [u8; 16],
            receiver_this: Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash(
                    [112, 59, 117, 151],
                    (
                        sender_this,
                        sender_other,
                        receiver_this,
                        balance,
                        transfer_id,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getUnlockedAccountInfo` (0xc829f8e7) function"]
        pub fn get_unlocked_account_info(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, Token> {
            self.0
                .method_hash([200, 41, 248, 231], (transfer_id,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `createTransferAccount` (0x4f411670) function"]
        pub fn create_transfer_account(
            &self,
            receiver_address: [u8; 16],
            amount: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash([79, 65, 22, 112], (receiver_address, amount, transfer_id))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `withdrawFromEscrowThis` (0x5c904ac8) function"]
        pub fn withdraw_from_escrow_this(
            &self,
            sender_address: Address,
            receiver_address: Address,
            balance: u64,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash(
                    [92, 144, 74, 200],
                    (sender_address, receiver_address, balance, transfer_id),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `closeTransferAccountSender` (0x73df63f4) function"]
        pub fn close_transfer_account_sender(
            &self,
            transfer_id: [u8; 16],
        ) -> ContractCall<'a, P, S, H256> {
            self.0
                .method_hash([115, 223, 99, 244], (transfer_id,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(&self) -> ContractCall<'a, P, S, Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `executor` (0xc34c08e5) function"]
        pub fn executor(&self) -> ContractCall<'a, P, S, Address> {
            self.0
                .method_hash([195, 76, 8, 229], ())
                .expect("method not found (this should never happen)")
        }
    }
}
