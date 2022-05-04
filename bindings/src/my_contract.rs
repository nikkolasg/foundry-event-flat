pub use mycontract_mod::*;
#[allow(clippy::too_many_arguments)]
mod mycontract_mod {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "MyContract was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    pub static MYCONTRACT_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            serde_json :: from_str ("[{\"inputs\":[{\"internalType\":\"struct MyContract.Bundle\",\"name\":\"b\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct MyContract.Point[]\",\"name\":\"points\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct MyContract.SubBundle[]\",\"name\":\"sub\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct MyContract.Point\",\"name\":\"point\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]}]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewBundle\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct MyContract.Point\",\"name\":\"x\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"NewPoint\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"struct MyContract.Bundle\",\"name\":\"_bundle\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct MyContract.Point[]\",\"name\":\"points\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct MyContract.SubBundle[]\",\"name\":\"sub\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"id\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct MyContract.Point\",\"name\":\"point\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]}]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitBundle\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct MyContract.Point\",\"name\":\"_point\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"x\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"y\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"submitPoint\",\"outputs\":[]}]") . expect ("invalid abi")
        });
    #[doc = r" Bytecode of the #name contract"]
    pub static MYCONTRACT_BYTECODE: ethers::contract::Lazy<ethers::core::types::Bytes> =
        ethers::contract::Lazy::new(|| {
            "0x608060405234801561001057600080fd5b50610435806100206000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c806331257d161461003b578063b3c67bb814610050575b600080fd5b61004e610049366004610232565b610063565b005b61004e61005e366004610324565b61009d565b7f623cd61b549c4b3d7e23d3bdbe5c2a56f5569902992fe05b6d295f004d5039cb816040516100929190610347565b60405180910390a150565b6040805182518152602080840151908201527f0a10caaea7ac3c68834bca5fb5f42a10cb68bc3866ed86e6379d62bea6d591669101610092565b634e487b7160e01b600052604160045260246000fd5b6040805190810167ffffffffffffffff81118282101715610110576101106100d7565b60405290565b604051601f8201601f1916810167ffffffffffffffff8111828210171561013f5761013f6100d7565b604052919050565b600067ffffffffffffffff821115610161576101616100d7565b5060051b60200190565b60006040828403121561017d57600080fd5b6101856100ed565b9050813581526020820135602082015292915050565b600082601f8301126101ac57600080fd5b813560206101c16101bc83610147565b610116565b828152606092830285018201928282019190878511156101e057600080fd5b8387015b858110156102255781818a0312156101fc5760008081fd5b6102046100ed565b813581526102148a87840161016b565b8187015284529284019281016101e4565b5090979650505050505050565b6000602080838503121561024557600080fd5b823567ffffffffffffffff8082111561025d57600080fd5b8185019150604080838803121561027357600080fd5b61027b6100ed565b83358381111561028a57600080fd5b8401601f8101891361029b57600080fd5b80356102a96101bc82610147565b81815260069190911b8201870190878101908b8311156102c857600080fd5b928801925b828410156102ee576102df8c8561016b565b825292850192908801906102cd565b84525050508385013591508282111561030657600080fd5b6103128883860161019b565b85820152809550505050505092915050565b60006040828403121561033657600080fd5b610340838361016b565b9392505050565b6000602080835260608084018551604080858801528282518085526080890191508684019450600093505b808410156103a35761038f82865180518252602090810151910152565b938601936001939093019290820190610372565b5088860151888203601f190192890192909252815180825291860193508501915060005b81811015610225578351805184528601516103ee8785018280518252602090810151910152565b5092850192918401916001016103c756fea2646970667358221220fadf043acf1e7d2a69ccf626debc89d2957b652f801e290f4e2b450478ceb92d64736f6c634300080a0033" . parse () . expect ("invalid bytecode")
        });
    pub struct MyContract<M>(ethers::contract::Contract<M>);
    impl<M> Clone for MyContract<M> {
        fn clone(&self) -> Self {
            MyContract(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for MyContract<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M: ethers::providers::Middleware> std::fmt::Debug for MyContract<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(MyContract))
                .field(&self.address())
                .finish()
        }
    }
    impl<'a, M: ethers::providers::Middleware> MyContract<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), MYCONTRACT_ABI.clone(), client).into()
        }
        #[doc = r" Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it."]
        #[doc = r" Returns a new instance of a deployer that returns an instance of this contract after sending the transaction"]
        #[doc = r""]
        #[doc = r" Notes:"]
        #[doc = r" 1. If there are no constructor arguments, you should pass `()` as the argument."]
        #[doc = r" 1. The default poll duration is 7 seconds."]
        #[doc = r" 1. The default number of confirmations is 1 block."]
        #[doc = r""]
        #[doc = r""]
        #[doc = r" # Example"]
        #[doc = r""]
        #[doc = r" Generate contract bindings with `abigen!` and deploy a new contract instance."]
        #[doc = r""]
        #[doc = r" *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact."]
        #[doc = r""]
        #[doc = r" ```ignore"]
        #[doc = r" # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {"]
        #[doc = r#"     abigen!(Greeter,"../greeter.json");"#]
        #[doc = r""]
        #[doc = r#"    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();"#]
        #[doc = r"    let msg = greeter_contract.greet().call().await.unwrap();"]
        #[doc = r" # }"]
        #[doc = r" ```"]
        pub fn deploy<T: ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> Result<
            ethers::contract::builders::ContractDeployer<M, Self>,
            ethers::contract::ContractError<M>,
        > {
            let factory = ethers::contract::ContractFactory::new(
                MYCONTRACT_ABI.clone(),
                MYCONTRACT_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        #[doc = "Calls the contract's `submitBundle` (0x31257d16) function"]
        pub fn submit_bundle(
            &self,
            bundle: Bundle,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 37, 125, 22], (bundle,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `submitPoint` (0xb3c67bb8) function"]
        pub fn submit_point(
            &self,
            point: Point,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 198, 123, 184], (point,))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `NewBundle` event"]
        pub fn new_bundle_filter(&self) -> ethers::contract::builders::Event<M, NewBundleFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `NewPoint` event"]
        pub fn new_point_filter(&self) -> ethers::contract::builders::Event<M, NewPointFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, MyContractEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for MyContract<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(
        name = "NewBundle",
        abi = "NewBundle(((uint256,uint256)[],(uint256,(uint256,uint256))[]))"
    )]
    pub struct NewBundleFilter {
        pub b: (
            Vec<(ethers::core::types::U256, ethers::core::types::U256)>,
            Vec<(
                ethers::core::types::U256,
                (ethers::core::types::U256, ethers::core::types::U256),
            )>,
        ),
    }
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
    )]
    #[ethevent(name = "NewPoint", abi = "NewPoint((uint256,uint256))")]
    pub struct NewPointFilter {
        pub x: (ethers::core::types::U256, ethers::core::types::U256),
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MyContractEvents {
        NewBundleFilter(NewBundleFilter),
        NewPointFilter(NewPointFilter),
    }
    impl ethers::contract::EthLogDecode for MyContractEvents {
        fn decode_log(log: &ethers::core::abi::RawLog) -> Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = NewBundleFilter::decode_log(log) {
                return Ok(MyContractEvents::NewBundleFilter(decoded));
            }
            if let Ok(decoded) = NewPointFilter::decode_log(log) {
                return Ok(MyContractEvents::NewPointFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for MyContractEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MyContractEvents::NewBundleFilter(element) => element.fmt(f),
                MyContractEvents::NewPointFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `submitBundle`function with signature `submitBundle(((uint256,uint256)[],(uint256,(uint256,uint256))[]))` and selector `[49, 37, 125, 22]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(
        name = "submitBundle",
        abi = "submitBundle(((uint256,uint256)[],(uint256,(uint256,uint256))[]))"
    )]
    pub struct SubmitBundleCall {
        pub bundle: Bundle,
    }
    #[doc = "Container type for all input parameters for the `submitPoint`function with signature `submitPoint((uint256,uint256))` and selector `[179, 198, 123, 184]`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
    )]
    #[ethcall(name = "submitPoint", abi = "submitPoint((uint256,uint256))")]
    pub struct SubmitPointCall {
        pub point: Point,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum MyContractCalls {
        SubmitBundle(SubmitBundleCall),
        SubmitPoint(SubmitPointCall),
    }
    impl ethers::core::abi::AbiDecode for MyContractCalls {
        fn decode(data: impl AsRef<[u8]>) -> Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <SubmitBundleCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MyContractCalls::SubmitBundle(decoded));
            }
            if let Ok(decoded) =
                <SubmitPointCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(MyContractCalls::SubmitPoint(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for MyContractCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                MyContractCalls::SubmitBundle(element) => element.encode(),
                MyContractCalls::SubmitPoint(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for MyContractCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                MyContractCalls::SubmitBundle(element) => element.fmt(f),
                MyContractCalls::SubmitPoint(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<SubmitBundleCall> for MyContractCalls {
        fn from(var: SubmitBundleCall) -> Self {
            MyContractCalls::SubmitBundle(var)
        }
    }
    impl ::std::convert::From<SubmitPointCall> for MyContractCalls {
        fn from(var: SubmitPointCall) -> Self {
            MyContractCalls::SubmitPoint(var)
        }
    }
    #[doc = "`Bundle((uint256,uint256)[],(uint256,(uint256,uint256))[])`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Bundle {
        pub points: ::std::vec::Vec<Point>,
        pub sub: ::std::vec::Vec<SubBundle>,
    }
    #[doc = "`Point(uint256,uint256)`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct Point {
        pub x: ethers::core::types::U256,
        pub y: ethers::core::types::U256,
    }
    #[doc = "`SubBundle(uint256,(uint256,uint256))`"]
    #[derive(
        Clone,
        Debug,
        Default,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
    )]
    pub struct SubBundle {
        pub id: ethers::core::types::U256,
        pub point: Point,
    }
}
