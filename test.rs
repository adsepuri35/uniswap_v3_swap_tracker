#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use anyhow::Result;
use backend_update::BackendUpdate;
use tokio::sync::mpsc;
mod ierc20 {
    use alloy::core::sol_types::sol;
    ///Module containing a contract's types and functions.
    /**

```solidity
interface IERC20 {
    event Approval(address indexed owner, address indexed spender, uint value);
    event Transfer(address indexed from, address indexed to, uint value);
    function name() external view returns (string memory);
    function symbol() external view returns (string memory);
    function decimals() external view returns (uint8);
    function totalSupply() external view returns (uint);
    function balanceOf(address owner) external view returns (uint);
    function allowance(address owner, address spender) external view returns (uint);
    function approve(address spender, uint value) external returns (bool);
    function transfer(address to, uint value) external returns (bool);
    function transferFrom(address from, address to, uint value) external returns (bool);
}
```*/
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::pub_underscore_fields,
        clippy::style,
        clippy::empty_structs_with_brackets
    )]
    pub mod IERC20 {
        use super::*;
        use ::alloy_sol_types as alloy_sol_types;
        /**Event with signature `Approval(address,address,uint256)` and selector `0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925`.
```solidity
event Approval(address indexed owner, address indexed spender, uint value);
```*/
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        pub struct Approval {
            #[allow(missing_docs)]
            pub owner: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub spender: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub value: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::clone::Clone for Approval {
            #[inline]
            fn clone(&self) -> Approval {
                Approval {
                    owner: ::core::clone::Clone::clone(&self.owner),
                    spender: ::core::clone::Clone::clone(&self.spender),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::fmt::Debug for Approval {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Approval",
                    "owner",
                    &self.owner,
                    "spender",
                    &self.spender,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::marker::StructuralPartialEq for Approval {}
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::cmp::PartialEq for Approval {
            #[inline]
            fn eq(&self, other: &Approval) -> bool {
                self.owner == other.owner && self.spender == other.spender
                    && self.value == other.value
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::cmp::Eq for Approval {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            #[automatically_derived]
            impl alloy_sol_types::SolEvent for Approval {
                type DataTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                type DataToken<'a> = <Self::DataTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type TopicList = (
                    alloy_sol_types::sol_data::FixedBytes<32>,
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                );
                const SIGNATURE: &'static str = "Approval(address,address,uint256)";
                const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                    140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8,
                    113u8, 66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8,
                    247u8, 178u8, 41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8,
                    185u8, 37u8,
                ]);
                const ANONYMOUS: bool = false;
                #[allow(unused_variables)]
                #[inline]
                fn new(
                    topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                    data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    Self {
                        owner: topics.1,
                        spender: topics.2,
                        value: data.0,
                    }
                }
                #[inline]
                fn check_signature(
                    topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
                ) -> alloy_sol_types::Result<()> {
                    if topics.0 != Self::SIGNATURE_HASH {
                        return Err(
                            alloy_sol_types::Error::invalid_event_signature_hash(
                                Self::SIGNATURE,
                                topics.0,
                                Self::SIGNATURE_HASH,
                            ),
                        );
                    }
                    Ok(())
                }
                #[inline]
                fn tokenize_body(&self) -> Self::DataToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(&self.value),
                    )
                }
                #[inline]
                fn topics(
                    &self,
                ) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                    (
                        Self::SIGNATURE_HASH.into(),
                        self.owner.clone(),
                        self.spender.clone(),
                    )
                }
                #[inline]
                fn encode_topics_raw(
                    &self,
                    out: &mut [alloy_sol_types::abi::token::WordToken],
                ) -> alloy_sol_types::Result<()> {
                    if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT
                    {
                        return Err(alloy_sol_types::Error::Overrun);
                    }
                    out[0usize] = alloy_sol_types::abi::token::WordToken(
                        Self::SIGNATURE_HASH,
                    );
                    out[1usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                        &self.owner,
                    );
                    out[2usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                        &self.spender,
                    );
                    Ok(())
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::private::IntoLogData for Approval {
                fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                    From::from(self)
                }
                fn into_log_data(self) -> alloy_sol_types::private::LogData {
                    From::from(&self)
                }
            }
            #[automatically_derived]
            impl From<&Approval> for alloy_sol_types::private::LogData {
                #[inline]
                fn from(this: &Approval) -> alloy_sol_types::private::LogData {
                    alloy_sol_types::SolEvent::encode_log_data(this)
                }
            }
        };
        /**Event with signature `Transfer(address,address,uint256)` and selector `0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef`.
```solidity
event Transfer(address indexed from, address indexed to, uint value);
```*/
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        pub struct Transfer {
            #[allow(missing_docs)]
            pub from: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub to: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub value: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::clone::Clone for Transfer {
            #[inline]
            fn clone(&self) -> Transfer {
                Transfer {
                    from: ::core::clone::Clone::clone(&self.from),
                    to: ::core::clone::Clone::clone(&self.to),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::fmt::Debug for Transfer {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Transfer",
                    "from",
                    &self.from,
                    "to",
                    &self.to,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::marker::StructuralPartialEq for Transfer {}
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::cmp::PartialEq for Transfer {
            #[inline]
            fn eq(&self, other: &Transfer) -> bool {
                self.from == other.from && self.to == other.to
                    && self.value == other.value
            }
        }
        #[automatically_derived]
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        impl ::core::cmp::Eq for Transfer {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            #[automatically_derived]
            impl alloy_sol_types::SolEvent for Transfer {
                type DataTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                type DataToken<'a> = <Self::DataTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type TopicList = (
                    alloy_sol_types::sol_data::FixedBytes<32>,
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                );
                const SIGNATURE: &'static str = "Transfer(address,address,uint256)";
                const SIGNATURE_HASH: alloy_sol_types::private::B256 = alloy_sol_types::private::B256::new([
                    221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                    176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8,
                    99u8, 196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8,
                    179u8, 239u8,
                ]);
                const ANONYMOUS: bool = false;
                #[allow(unused_variables)]
                #[inline]
                fn new(
                    topics: <Self::TopicList as alloy_sol_types::SolType>::RustType,
                    data: <Self::DataTuple<'_> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    Self {
                        from: topics.1,
                        to: topics.2,
                        value: data.0,
                    }
                }
                #[inline]
                fn check_signature(
                    topics: &<Self::TopicList as alloy_sol_types::SolType>::RustType,
                ) -> alloy_sol_types::Result<()> {
                    if topics.0 != Self::SIGNATURE_HASH {
                        return Err(
                            alloy_sol_types::Error::invalid_event_signature_hash(
                                Self::SIGNATURE,
                                topics.0,
                                Self::SIGNATURE_HASH,
                            ),
                        );
                    }
                    Ok(())
                }
                #[inline]
                fn tokenize_body(&self) -> Self::DataToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(&self.value),
                    )
                }
                #[inline]
                fn topics(
                    &self,
                ) -> <Self::TopicList as alloy_sol_types::SolType>::RustType {
                    (Self::SIGNATURE_HASH.into(), self.from.clone(), self.to.clone())
                }
                #[inline]
                fn encode_topics_raw(
                    &self,
                    out: &mut [alloy_sol_types::abi::token::WordToken],
                ) -> alloy_sol_types::Result<()> {
                    if out.len() < <Self::TopicList as alloy_sol_types::TopicList>::COUNT
                    {
                        return Err(alloy_sol_types::Error::Overrun);
                    }
                    out[0usize] = alloy_sol_types::abi::token::WordToken(
                        Self::SIGNATURE_HASH,
                    );
                    out[1usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                        &self.from,
                    );
                    out[2usize] = <::alloy_sol_types::sol_data::Address as alloy_sol_types::EventTopic>::encode_topic(
                        &self.to,
                    );
                    Ok(())
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::private::IntoLogData for Transfer {
                fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                    From::from(self)
                }
                fn into_log_data(self) -> alloy_sol_types::private::LogData {
                    From::from(&self)
                }
            }
            #[automatically_derived]
            impl From<&Transfer> for alloy_sol_types::private::LogData {
                #[inline]
                fn from(this: &Transfer) -> alloy_sol_types::private::LogData {
                    alloy_sol_types::SolEvent::encode_log_data(this)
                }
            }
        };
        /**Function with signature `name()` and selector `0x06fdde03`.
```solidity
function name() external view returns (string memory);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct nameCall;
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for nameCall {
            #[inline]
            fn clone(&self) -> nameCall {
                nameCall
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for nameCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "nameCall")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for nameCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for nameCall {
            #[inline]
            fn eq(&self, other: &nameCall) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for nameCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        ///Container type for the return parameters of the [`name()`](nameCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct nameReturn {
            #[allow(missing_docs)]
            pub _0: ::alloy_sol_types::private::String,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for nameReturn {
            #[inline]
            fn clone(&self) -> nameReturn {
                nameReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for nameReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "nameReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for nameReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for nameReturn {
            #[inline]
            fn eq(&self, other: &nameReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for nameReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::String>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = ();
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = ();
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<nameCall> for UnderlyingRustTuple<'_> {
                    fn from(value: nameCall) -> Self {
                        ()
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::String,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<nameReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: nameReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for nameReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for nameCall {
                type Parameters<'a> = ();
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = ::alloy_sol_types::private::String;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::String,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "name()";
                const SELECTOR: [u8; 4] = [6u8, 253u8, 222u8, 3u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    ()
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                            ret,
                        ),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: nameReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: nameReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `symbol()` and selector `0x95d89b41`.
```solidity
function symbol() external view returns (string memory);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct symbolCall;
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for symbolCall {
            #[inline]
            fn clone(&self) -> symbolCall {
                symbolCall
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for symbolCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "symbolCall")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for symbolCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for symbolCall {
            #[inline]
            fn eq(&self, other: &symbolCall) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for symbolCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        ///Container type for the return parameters of the [`symbol()`](symbolCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct symbolReturn {
            #[allow(missing_docs)]
            pub _0: ::alloy_sol_types::private::String,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for symbolReturn {
            #[inline]
            fn clone(&self) -> symbolReturn {
                symbolReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for symbolReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "symbolReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for symbolReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for symbolReturn {
            #[inline]
            fn eq(&self, other: &symbolReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for symbolReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::String>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = ();
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = ();
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<symbolCall> for UnderlyingRustTuple<'_> {
                    fn from(value: symbolCall) -> Self {
                        ()
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::String,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::String,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<symbolReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: symbolReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for symbolReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for symbolCall {
                type Parameters<'a> = ();
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = ::alloy_sol_types::private::String;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::String,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "symbol()";
                const SELECTOR: [u8; 4] = [149u8, 216u8, 155u8, 65u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    ()
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::String as alloy_sol_types::SolType>::tokenize(
                            ret,
                        ),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: symbolReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: symbolReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `decimals()` and selector `0x313ce567`.
```solidity
function decimals() external view returns (uint8);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct decimalsCall;
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for decimalsCall {
            #[inline]
            fn clone(&self) -> decimalsCall {
                decimalsCall
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for decimalsCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "decimalsCall")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for decimalsCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for decimalsCall {
            #[inline]
            fn eq(&self, other: &decimalsCall) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for decimalsCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        ///Container type for the return parameters of the [`decimals()`](decimalsCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct decimalsReturn {
            #[allow(missing_docs)]
            pub _0: u8,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for decimalsReturn {
            #[inline]
            fn clone(&self) -> decimalsReturn {
                decimalsReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for decimalsReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "decimalsReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for decimalsReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for decimalsReturn {
            #[inline]
            fn eq(&self, other: &decimalsReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for decimalsReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<u8>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = ();
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = ();
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<decimalsCall> for UnderlyingRustTuple<'_> {
                    fn from(value: decimalsCall) -> Self {
                        ()
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<8>,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (u8,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<decimalsReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: decimalsReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for decimalsReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for decimalsCall {
                type Parameters<'a> = ();
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = u8;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<8>,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "decimals()";
                const SELECTOR: [u8; 4] = [49u8, 60u8, 229u8, 103u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    ()
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            8,
                        > as alloy_sol_types::SolType>::tokenize(ret),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: decimalsReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: decimalsReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `totalSupply()` and selector `0x18160ddd`.
```solidity
function totalSupply() external view returns (uint);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct totalSupplyCall;
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for totalSupplyCall {
            #[inline]
            fn clone(&self) -> totalSupplyCall {
                totalSupplyCall
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for totalSupplyCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::write_str(f, "totalSupplyCall")
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for totalSupplyCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for totalSupplyCall {
            #[inline]
            fn eq(&self, other: &totalSupplyCall) -> bool {
                true
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for totalSupplyCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {}
        }
        ///Container type for the return parameters of the [`totalSupply()`](totalSupplyCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct totalSupplyReturn {
            #[allow(missing_docs)]
            pub _0: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for totalSupplyReturn {
            #[inline]
            fn clone(&self) -> totalSupplyReturn {
                totalSupplyReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for totalSupplyReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "totalSupplyReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for totalSupplyReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for totalSupplyReturn {
            #[inline]
            fn eq(&self, other: &totalSupplyReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for totalSupplyReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = ();
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = ();
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<totalSupplyCall> for UnderlyingRustTuple<'_> {
                    fn from(value: totalSupplyCall) -> Self {
                        ()
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for totalSupplyCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<totalSupplyReturn>
                for UnderlyingRustTuple<'_> {
                    fn from(value: totalSupplyReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for totalSupplyReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for totalSupplyCall {
                type Parameters<'a> = ();
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = ::alloy_sol_types::private::primitives::aliases::U256;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "totalSupply()";
                const SELECTOR: [u8; 4] = [24u8, 22u8, 13u8, 221u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    ()
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(ret),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: totalSupplyReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: totalSupplyReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `balanceOf(address)` and selector `0x70a08231`.
```solidity
function balanceOf(address owner) external view returns (uint);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct balanceOfCall {
            #[allow(missing_docs)]
            pub owner: ::alloy_sol_types::private::Address,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for balanceOfCall {
            #[inline]
            fn clone(&self) -> balanceOfCall {
                balanceOfCall {
                    owner: ::core::clone::Clone::clone(&self.owner),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for balanceOfCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "balanceOfCall",
                    "owner",
                    &&self.owner,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for balanceOfCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for balanceOfCall {
            #[inline]
            fn eq(&self, other: &balanceOfCall) -> bool {
                self.owner == other.owner
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for balanceOfCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
            }
        }
        ///Container type for the return parameters of the [`balanceOf(address)`](balanceOfCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct balanceOfReturn {
            #[allow(missing_docs)]
            pub _0: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for balanceOfReturn {
            #[inline]
            fn clone(&self) -> balanceOfReturn {
                balanceOfReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for balanceOfReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "balanceOfReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for balanceOfReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for balanceOfReturn {
            #[inline]
            fn eq(&self, other: &balanceOfReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for balanceOfReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Address,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (::alloy_sol_types::private::Address,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<balanceOfCall> for UnderlyingRustTuple<'_> {
                    fn from(value: balanceOfCall) -> Self {
                        (value.owner,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { owner: tuple.0 }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<balanceOfReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: balanceOfReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for balanceOfReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for balanceOfCall {
                type Parameters<'a> = (::alloy_sol_types::sol_data::Address,);
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = ::alloy_sol_types::private::primitives::aliases::U256;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "balanceOf(address)";
                const SELECTOR: [u8; 4] = [112u8, 160u8, 130u8, 49u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.owner,
                        ),
                    )
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(ret),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: balanceOfReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: balanceOfReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `allowance(address,address)` and selector `0xdd62ed3e`.
```solidity
function allowance(address owner, address spender) external view returns (uint);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct allowanceCall {
            #[allow(missing_docs)]
            pub owner: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub spender: ::alloy_sol_types::private::Address,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for allowanceCall {
            #[inline]
            fn clone(&self) -> allowanceCall {
                allowanceCall {
                    owner: ::core::clone::Clone::clone(&self.owner),
                    spender: ::core::clone::Clone::clone(&self.spender),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for allowanceCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "allowanceCall",
                    "owner",
                    &self.owner,
                    "spender",
                    &&self.spender,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for allowanceCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for allowanceCall {
            #[inline]
            fn eq(&self, other: &allowanceCall) -> bool {
                self.owner == other.owner && self.spender == other.spender
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for allowanceCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
            }
        }
        ///Container type for the return parameters of the [`allowance(address,address)`](allowanceCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct allowanceReturn {
            #[allow(missing_docs)]
            pub _0: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for allowanceReturn {
            #[inline]
            fn clone(&self) -> allowanceReturn {
                allowanceReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for allowanceReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "allowanceReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for allowanceReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for allowanceReturn {
            #[inline]
            fn eq(&self, other: &allowanceReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for allowanceReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                );
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::Address,
                    ::alloy_sol_types::private::Address,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<allowanceCall> for UnderlyingRustTuple<'_> {
                    fn from(value: allowanceCall) -> Self {
                        (value.owner, value.spender)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self {
                            owner: tuple.0,
                            spender: tuple.1,
                        }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<allowanceReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: allowanceReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for allowanceReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for allowanceCall {
                type Parameters<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                );
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = ::alloy_sol_types::private::primitives::aliases::U256;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Uint<256>,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "allowance(address,address)";
                const SELECTOR: [u8; 4] = [221u8, 98u8, 237u8, 62u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.owner,
                        ),
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.spender,
                        ),
                    )
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(ret),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: allowanceReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: allowanceReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `approve(address,uint256)` and selector `0x095ea7b3`.
```solidity
function approve(address spender, uint value) external returns (bool);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct approveCall {
            #[allow(missing_docs)]
            pub spender: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub value: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for approveCall {
            #[inline]
            fn clone(&self) -> approveCall {
                approveCall {
                    spender: ::core::clone::Clone::clone(&self.spender),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for approveCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "approveCall",
                    "spender",
                    &self.spender,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for approveCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for approveCall {
            #[inline]
            fn eq(&self, other: &approveCall) -> bool {
                self.spender == other.spender && self.value == other.value
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for approveCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        ///Container type for the return parameters of the [`approve(address,uint256)`](approveCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct approveReturn {
            #[allow(missing_docs)]
            pub _0: bool,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for approveReturn {
            #[inline]
            fn clone(&self) -> approveReturn {
                approveReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for approveReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "approveReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for approveReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for approveReturn {
            #[inline]
            fn eq(&self, other: &approveReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for approveReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::Address,
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<approveCall> for UnderlyingRustTuple<'_> {
                    fn from(value: approveCall) -> Self {
                        (value.spender, value.value)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self {
                            spender: tuple.0,
                            value: tuple.1,
                        }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (bool,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<approveReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: approveReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for approveReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for approveCall {
                type Parameters<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = bool;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "approve(address,uint256)";
                const SELECTOR: [u8; 4] = [9u8, 94u8, 167u8, 179u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.spender,
                        ),
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(&self.value),
                    )
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                            ret,
                        ),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: approveReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: approveReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `transfer(address,uint256)` and selector `0xa9059cbb`.
```solidity
function transfer(address to, uint value) external returns (bool);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct transferCall {
            #[allow(missing_docs)]
            pub to: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub value: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for transferCall {
            #[inline]
            fn clone(&self) -> transferCall {
                transferCall {
                    to: ::core::clone::Clone::clone(&self.to),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for transferCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "transferCall",
                    "to",
                    &self.to,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for transferCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for transferCall {
            #[inline]
            fn eq(&self, other: &transferCall) -> bool {
                self.to == other.to && self.value == other.value
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for transferCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        ///Container type for the return parameters of the [`transfer(address,uint256)`](transferCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct transferReturn {
            #[allow(missing_docs)]
            pub _0: bool,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for transferReturn {
            #[inline]
            fn clone(&self) -> transferReturn {
                transferReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for transferReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "transferReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for transferReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for transferReturn {
            #[inline]
            fn eq(&self, other: &transferReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for transferReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::Address,
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<transferCall> for UnderlyingRustTuple<'_> {
                    fn from(value: transferCall) -> Self {
                        (value.to, value.value)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self {
                            to: tuple.0,
                            value: tuple.1,
                        }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (bool,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<transferReturn> for UnderlyingRustTuple<'_> {
                    fn from(value: transferReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>> for transferReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for transferCall {
                type Parameters<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = bool;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "transfer(address,uint256)";
                const SELECTOR: [u8; 4] = [169u8, 5u8, 156u8, 187u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.to,
                        ),
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(&self.value),
                    )
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                            ret,
                        ),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: transferReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: transferReturn = r.into();
                            r._0
                        })
                }
            }
        };
        /**Function with signature `transferFrom(address,address,uint256)` and selector `0x23b872dd`.
```solidity
function transferFrom(address from, address to, uint value) external returns (bool);
```*/
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct transferFromCall {
            #[allow(missing_docs)]
            pub from: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub to: ::alloy_sol_types::private::Address,
            #[allow(missing_docs)]
            pub value: ::alloy_sol_types::private::primitives::aliases::U256,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for transferFromCall {
            #[inline]
            fn clone(&self) -> transferFromCall {
                transferFromCall {
                    from: ::core::clone::Clone::clone(&self.from),
                    to: ::core::clone::Clone::clone(&self.to),
                    value: ::core::clone::Clone::clone(&self.value),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for transferFromCall {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "transferFromCall",
                    "from",
                    &self.from,
                    "to",
                    &self.to,
                    "value",
                    &&self.value,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for transferFromCall {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for transferFromCall {
            #[inline]
            fn eq(&self, other: &transferFromCall) -> bool {
                self.from == other.from && self.to == other.to
                    && self.value == other.value
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for transferFromCall {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<::alloy_sol_types::private::Address>;
                let _: ::core::cmp::AssertParamIsEq<
                    ::alloy_sol_types::private::primitives::aliases::U256,
                >;
            }
        }
        ///Container type for the return parameters of the [`transferFrom(address,address,uint256)`](transferFromCall) function.
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        pub struct transferFromReturn {
            #[allow(missing_docs)]
            pub _0: bool,
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::clone::Clone for transferFromReturn {
            #[inline]
            fn clone(&self) -> transferFromReturn {
                transferFromReturn {
                    _0: ::core::clone::Clone::clone(&self._0),
                }
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::fmt::Debug for transferFromReturn {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "transferFromReturn",
                    "_0",
                    &&self._0,
                )
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::marker::StructuralPartialEq for transferFromReturn {}
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::PartialEq for transferFromReturn {
            #[inline]
            fn eq(&self, other: &transferFromReturn) -> bool {
                self._0 == other._0
            }
        }
        #[automatically_derived]
        #[allow(non_camel_case_types, non_snake_case, clippy::pub_underscore_fields)]
        impl ::core::cmp::Eq for transferFromReturn {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<bool>;
            }
        }
        #[allow(
            non_camel_case_types,
            non_snake_case,
            clippy::pub_underscore_fields,
            clippy::style
        )]
        const _: () = {
            use ::alloy_sol_types as alloy_sol_types;
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (
                    ::alloy_sol_types::private::Address,
                    ::alloy_sol_types::private::Address,
                    ::alloy_sol_types::private::primitives::aliases::U256,
                );
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<transferFromCall>
                for UnderlyingRustTuple<'_> {
                    fn from(value: transferFromCall) -> Self {
                        (value.from, value.to, value.value)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for transferFromCall {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self {
                            from: tuple.0,
                            to: tuple.1,
                            value: tuple.2,
                        }
                    }
                }
            }
            {
                #[doc(hidden)]
                type UnderlyingSolTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                #[doc(hidden)]
                type UnderlyingRustTuple<'a> = (bool,);
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<transferFromReturn>
                for UnderlyingRustTuple<'_> {
                    fn from(value: transferFromReturn) -> Self {
                        (value._0,)
                    }
                }
                #[automatically_derived]
                #[doc(hidden)]
                impl ::core::convert::From<UnderlyingRustTuple<'_>>
                for transferFromReturn {
                    fn from(tuple: UnderlyingRustTuple<'_>) -> Self {
                        Self { _0: tuple.0 }
                    }
                }
            }
            #[automatically_derived]
            impl alloy_sol_types::SolCall for transferFromCall {
                type Parameters<'a> = (
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Address,
                    ::alloy_sol_types::sol_data::Uint<256>,
                );
                type Token<'a> = <Self::Parameters<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                type Return = bool;
                type ReturnTuple<'a> = (::alloy_sol_types::sol_data::Bool,);
                type ReturnToken<'a> = <Self::ReturnTuple<
                    'a,
                > as alloy_sol_types::SolType>::Token<'a>;
                const SIGNATURE: &'static str = "transferFrom(address,address,uint256)";
                const SELECTOR: [u8; 4] = [35u8, 184u8, 114u8, 221u8];
                #[inline]
                fn new<'a>(
                    tuple: <Self::Parameters<'a> as alloy_sol_types::SolType>::RustType,
                ) -> Self {
                    tuple.into()
                }
                #[inline]
                fn tokenize(&self) -> Self::Token<'_> {
                    (
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.from,
                        ),
                        <::alloy_sol_types::sol_data::Address as alloy_sol_types::SolType>::tokenize(
                            &self.to,
                        ),
                        <::alloy_sol_types::sol_data::Uint<
                            256,
                        > as alloy_sol_types::SolType>::tokenize(&self.value),
                    )
                }
                #[inline]
                fn tokenize_returns(ret: &Self::Return) -> Self::ReturnToken<'_> {
                    (
                        <::alloy_sol_types::sol_data::Bool as alloy_sol_types::SolType>::tokenize(
                            ret,
                        ),
                    )
                }
                #[inline]
                fn abi_decode_returns(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence(data)
                        .map(|r| {
                            let r: transferFromReturn = r.into();
                            r._0
                        })
                }
                #[inline]
                fn abi_decode_returns_validate(
                    data: &[u8],
                ) -> alloy_sol_types::Result<Self::Return> {
                    <Self::ReturnTuple<
                        '_,
                    > as alloy_sol_types::SolType>::abi_decode_sequence_validate(data)
                        .map(|r| {
                            let r: transferFromReturn = r.into();
                            r._0
                        })
                }
            }
        };
        ///Container for all the [`IERC20`](self) function calls.
        pub enum IERC20Calls {
            #[allow(missing_docs)]
            name(nameCall),
            #[allow(missing_docs)]
            symbol(symbolCall),
            #[allow(missing_docs)]
            decimals(decimalsCall),
            #[allow(missing_docs)]
            totalSupply(totalSupplyCall),
            #[allow(missing_docs)]
            balanceOf(balanceOfCall),
            #[allow(missing_docs)]
            allowance(allowanceCall),
            #[allow(missing_docs)]
            approve(approveCall),
            #[allow(missing_docs)]
            transfer(transferCall),
            #[allow(missing_docs)]
            transferFrom(transferFromCall),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IERC20Calls {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    IERC20Calls::name(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "name",
                            &__self_0,
                        )
                    }
                    IERC20Calls::symbol(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "symbol",
                            &__self_0,
                        )
                    }
                    IERC20Calls::decimals(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "decimals",
                            &__self_0,
                        )
                    }
                    IERC20Calls::totalSupply(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "totalSupply",
                            &__self_0,
                        )
                    }
                    IERC20Calls::balanceOf(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "balanceOf",
                            &__self_0,
                        )
                    }
                    IERC20Calls::allowance(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "allowance",
                            &__self_0,
                        )
                    }
                    IERC20Calls::approve(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "approve",
                            &__self_0,
                        )
                    }
                    IERC20Calls::transfer(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "transfer",
                            &__self_0,
                        )
                    }
                    IERC20Calls::transferFrom(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "transferFrom",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IERC20Calls {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IERC20Calls {
            #[inline]
            fn eq(&self, other: &IERC20Calls) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (IERC20Calls::name(__self_0), IERC20Calls::name(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (
                            IERC20Calls::symbol(__self_0),
                            IERC20Calls::symbol(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::decimals(__self_0),
                            IERC20Calls::decimals(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::totalSupply(__self_0),
                            IERC20Calls::totalSupply(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::balanceOf(__self_0),
                            IERC20Calls::balanceOf(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::allowance(__self_0),
                            IERC20Calls::allowance(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::approve(__self_0),
                            IERC20Calls::approve(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::transfer(__self_0),
                            IERC20Calls::transfer(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Calls::transferFrom(__self_0),
                            IERC20Calls::transferFrom(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for IERC20Calls {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<nameCall>;
                let _: ::core::cmp::AssertParamIsEq<symbolCall>;
                let _: ::core::cmp::AssertParamIsEq<decimalsCall>;
                let _: ::core::cmp::AssertParamIsEq<totalSupplyCall>;
                let _: ::core::cmp::AssertParamIsEq<balanceOfCall>;
                let _: ::core::cmp::AssertParamIsEq<allowanceCall>;
                let _: ::core::cmp::AssertParamIsEq<approveCall>;
                let _: ::core::cmp::AssertParamIsEq<transferCall>;
                let _: ::core::cmp::AssertParamIsEq<transferFromCall>;
            }
        }
        #[automatically_derived]
        impl IERC20Calls {
            /// All the selectors of this enum.
            ///
            /// Note that the selectors might not be in the same order as the variants.
            /// No guarantees are made about the order of the selectors.
            ///
            /// Prefer using `SolInterface` methods instead.
            pub const SELECTORS: &'static [[u8; 4usize]] = &[
                [6u8, 253u8, 222u8, 3u8],
                [9u8, 94u8, 167u8, 179u8],
                [24u8, 22u8, 13u8, 221u8],
                [35u8, 184u8, 114u8, 221u8],
                [49u8, 60u8, 229u8, 103u8],
                [112u8, 160u8, 130u8, 49u8],
                [149u8, 216u8, 155u8, 65u8],
                [169u8, 5u8, 156u8, 187u8],
                [221u8, 98u8, 237u8, 62u8],
            ];
        }
        #[automatically_derived]
        impl alloy_sol_types::SolInterface for IERC20Calls {
            const NAME: &'static str = "IERC20Calls";
            const MIN_DATA_LENGTH: usize = 0usize;
            const COUNT: usize = 9usize;
            #[inline]
            fn selector(&self) -> [u8; 4] {
                match self {
                    Self::name(_) => <nameCall as alloy_sol_types::SolCall>::SELECTOR,
                    Self::symbol(_) => <symbolCall as alloy_sol_types::SolCall>::SELECTOR,
                    Self::decimals(_) => {
                        <decimalsCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::totalSupply(_) => {
                        <totalSupplyCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::balanceOf(_) => {
                        <balanceOfCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::allowance(_) => {
                        <allowanceCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::approve(_) => {
                        <approveCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::transfer(_) => {
                        <transferCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                    Self::transferFrom(_) => {
                        <transferFromCall as alloy_sol_types::SolCall>::SELECTOR
                    }
                }
            }
            #[inline]
            fn selector_at(i: usize) -> ::core::option::Option<[u8; 4]> {
                Self::SELECTORS.get(i).copied()
            }
            #[inline]
            fn valid_selector(selector: [u8; 4]) -> bool {
                Self::SELECTORS.binary_search(&selector).is_ok()
            }
            #[inline]
            #[allow(non_snake_case)]
            fn abi_decode_raw(
                selector: [u8; 4],
                data: &[u8],
            ) -> alloy_sol_types::Result<Self> {
                static DECODE_SHIMS: &[fn(
                    &[u8],
                ) -> alloy_sol_types::Result<IERC20Calls>] = &[
                    {
                        fn name(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <nameCall as alloy_sol_types::SolCall>::abi_decode_raw(data)
                                .map(IERC20Calls::name)
                        }
                        name
                    },
                    {
                        fn approve(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <approveCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::approve)
                        }
                        approve
                    },
                    {
                        fn totalSupply(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::totalSupply)
                        }
                        totalSupply
                    },
                    {
                        fn transferFrom(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::transferFrom)
                        }
                        transferFrom
                    },
                    {
                        fn decimals(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::decimals)
                        }
                        decimals
                    },
                    {
                        fn balanceOf(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::balanceOf)
                        }
                        balanceOf
                    },
                    {
                        fn symbol(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::symbol)
                        }
                        symbol
                    },
                    {
                        fn transfer(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <transferCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::transfer)
                        }
                        transfer
                    },
                    {
                        fn allowance(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw(
                                    data,
                                )
                                .map(IERC20Calls::allowance)
                        }
                        allowance
                    },
                ];
                let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                    return Err(
                        alloy_sol_types::Error::unknown_selector(
                            <Self as alloy_sol_types::SolInterface>::NAME,
                            selector,
                        ),
                    );
                };
                DECODE_SHIMS[idx](data)
            }
            #[inline]
            #[allow(non_snake_case)]
            fn abi_decode_raw_validate(
                selector: [u8; 4],
                data: &[u8],
            ) -> alloy_sol_types::Result<Self> {
                static DECODE_VALIDATE_SHIMS: &[fn(
                    &[u8],
                ) -> alloy_sol_types::Result<IERC20Calls>] = &[
                    {
                        fn name(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <nameCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::name)
                        }
                        name
                    },
                    {
                        fn approve(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <approveCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::approve)
                        }
                        approve
                    },
                    {
                        fn totalSupply(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <totalSupplyCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::totalSupply)
                        }
                        totalSupply
                    },
                    {
                        fn transferFrom(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <transferFromCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::transferFrom)
                        }
                        transferFrom
                    },
                    {
                        fn decimals(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <decimalsCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::decimals)
                        }
                        decimals
                    },
                    {
                        fn balanceOf(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <balanceOfCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::balanceOf)
                        }
                        balanceOf
                    },
                    {
                        fn symbol(data: &[u8]) -> alloy_sol_types::Result<IERC20Calls> {
                            <symbolCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::symbol)
                        }
                        symbol
                    },
                    {
                        fn transfer(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <transferCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::transfer)
                        }
                        transfer
                    },
                    {
                        fn allowance(
                            data: &[u8],
                        ) -> alloy_sol_types::Result<IERC20Calls> {
                            <allowanceCall as alloy_sol_types::SolCall>::abi_decode_raw_validate(
                                    data,
                                )
                                .map(IERC20Calls::allowance)
                        }
                        allowance
                    },
                ];
                let Ok(idx) = Self::SELECTORS.binary_search(&selector) else {
                    return Err(
                        alloy_sol_types::Error::unknown_selector(
                            <Self as alloy_sol_types::SolInterface>::NAME,
                            selector,
                        ),
                    );
                };
                DECODE_VALIDATE_SHIMS[idx](data)
            }
            #[inline]
            fn abi_encoded_size(&self) -> usize {
                match self {
                    Self::name(inner) => {
                        <nameCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                    }
                    Self::symbol(inner) => {
                        <symbolCall as alloy_sol_types::SolCall>::abi_encoded_size(inner)
                    }
                    Self::decimals(inner) => {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::totalSupply(inner) => {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::balanceOf(inner) => {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::allowance(inner) => {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::approve(inner) => {
                        <approveCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::transfer(inner) => {
                        <transferCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                    Self::transferFrom(inner) => {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_encoded_size(
                            inner,
                        )
                    }
                }
            }
            #[inline]
            fn abi_encode_raw(&self, out: &mut alloy_sol_types::private::Vec<u8>) {
                match self {
                    Self::name(inner) => {
                        <nameCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::symbol(inner) => {
                        <symbolCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::decimals(inner) => {
                        <decimalsCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::totalSupply(inner) => {
                        <totalSupplyCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::balanceOf(inner) => {
                        <balanceOfCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::allowance(inner) => {
                        <allowanceCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::approve(inner) => {
                        <approveCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::transfer(inner) => {
                        <transferCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                    Self::transferFrom(inner) => {
                        <transferFromCall as alloy_sol_types::SolCall>::abi_encode_raw(
                            inner,
                            out,
                        )
                    }
                }
            }
        }
        ///Container for all the [`IERC20`](self) events.
        pub enum IERC20Events {
            #[allow(missing_docs)]
            Approval(Approval),
            #[allow(missing_docs)]
            Transfer(Transfer),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for IERC20Events {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    IERC20Events::Approval(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Approval",
                            &__self_0,
                        )
                    }
                    IERC20Events::Transfer(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Transfer",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for IERC20Events {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for IERC20Events {
            #[inline]
            fn eq(&self, other: &IERC20Events) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (
                            IERC20Events::Approval(__self_0),
                            IERC20Events::Approval(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        (
                            IERC20Events::Transfer(__self_0),
                            IERC20Events::Transfer(__arg1_0),
                        ) => __self_0 == __arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[automatically_derived]
        impl ::core::cmp::Eq for IERC20Events {
            #[inline]
            #[doc(hidden)]
            #[coverage(off)]
            fn assert_receiver_is_total_eq(&self) -> () {
                let _: ::core::cmp::AssertParamIsEq<Approval>;
                let _: ::core::cmp::AssertParamIsEq<Transfer>;
            }
        }
        #[automatically_derived]
        impl IERC20Events {
            /// All the selectors of this enum.
            ///
            /// Note that the selectors might not be in the same order as the variants.
            /// No guarantees are made about the order of the selectors.
            ///
            /// Prefer using `SolInterface` methods instead.
            pub const SELECTORS: &'static [[u8; 32usize]] = &[
                [
                    140u8, 91u8, 225u8, 229u8, 235u8, 236u8, 125u8, 91u8, 209u8, 79u8,
                    113u8, 66u8, 125u8, 30u8, 132u8, 243u8, 221u8, 3u8, 20u8, 192u8,
                    247u8, 178u8, 41u8, 30u8, 91u8, 32u8, 10u8, 200u8, 199u8, 195u8,
                    185u8, 37u8,
                ],
                [
                    221u8, 242u8, 82u8, 173u8, 27u8, 226u8, 200u8, 155u8, 105u8, 194u8,
                    176u8, 104u8, 252u8, 55u8, 141u8, 170u8, 149u8, 43u8, 167u8, 241u8,
                    99u8, 196u8, 161u8, 22u8, 40u8, 245u8, 90u8, 77u8, 245u8, 35u8,
                    179u8, 239u8,
                ],
            ];
        }
        #[automatically_derived]
        impl alloy_sol_types::SolEventInterface for IERC20Events {
            const NAME: &'static str = "IERC20Events";
            const COUNT: usize = 2usize;
            fn decode_raw_log(
                topics: &[alloy_sol_types::Word],
                data: &[u8],
            ) -> alloy_sol_types::Result<Self> {
                match topics.first().copied() {
                    Some(<Approval as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                        <Approval as alloy_sol_types::SolEvent>::decode_raw_log(
                                topics,
                                data,
                            )
                            .map(Self::Approval)
                    }
                    Some(<Transfer as alloy_sol_types::SolEvent>::SIGNATURE_HASH) => {
                        <Transfer as alloy_sol_types::SolEvent>::decode_raw_log(
                                topics,
                                data,
                            )
                            .map(Self::Transfer)
                    }
                    _ => {
                        alloy_sol_types::private::Err(alloy_sol_types::Error::InvalidLog {
                            name: <Self as alloy_sol_types::SolEventInterface>::NAME,
                            log: alloy_sol_types::private::Box::new(
                                alloy_sol_types::private::LogData::new_unchecked(
                                    topics.to_vec(),
                                    data.to_vec().into(),
                                ),
                            ),
                        })
                    }
                }
            }
        }
        #[automatically_derived]
        impl alloy_sol_types::private::IntoLogData for IERC20Events {
            fn to_log_data(&self) -> alloy_sol_types::private::LogData {
                match self {
                    Self::Approval(inner) => {
                        alloy_sol_types::private::IntoLogData::to_log_data(inner)
                    }
                    Self::Transfer(inner) => {
                        alloy_sol_types::private::IntoLogData::to_log_data(inner)
                    }
                }
            }
            fn into_log_data(self) -> alloy_sol_types::private::LogData {
                match self {
                    Self::Approval(inner) => {
                        alloy_sol_types::private::IntoLogData::into_log_data(inner)
                    }
                    Self::Transfer(inner) => {
                        alloy_sol_types::private::IntoLogData::into_log_data(inner)
                    }
                }
            }
        }
        use ::alloy_contract as alloy_contract;
        /**Creates a new wrapper around an on-chain [`IERC20`](self) contract instance.

See the [wrapper's documentation](`IERC20Instance`) for more details.*/
        #[inline]
        pub const fn new<
            P: alloy_contract::private::Provider<N>,
            N: alloy_contract::private::Network,
        >(
            address: alloy_sol_types::private::Address,
            provider: P,
        ) -> IERC20Instance<P, N> {
            IERC20Instance::<P, N>::new(address, provider)
        }
        /**A [`IERC20`](self) instance.

Contains type-safe methods for interacting with an on-chain instance of the
[`IERC20`](self) contract located at a given `address`, using a given
provider `P`.

If the contract bytecode is available (see the [`sol!`](alloy_sol_types::sol!)
documentation on how to provide it), the `deploy` and `deploy_builder` methods can
be used to deploy a new instance of the contract.

See the [module-level documentation](self) for all the available methods.*/
        pub struct IERC20Instance<P, N = alloy_contract::private::Ethereum> {
            address: alloy_sol_types::private::Address,
            provider: P,
            _network: ::core::marker::PhantomData<N>,
        }
        #[automatically_derived]
        impl<P: ::core::clone::Clone, N: ::core::clone::Clone> ::core::clone::Clone
        for IERC20Instance<P, N> {
            #[inline]
            fn clone(&self) -> IERC20Instance<P, N> {
                IERC20Instance {
                    address: ::core::clone::Clone::clone(&self.address),
                    provider: ::core::clone::Clone::clone(&self.provider),
                    _network: ::core::clone::Clone::clone(&self._network),
                }
            }
        }
        #[automatically_derived]
        impl<P, N> ::core::fmt::Debug for IERC20Instance<P, N> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.debug_tuple("IERC20Instance").field(&self.address).finish()
            }
        }
        /// Instantiation and getters/setters.
        #[automatically_derived]
        impl<
            P: alloy_contract::private::Provider<N>,
            N: alloy_contract::private::Network,
        > IERC20Instance<P, N> {
            /**Creates a new wrapper around an on-chain [`IERC20`](self) contract instance.

See the [wrapper's documentation](`IERC20Instance`) for more details.*/
            #[inline]
            pub const fn new(
                address: alloy_sol_types::private::Address,
                provider: P,
            ) -> Self {
                Self {
                    address,
                    provider,
                    _network: ::core::marker::PhantomData,
                }
            }
            /// Returns a reference to the address.
            #[inline]
            pub const fn address(&self) -> &alloy_sol_types::private::Address {
                &self.address
            }
            /// Sets the address.
            #[inline]
            pub fn set_address(&mut self, address: alloy_sol_types::private::Address) {
                self.address = address;
            }
            /// Sets the address and returns `self`.
            pub fn at(mut self, address: alloy_sol_types::private::Address) -> Self {
                self.set_address(address);
                self
            }
            /// Returns a reference to the provider.
            #[inline]
            pub const fn provider(&self) -> &P {
                &self.provider
            }
        }
        impl<P: ::core::clone::Clone, N> IERC20Instance<&P, N> {
            /// Clones the provider and returns a new instance with the cloned provider.
            #[inline]
            pub fn with_cloned_provider(self) -> IERC20Instance<P, N> {
                IERC20Instance {
                    address: self.address,
                    provider: ::core::clone::Clone::clone(&self.provider),
                    _network: ::core::marker::PhantomData,
                }
            }
        }
        /// Function calls.
        #[automatically_derived]
        impl<
            P: alloy_contract::private::Provider<N>,
            N: alloy_contract::private::Network,
        > IERC20Instance<P, N> {
            /// Creates a new call builder using this contract instance's provider and address.
            ///
            /// Note that the call can be any function call, not just those defined in this
            /// contract. Prefer using the other methods for building type-safe contract calls.
            pub fn call_builder<C: alloy_sol_types::SolCall>(
                &self,
                call: &C,
            ) -> alloy_contract::SolCallBuilder<&P, C, N> {
                alloy_contract::SolCallBuilder::new_sol(
                    &self.provider,
                    &self.address,
                    call,
                )
            }
            ///Creates a new call builder for the [`name`] function.
            pub fn name(&self) -> alloy_contract::SolCallBuilder<&P, nameCall, N> {
                self.call_builder(&nameCall)
            }
            ///Creates a new call builder for the [`symbol`] function.
            pub fn symbol(&self) -> alloy_contract::SolCallBuilder<&P, symbolCall, N> {
                self.call_builder(&symbolCall)
            }
            ///Creates a new call builder for the [`decimals`] function.
            pub fn decimals(
                &self,
            ) -> alloy_contract::SolCallBuilder<&P, decimalsCall, N> {
                self.call_builder(&decimalsCall)
            }
            ///Creates a new call builder for the [`totalSupply`] function.
            pub fn totalSupply(
                &self,
            ) -> alloy_contract::SolCallBuilder<&P, totalSupplyCall, N> {
                self.call_builder(&totalSupplyCall)
            }
            ///Creates a new call builder for the [`balanceOf`] function.
            pub fn balanceOf(
                &self,
                owner: ::alloy_sol_types::private::Address,
            ) -> alloy_contract::SolCallBuilder<&P, balanceOfCall, N> {
                self.call_builder(&balanceOfCall { owner })
            }
            ///Creates a new call builder for the [`allowance`] function.
            pub fn allowance(
                &self,
                owner: ::alloy_sol_types::private::Address,
                spender: ::alloy_sol_types::private::Address,
            ) -> alloy_contract::SolCallBuilder<&P, allowanceCall, N> {
                self.call_builder(&allowanceCall { owner, spender })
            }
            ///Creates a new call builder for the [`approve`] function.
            pub fn approve(
                &self,
                spender: ::alloy_sol_types::private::Address,
                value: ::alloy_sol_types::private::primitives::aliases::U256,
            ) -> alloy_contract::SolCallBuilder<&P, approveCall, N> {
                self.call_builder(&approveCall { spender, value })
            }
            ///Creates a new call builder for the [`transfer`] function.
            pub fn transfer(
                &self,
                to: ::alloy_sol_types::private::Address,
                value: ::alloy_sol_types::private::primitives::aliases::U256,
            ) -> alloy_contract::SolCallBuilder<&P, transferCall, N> {
                self.call_builder(&transferCall { to, value })
            }
            ///Creates a new call builder for the [`transferFrom`] function.
            pub fn transferFrom(
                &self,
                from: ::alloy_sol_types::private::Address,
                to: ::alloy_sol_types::private::Address,
                value: ::alloy_sol_types::private::primitives::aliases::U256,
            ) -> alloy_contract::SolCallBuilder<&P, transferFromCall, N> {
                self.call_builder(
                    &transferFromCall {
                        from,
                        to,
                        value,
                    },
                )
            }
        }
        /// Event filters.
        #[automatically_derived]
        impl<
            P: alloy_contract::private::Provider<N>,
            N: alloy_contract::private::Network,
        > IERC20Instance<P, N> {
            /// Creates a new event filter using this contract instance's provider and address.
            ///
            /// Note that the type can be any event, not just those defined in this contract.
            /// Prefer using the other methods for building type-safe event filters.
            pub fn event_filter<E: alloy_sol_types::SolEvent>(
                &self,
            ) -> alloy_contract::Event<&P, E, N> {
                alloy_contract::Event::new_sol(&self.provider, &self.address)
            }
            ///Creates a new event filter for the [`Approval`] event.
            pub fn Approval_filter(&self) -> alloy_contract::Event<&P, Approval, N> {
                self.event_filter::<Approval>()
            }
            ///Creates a new event filter for the [`Transfer`] event.
            pub fn Transfer_filter(&self) -> alloy_contract::Event<&P, Transfer, N> {
                self.event_filter::<Transfer>()
            }
        }
    }
}
mod pool_info {
    use alloy::core::primitives::Address;
    use crate::token_info::TokenInfo;
    pub struct PoolInfo {
        pub networks: Vec<String>,
        pub pool_address: Address,
        pub pool_name: String,
        pub token0: Address,
        pub token1: Address,
        pub token0_info: TokenInfo,
        pub token1_info: TokenInfo,
        pub swaps_tracked: usize,
        pub fee: u32,
        pub current_price: f64,
        pub prev_price: f64,
        pub liquidity: u128,
        pub tick_range: (i32, i32),
        pub current_apr: f64,
        pub volume: f64,
        pub swap_store: Vec<(f64, f64, String)>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PoolInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "networks",
                "pool_address",
                "pool_name",
                "token0",
                "token1",
                "token0_info",
                "token1_info",
                "swaps_tracked",
                "fee",
                "current_price",
                "prev_price",
                "liquidity",
                "tick_range",
                "current_apr",
                "volume",
                "swap_store",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.networks,
                &self.pool_address,
                &self.pool_name,
                &self.token0,
                &self.token1,
                &self.token0_info,
                &self.token1_info,
                &self.swaps_tracked,
                &self.fee,
                &self.current_price,
                &self.prev_price,
                &self.liquidity,
                &self.tick_range,
                &self.current_apr,
                &self.volume,
                &&self.swap_store,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "PoolInfo",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PoolInfo {
        #[inline]
        fn clone(&self) -> PoolInfo {
            PoolInfo {
                networks: ::core::clone::Clone::clone(&self.networks),
                pool_address: ::core::clone::Clone::clone(&self.pool_address),
                pool_name: ::core::clone::Clone::clone(&self.pool_name),
                token0: ::core::clone::Clone::clone(&self.token0),
                token1: ::core::clone::Clone::clone(&self.token1),
                token0_info: ::core::clone::Clone::clone(&self.token0_info),
                token1_info: ::core::clone::Clone::clone(&self.token1_info),
                swaps_tracked: ::core::clone::Clone::clone(&self.swaps_tracked),
                fee: ::core::clone::Clone::clone(&self.fee),
                current_price: ::core::clone::Clone::clone(&self.current_price),
                prev_price: ::core::clone::Clone::clone(&self.prev_price),
                liquidity: ::core::clone::Clone::clone(&self.liquidity),
                tick_range: ::core::clone::Clone::clone(&self.tick_range),
                current_apr: ::core::clone::Clone::clone(&self.current_apr),
                volume: ::core::clone::Clone::clone(&self.volume),
                swap_store: ::core::clone::Clone::clone(&self.swap_store),
            }
        }
    }
    impl PoolInfo {
        pub fn new(
            network: String,
            pool_address: Address,
            token0: Address,
            token1: Address,
            token0_info: TokenInfo,
            token1_info: TokenInfo,
            swaps_tracked: usize,
            fee: u32,
            current_price: f64,
            prev_price: f64,
            liquidity: u128,
            tick_range: (i32, i32),
            current_apr: f64,
            volume: f64,
            swap_store: Vec<(f64, f64, String)>,
        ) -> Self {
            let token0_symbol = &token0_info.symbol;
            let token1_symbol = &token1_info.symbol;
            let pool_name = if token0_symbol.is_empty() || token1_symbol.is_empty() {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("Pool-{0:?}", pool_address))
                })
            } else if token0_symbol < token1_symbol {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}/{1}", token0_symbol, token1_symbol),
                    )
                })
            } else {
                ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("{0}/{1}", token1_symbol, token0_symbol),
                    )
                })
            };
            PoolInfo {
                networks: <[_]>::into_vec(::alloc::boxed::box_new([network])),
                pool_address,
                pool_name,
                token0,
                token1,
                token0_info,
                token1_info,
                swaps_tracked,
                fee,
                current_price,
                prev_price,
                liquidity,
                tick_range,
                current_apr,
                volume,
                swap_store,
            }
        }
        pub fn increment_swap_count(&mut self) {
            self.swaps_tracked += 1;
        }
        pub fn update_current_price(&mut self, new_price: f64) {
            self.prev_price = self.current_price;
            self.current_price = new_price;
        }
        pub fn get_price_change_color(&self) -> ratatui::style::Color {
            if self.current_price > self.prev_price && self.prev_price != 0.0 {
                ratatui::style::Color::Green
            } else if self.current_price < self.prev_price {
                ratatui::style::Color::Red
            } else {
                ratatui::style::Color::White
            }
        }
        pub fn get_token0_decimals(&self) -> u8 {
            self.token0_info.decimals
        }
        pub fn get_token1_decimals(&self) -> u8 {
            self.token1_info.decimals
        }
        pub fn update_liquidity(&mut self, new_liquidity: u128) {
            self.liquidity = new_liquidity;
        }
        pub fn get_fee_percent(&self) -> f64 {
            self.fee as f64 / 10000.0
        }
        pub fn update_current_apr(&mut self, new_apr: f64) {
            self.current_apr = new_apr;
        }
        pub fn add_volume(&mut self, this_swap_volume: f64) {
            self.volume += this_swap_volume;
        }
        pub fn add_swap_store(&mut self, amount0: f64, amount1: f64, timestamp: String) {
            const MAX_SWAP_HISTORY: usize = 80;
            self.swap_store.push((amount0, amount1, timestamp));
            if self.swap_store.len() > MAX_SWAP_HISTORY {
                self.swap_store = self
                    .swap_store
                    .split_off(self.swap_store.len() - MAX_SWAP_HISTORY);
            }
        }
        pub fn get_token0_symbol(&self) -> &str {
            &self.token0_info.symbol
        }
        pub fn get_token1_symbol(&self) -> &str {
            &self.token1_info.symbol
        }
    }
}
use pool_info::PoolInfo;
mod ui {
    use std::collections::HashMap;
    use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
    use ratatui::{
        buffer::Buffer, layout::{Rect, Constraint},
        style::Stylize, text::{Line, Text},
        widgets::{Block, Paragraph, Widget, Table, Row, Cell},
        DefaultTerminal, Frame,
    };
    use anyhow::Result;
    use tokio::sync::mpsc::Receiver;
    use std::cmp::min;
    use alloy::core::primitives::Address;
    use crate::pool_info::PoolInfo;
    use crate::token_info::TokenInfo;
    use crate::backend_update::BackendUpdate;
    pub struct TerminalUI {
        exit: bool,
        total_swaps: usize,
        rx: Option<Receiver<BackendUpdate>>,
        scroll_offset: usize,
        selected_pool_index: usize,
        paused: bool,
        eth_swaps: usize,
        base_swaps: usize,
        arb_swaps: usize,
        show_prices: bool,
        pool_info_map: HashMap<Address, PoolInfo>,
        token_info_map: HashMap<Address, TokenInfo>,
        token_scroll_offset: usize,
        selected_token_index: usize,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TerminalUI {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "exit",
                "total_swaps",
                "rx",
                "scroll_offset",
                "selected_pool_index",
                "paused",
                "eth_swaps",
                "base_swaps",
                "arb_swaps",
                "show_prices",
                "pool_info_map",
                "token_info_map",
                "token_scroll_offset",
                "selected_token_index",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.exit,
                &self.total_swaps,
                &self.rx,
                &self.scroll_offset,
                &self.selected_pool_index,
                &self.paused,
                &self.eth_swaps,
                &self.base_swaps,
                &self.arb_swaps,
                &self.show_prices,
                &self.pool_info_map,
                &self.token_info_map,
                &self.token_scroll_offset,
                &&self.selected_token_index,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "TerminalUI",
                names,
                values,
            )
        }
    }
    impl Default for TerminalUI {
        fn default() -> Self {
            Self {
                exit: false,
                total_swaps: 0,
                rx: None,
                scroll_offset: 0,
                selected_pool_index: 0,
                paused: false,
                eth_swaps: 0,
                base_swaps: 0,
                arb_swaps: 0,
                show_prices: false,
                pool_info_map: HashMap::new(),
                token_info_map: HashMap::new(),
                token_scroll_offset: 0,
                selected_token_index: 0,
            }
        }
    }
    impl TerminalUI {
        pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
            while !self.exit {
                let handle_events = if let Some(_rx) = &mut self.rx {
                    event::poll(std::time::Duration::from_millis(10))?
                } else {
                    self.handle_events()?;
                    false
                };
                if handle_events {
                    self.handle_events()?;
                }
                if let Some(rx) = &mut self.rx {
                    match rx.try_recv() {
                        Ok(backend_update) => {
                            match backend_update {
                                BackendUpdate::PoolUpdated(pool) => {
                                    self.pool_info_map.insert(pool.pool_address, pool);
                                    self.total_swaps = self
                                        .pool_info_map
                                        .values()
                                        .map(|p| p.swaps_tracked)
                                        .sum();
                                }
                                BackendUpdate::TokenUpdated(token) => {
                                    self.token_info_map.insert(token.address, token);
                                }
                                BackendUpdate::ChainStats {
                                    eth_swaps,
                                    base_swaps,
                                    arb_swaps,
                                } => {
                                    self.eth_swaps = eth_swaps;
                                    self.base_swaps = base_swaps;
                                    self.arb_swaps = arb_swaps;
                                }
                            }
                        }
                        Err(tokio::sync::mpsc::error::TryRecvError::Empty) => {}
                        Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => {
                            self.rx = None;
                        }
                    }
                }
                if !self.paused {
                    terminal.draw(|frame| self.draw(frame))?;
                }
                std::thread::sleep(std::time::Duration::from_millis(50));
            }
            Ok(())
        }
        fn draw(&self, frame: &mut Frame) {
            frame.render_widget(self, frame.area());
        }
        fn handle_events(&mut self) -> Result<()> {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
            Ok(())
        }
        fn handle_key_event(&mut self, key_event: KeyEvent) {
            match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('p') => {
                    self.paused = !self.paused;
                }
                KeyCode::Char('t') => self.show_prices = !self.show_prices,
                KeyCode::Up | KeyCode::Char('k') => {
                    if self.selected_pool_index > 0 {
                        self.selected_pool_index -= 1;
                        if self.selected_pool_index < self.scroll_offset {
                            self.scroll_offset = self.selected_pool_index;
                        }
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if self.selected_pool_index
                        < self.pool_info_map.len().saturating_sub(1)
                    {
                        self.selected_pool_index += 1;
                        let max_visible = 10;
                        if self.selected_pool_index >= self.scroll_offset + max_visible {
                            self.scroll_offset = self.selected_pool_index + 1
                                - max_visible;
                        }
                    }
                }
                KeyCode::PageUp => {
                    let max_visible = 10;
                    if self.scroll_offset > 0 {
                        self.scroll_offset = self
                            .scroll_offset
                            .saturating_sub(max_visible);
                        self.selected_pool_index = self.scroll_offset;
                    }
                }
                KeyCode::PageDown => {
                    let max_visible = 10;
                    let max_scroll = self
                        .pool_info_map
                        .len()
                        .saturating_sub(max_visible);
                    if self.scroll_offset < max_scroll {
                        self.scroll_offset = (self.scroll_offset + max_visible)
                            .min(max_scroll);
                        self.selected_pool_index = self.scroll_offset;
                    }
                }
                KeyCode::Home => {
                    self.scroll_offset = 0;
                }
                KeyCode::End => {
                    self.scroll_offset = self.pool_info_map.len().saturating_sub(1);
                }
                KeyCode::Char('w') => {
                    if self.selected_token_index > 0 {
                        self.selected_token_index -= 1;
                        if self.selected_token_index < self.token_scroll_offset {
                            self.token_scroll_offset = self.selected_token_index;
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if self.selected_token_index
                        < self.token_info_map.len().saturating_sub(1)
                    {
                        self.selected_token_index += 1;
                        let max_visible = 10;
                        if self.selected_token_index
                            >= self.token_scroll_offset + max_visible
                        {
                            self.token_scroll_offset = self.selected_token_index + 1
                                - max_visible;
                        }
                    }
                }
                _ => {}
            }
        }
        fn exit(&mut self) {
            self.exit = true;
        }
        pub fn is_exiting(&self) -> bool {
            self.exit
        }
        pub fn with_receiver(rx: Receiver<BackendUpdate>) -> Self {
            Self {
                exit: false,
                total_swaps: 0,
                rx: Some(rx),
                scroll_offset: 0,
                selected_pool_index: 0,
                paused: false,
                eth_swaps: 0,
                base_swaps: 0,
                arb_swaps: 0,
                show_prices: false,
                pool_info_map: HashMap::new(),
                token_info_map: HashMap::new(),
                token_scroll_offset: 0,
                selected_token_index: 0,
            }
        }
    }
    impl Widget for &TerminalUI {
        fn render(self, area: Rect, buf: &mut Buffer) {
            let instructions = Line::from(
                <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        " Up (Pool/Price) ".into(),
                        "<↑/w>".blue().bold(),
                        " Down (Pool/Price) ".into(),
                        "<↓/s>".blue().bold(),
                        " Page Up ".into(),
                        "<PgUp>".blue().bold(),
                        " Page Down ".into(),
                        "<PgDn>".blue().bold(),
                        " Show Prices ".into(),
                        "<T>".blue().bold(),
                        " Pause ".into(),
                        "<P>".blue().bold(),
                        " Quit ".into(),
                        "<Q> ".blue().bold(),
                    ]),
                ),
            );
            let title = Line::from(
                " Uniswap v3 Swap Tracker (ARB, BASE, ETH) ".magenta().bold(),
            );
            let block = Block::bordered()
                .title(title.centered())
                .title_bottom(instructions.centered())
                .border_type(ratatui::widgets::BorderType::Rounded);
            block.clone().render(area, buf);
            let inner_area = block.inner(area);
            let header = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        " Pools Tracked: {0} | Swaps Tracked: {1} | Tokens Tracked: {2} | ETH Swaps: {3} | BASE Swaps: {4} | ARB Swaps: {5} ",
                        self.pool_info_map.len(),
                        self.total_swaps,
                        self.token_info_map.len(),
                        self.eth_swaps,
                        self.base_swaps,
                        self.arb_swaps,
                    ),
                )
            });
            let header_text = Text::from(header);
            Paragraph::new(header_text)
                .render(Rect::new(inner_area.x, inner_area.y, inner_area.width, 1), buf);
            let panes = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
                .split(
                    Rect::new(
                        inner_area.x,
                        inner_area.y + 2,
                        inner_area.width,
                        inner_area.height.saturating_sub(2),
                    ),
                );
            let pools_area = panes[0];
            let right_area = panes[1];
            let mut sorted_pools: Vec<PoolInfo> = self
                .pool_info_map
                .values()
                .cloned()
                .collect();
            sorted_pools.sort_by(|a, b| b.swaps_tracked.cmp(&a.swaps_tracked));
            let pool_count = sorted_pools.len();
            let max_visible = pools_area.height as usize;
            let start_idx: usize = if pool_count <= max_visible {
                0
            } else {
                let max_start = pool_count.saturating_sub(max_visible);
                self.scroll_offset.min(max_start)
            };
            let visible_pools = &sorted_pools[start_idx..min(
                start_idx + max_visible,
                pool_count,
            )];
            let headers = Row::new(
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Cell::from("ID"),
                            Cell::from("Pool Name"),
                            Cell::from("Chain"),
                            Cell::from("Fee"),
                            Cell::from("Swaps"),
                            Cell::from("Price"),
                            Cell::from("Liquidity"),
                            Cell::from("Lower Tick"),
                            Cell::from("Upper Tick"),
                            Cell::from("APR"),
                            Cell::from("Volume (ETH)"),
                        ]),
                    ),
                )
                .style(
                    ratatui::style::Style::default()
                        .fg(ratatui::style::Color::Cyan)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                );
            let rows: Vec<Row> = visible_pools
                .iter()
                .enumerate()
                .map(|(i, pool)| {
                    let is_selected = start_idx + i == self.selected_pool_index;
                    let price_display = {
                        let price = pool.current_price;
                        let price_text = if price == 0.0 {
                            "Price unknown".to_string()
                        } else if price >= 1_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("${0:.2}M", price / 1_000_000.0),
                                )
                            })
                        } else if price >= 1000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0:.2}", price))
                            })
                        } else if price >= 1.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0:.4}", price))
                            })
                        } else if price >= 0.01 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0:.6}", price))
                            })
                        } else if price >= 0.00001 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0:.8}", price))
                            })
                        } else {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0:.10}", price))
                            })
                        };
                        let color = pool.get_price_change_color();
                        if is_selected {
                            if color == ratatui::style::Color::White {
                                Cell::from(price_text)
                                    .style(
                                        ratatui::style::Style::default()
                                            .fg(ratatui::style::Color::Yellow)
                                            .add_modifier(ratatui::style::Modifier::BOLD),
                                    )
                            } else {
                                Cell::from(price_text)
                                    .style(
                                        ratatui::style::Style::default()
                                            .fg(color)
                                            .add_modifier(ratatui::style::Modifier::BOLD),
                                    )
                            }
                        } else {
                            Cell::from(price_text)
                                .style(ratatui::style::Style::default().fg(color))
                        }
                    };
                    let liquidity_display = {
                        let liquidity = pool.liquidity;
                        if liquidity >= 1_000_000_000_000_000_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0:.2}Q",
                                        liquidity as f64 / 1_000_000_000_000_000_000.0,
                                    ),
                                )
                            })
                        } else if liquidity >= 1_000_000_000_000_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0:.2}Qd",
                                        liquidity as f64 / 1_000_000_000_000_000.0,
                                    ),
                                )
                            })
                        } else if liquidity >= 1_000_000_000_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0:.2}T",
                                        liquidity as f64 / 1_000_000_000_000.0,
                                    ),
                                )
                            })
                        } else if liquidity >= 1_000_000_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}B", liquidity as f64 / 1_000_000_000.0),
                                )
                            })
                        } else if liquidity >= 1_000_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}M", liquidity as f64 / 1_000_000.0),
                                )
                            })
                        } else if liquidity >= 1_000 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}K", liquidity as f64 / 1_000.0),
                                )
                            })
                        } else {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("{0}", liquidity))
                            })
                        }
                    };
                    let (lower_tick, upper_tick) = pool.tick_range;
                    fn format_volume(volume: f64) -> String {
                        if volume >= 1_000_000_000_000_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "{0:.2}Qd",
                                        volume / 1_000_000_000_000_000_000.0,
                                    ),
                                )
                            })
                        } else if volume >= 1_000_000_000_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}Q", volume / 1_000_000_000_000_000.0),
                                )
                            })
                        } else if volume >= 1_000_000_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}T", volume / 1_000_000_000_000.0),
                                )
                            })
                        } else if volume >= 1_000_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}B", volume / 1_000_000_000.0),
                                )
                            })
                        } else if volume >= 1_000_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}M", volume / 1_000_000.0),
                                )
                            })
                        } else if volume >= 1_000.0 {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!("{0:.2}K", volume / 1_000.0),
                                )
                            })
                        } else if volume > 0.0 {
                            if volume.fract() == 0.0 {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("{0:.0}", volume))
                                })
                            } else {
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(format_args!("{0:.2}", volume))
                                })
                            }
                        } else {
                            "0".to_string()
                        }
                    }
                    let style = if is_selected {
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Yellow)
                            .add_modifier(ratatui::style::Modifier::BOLD)
                    } else {
                        ratatui::style::Style::default()
                    };
                    Row::new(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", start_idx + i + 1))
                                        }),
                                    )
                                    .style(style),
                                Cell::from(pool.pool_name.to_string()).style(style),
                                Cell::from(pool.networks.join(", ")).style(style),
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("{0:.2}%", pool.get_fee_percent()),
                                            )
                                        }),
                                    )
                                    .style(style),
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("{0}", pool.swaps_tracked),
                                            )
                                        }),
                                    )
                                    .style(style),
                                price_display,
                                Cell::from(liquidity_display).style(style),
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", lower_tick))
                                        }),
                                    )
                                    .style(style),
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", upper_tick))
                                        }),
                                    )
                                    .style(style),
                                Cell::from(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("{0:.2}%", pool.current_apr),
                                            )
                                        }),
                                    )
                                    .style(style),
                                Cell::from(format_volume(pool.volume)).style(style),
                            ]),
                        ),
                    )
                })
                .collect();
            let table = Table::new(
                    rows,
                    <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Constraint::Length(6),
                            Constraint::Length(21),
                            Constraint::Length(12),
                            Constraint::Length(10),
                            Constraint::Length(10),
                            Constraint::Length(20),
                            Constraint::Length(20),
                            Constraint::Length(15),
                            Constraint::Length(15),
                            Constraint::Length(15),
                            Constraint::Length(15),
                        ]),
                    ),
                )
                .header(headers)
                .block(
                    Block::default()
                        .borders(ratatui::widgets::Borders::ALL)
                        .title(" Pools "),
                );
            table.render(pools_area, buf);
            if self.show_prices {
                let headers = Row::new(
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Cell::from("Token"),
                                Cell::from("Price (USD)"),
                            ]),
                        ),
                    )
                    .style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Cyan)
                            .add_modifier(ratatui::style::Modifier::BOLD),
                    );
                let token_count = self.token_info_map.len();
                let max_visible = right_area.height as usize;
                let start_idx: usize = if token_count <= max_visible {
                    0
                } else {
                    let max_start = token_count.saturating_sub(max_visible);
                    self.token_scroll_offset.min(max_start)
                };
                let visible_tokens: Vec<_> = self
                    .token_info_map
                    .values()
                    .skip(start_idx)
                    .take(max_visible)
                    .collect();
                let rows: Vec<Row> = visible_tokens
                    .iter()
                    .enumerate()
                    .map(|(i, token_info)| {
                        let is_selected = start_idx + i == self.selected_token_index;
                        let price = token_info
                            .value
                            .clone()
                            .unwrap_or("Unknown".to_string());
                        let price_display = if price == "Unknown" {
                            price
                        } else {
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("${0}", price))
                            })
                        };
                        let color = token_info.get_price_change_color();
                        let style = if is_selected {
                            if color == ratatui::style::Color::White {
                                ratatui::style::Style::default()
                                    .fg(ratatui::style::Color::Yellow)
                                    .add_modifier(ratatui::style::Modifier::BOLD)
                            } else {
                                ratatui::style::Style::default()
                                    .fg(color)
                                    .add_modifier(ratatui::style::Modifier::BOLD)
                            }
                        } else {
                            ratatui::style::Style::default().fg(color)
                        };
                        Row::new(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    Cell::from(token_info.symbol.clone()).style(style),
                                    Cell::from(price_display).style(style),
                                ]),
                            ),
                        )
                    })
                    .collect();
                let prices_table = Table::new(
                        rows,
                        <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                Constraint::Length(30),
                                Constraint::Length(30),
                            ]),
                        ),
                    )
                    .header(headers)
                    .block(
                        Block::default()
                            .borders(ratatui::widgets::Borders::ALL)
                            .title(" Prices "),
                    );
                prices_table.render(right_area, buf);
            } else {
                if let Some(selected_pool) = sorted_pools.get(self.selected_pool_index) {
                    let key = selected_pool.pool_address.clone();
                    if let Some(pool) = self.pool_info_map.get(&key) {
                        let swap_store = &pool.swap_store;
                        let headers = Row::new(
                                <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        Cell::from("Timestamp"),
                                        Cell::from(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("Amount of {0}", pool.get_token0_symbol()),
                                                )
                                            }),
                                        ),
                                        Cell::from(
                                            ::alloc::__export::must_use({
                                                ::alloc::fmt::format(
                                                    format_args!("Amount of {0}", pool.get_token1_symbol()),
                                                )
                                            }),
                                        ),
                                    ]),
                                ),
                            )
                            .style(
                                ratatui::style::Style::default()
                                    .fg(ratatui::style::Color::Cyan)
                                    .add_modifier(ratatui::style::Modifier::BOLD),
                            );
                        let rows: Vec<Row> = if swap_store.is_empty() {
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    Row::new(
                                        <[_]>::into_vec(
                                            ::alloc::boxed::box_new([
                                                Cell::from("No swaps available"),
                                                Cell::from(""),
                                                Cell::from(""),
                                            ]),
                                        ),
                                    ),
                                ]),
                            )
                        } else {
                            swap_store
                                .iter()
                                .rev()
                                .map(|(amount0, amount1, timestamp)| {
                                    Row::new(
                                        <[_]>::into_vec(
                                            ::alloc::boxed::box_new([
                                                Cell::from(
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(format_args!("{0}", timestamp))
                                                    }),
                                                ),
                                                Cell::from(
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(format_args!("{0}", amount0))
                                                    }),
                                                ),
                                                Cell::from(
                                                    ::alloc::__export::must_use({
                                                        ::alloc::fmt::format(format_args!("{0}", amount1))
                                                    }),
                                                ),
                                            ]),
                                        ),
                                    )
                                })
                                .collect()
                        };
                        let swaps_table = Table::new(
                                rows,
                                <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        Constraint::Length(25),
                                        Constraint::Length(20),
                                        Constraint::Length(20),
                                    ]),
                                ),
                            )
                            .header(headers)
                            .block(
                                Block::default()
                                    .borders(ratatui::widgets::Borders::ALL)
                                    .title(
                                        ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!(" Swaps for Pool: {0} ", pool.pool_name),
                                            )
                                        }),
                                    ),
                            );
                        swaps_table.render(right_area, buf);
                    }
                } else {
                    let headers = Row::new(
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    Cell::from("Timestamp"),
                                    Cell::from("Amount0"),
                                    Cell::from("Amount1"),
                                ]),
                            ),
                        )
                        .style(
                            ratatui::style::Style::default()
                                .fg(ratatui::style::Color::Cyan)
                                .add_modifier(ratatui::style::Modifier::BOLD),
                        );
                    let rows = <[_]>::into_vec(
                        ::alloc::boxed::box_new([
                            Row::new(
                                <[_]>::into_vec(
                                    ::alloc::boxed::box_new([
                                        Cell::from("No pools available"),
                                        Cell::from(""),
                                        Cell::from(""),
                                    ]),
                                ),
                            ),
                        ]),
                    );
                    let swaps_table = Table::new(
                            rows,
                            <[_]>::into_vec(
                                ::alloc::boxed::box_new([
                                    Constraint::Length(20),
                                    Constraint::Length(20),
                                    Constraint::Length(20),
                                ]),
                            ),
                        )
                        .header(headers)
                        .block(
                            Block::default()
                                .borders(ratatui::widgets::Borders::ALL)
                                .title(" Track Swaps "),
                        );
                    swaps_table.render(right_area, buf);
                }
            }
        }
    }
}
use ui::TerminalUI;
mod swap_processor {
    use alloy::{
        core::primitives::{Address, U160, U256},
        providers::Provider,
    };
    use anyhow::Result;
    use std::collections::HashMap;
    use amms::amms::uniswap_v3::{
        IUniswapV3Pool::IUniswapV3PoolInstance, IUniswapV3PoolEvents::Swap,
    };
    use std::fs::OpenOptions;
    use std::io::Write;
    use std::str::FromStr;
    use chrono::Local;
    use crate::ierc20::IERC20;
    use crate::pool_info::PoolInfo;
    use crate::token_info::TokenInfo;
    use crate::prices::get_token_price;
    pub async fn process_swap_event<P: Provider + Clone>(
        log: &alloy::rpc::types::Log,
        provider: P,
        network: &str,
        token_info_map: &mut HashMap<Address, TokenInfo>,
        pool_info_map: &mut HashMap<Address, PoolInfo>,
        api_key: &str,
    ) -> Result<(Option<PoolInfo>, Option<TokenInfo>, Option<TokenInfo>)> {
        let data_bytes = &log.data().data;
        if data_bytes.len() < 128 {
            return Err(
                ::anyhow::Error::msg(
                    ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "Swap event data too short: {0}",
                                data_bytes.len(),
                            ),
                        )
                    }),
                ),
            );
        }
        let pool_address = log.address();
        let contract: IUniswapV3PoolInstance<P> = IUniswapV3PoolInstance::new(
            pool_address,
            provider.clone(),
        );
        let swap_event_log = match log.log_decode::<Swap>() {
            Ok(event) => event,
            Err(error) => {
                {
                    #[cold]
                    #[track_caller]
                    #[inline(never)]
                    #[rustc_const_panic_str]
                    #[rustc_do_not_const_check]
                    const fn panic_cold_display<T: ::core::fmt::Display>(arg: &T) -> ! {
                        ::core::panicking::panic_display(arg)
                    }
                    panic_cold_display(&error);
                };
            }
        };
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let amount0 = i128::try_from(swap_event_log.data().amount0).unwrap_or_default();
        let amount1 = i128::try_from(swap_event_log.data().amount1).unwrap_or_default();
        let sqrt_price_x96: U160 = swap_event_log.data().sqrtPriceX96.into();
        let liquidity = swap_event_log.data().liquidity;
        let tick = match swap_event_log.data().tick.try_into() {
            Ok(t) => t,
            Err(_) => {
                {
                    ::std::io::_print(
                        format_args!(
                            "Warning: Missing tick in swap event for pool {0}\n",
                            pool_address,
                        ),
                    );
                };
                0
            }
        };
        let mut updated_token0 = None;
        let mut updated_token1 = None;
        let chain_id = provider.get_chain_id().await.expect("caught error");
        {
            ::std::io::_print(format_args!("chain_id: {0:?}\n", chain_id));
        };
        let token0_address = contract.token0().call().await?;
        {
            ::std::io::_print(format_args!("Got here\n"));
        };
        let token1_address = contract.token1().call().await?;
        if !token_info_map.contains_key(&token0_address) {
            let token_price = get_token_price(
                    network.to_string(),
                    token0_address,
                    api_key,
                )
                .await?;
            let token_price_value = token_price.unwrap_or("Unknown".to_string());
            let ierc20_token0 = IERC20::new(token0_address, provider.clone());
            let token0_symbol = ierc20_token0.symbol().call().await?;
            let token0_decimal = ierc20_token0.decimals().call().await?;
            let new_token = TokenInfo {
                address: token0_address,
                symbol: token0_symbol,
                decimals: token0_decimal,
                value: Some(token_price_value),
                prev_value: None,
                last_updated: String::new(),
            };
            token_info_map.insert(token0_address, new_token.clone());
            updated_token0 = Some(new_token);
        } else {
            let curr_token_price = get_token_price(
                    network.to_string(),
                    token0_address,
                    api_key,
                )
                .await?;
            let curr_token_price_value = curr_token_price
                .unwrap_or("Unknown".to_string());
            if let Some(token_info) = token_info_map.get_mut(&token0_address) {
                token_info.update_value(Some(curr_token_price_value));
                updated_token0 = Some(token_info.clone());
            }
        }
        if !token_info_map.contains_key(&token1_address) {
            let token_price = get_token_price(
                    network.to_string(),
                    token0_address,
                    api_key,
                )
                .await?;
            let token_price_value = token_price.unwrap_or("Unknown".to_string());
            let ierc20_token1 = IERC20::new(token1_address, provider.clone());
            let token1_symbol = ierc20_token1.symbol().call().await?;
            let token1_decimal = ierc20_token1.decimals().call().await?;
            let new_token = TokenInfo {
                address: token1_address,
                symbol: token1_symbol,
                decimals: token1_decimal,
                value: Some(token_price_value),
                prev_value: None,
                last_updated: String::new(),
            };
            token_info_map.insert(token1_address, new_token.clone());
            updated_token1 = Some(new_token);
        } else {
            let curr_token_price = get_token_price(
                    network.to_string(),
                    token1_address,
                    api_key,
                )
                .await?;
            let curr_token_price_value = curr_token_price
                .unwrap_or("Unknown".to_string());
            if let Some(token_info) = token_info_map.get_mut(&token1_address) {
                token_info.update_value(Some(curr_token_price_value));
                updated_token1 = Some(token_info.clone());
            }
        }
        let updated_pool = if !pool_info_map.contains_key(&pool_address) {
            Some(
                process_new_pool(
                        network,
                        pool_address,
                        token0_address,
                        token1_address,
                        contract,
                        token_info_map,
                        pool_info_map,
                        amount0,
                        amount1,
                        sqrt_price_x96,
                        liquidity,
                        tick,
                        timestamp,
                    )
                    .await?,
            )
        } else {
            Some(
                update_existing_pool(
                    network,
                    pool_address,
                    pool_info_map,
                    amount0,
                    amount1,
                    sqrt_price_x96,
                    liquidity,
                    timestamp,
                )?,
            )
        };
        Ok((updated_pool, updated_token0, updated_token1))
    }
    async fn process_new_pool<P: Provider + Clone>(
        network: &str,
        pool_address: Address,
        token0_address: Address,
        token1_address: Address,
        contract: IUniswapV3PoolInstance<P>,
        token_info_map: &mut HashMap<Address, TokenInfo>,
        pool_info_map: &mut HashMap<Address, PoolInfo>,
        amount0: i128,
        amount1: i128,
        sqrt_price_x96: U160,
        liquidity: u128,
        tick: i32,
        timestamp: String,
    ) -> Result<PoolInfo> {
        let fee_uint = contract.fee().call().await?;
        let fee = fee_uint.to::<u32>();
        let token0_decimals = token_info_map.get(&token0_address).unwrap().decimals;
        let token1_decimals = token_info_map.get(&token1_address).unwrap().decimals;
        let current_price = calculate_price_from_sqrt_price_x96(
            sqrt_price_x96,
            token0_decimals,
            token1_decimals,
        );
        let tick_spacing = contract.tickSpacing().call().await?.as_i32();
        let tick_range = calculate_tick_range(tick, tick_spacing);
        let apr = calculate_apr(fee, 1, liquidity);
        let amount0_scaled = amount0.abs() as f64 / 10f64.powi(token0_decimals as i32);
        let amount1_scaled = amount1.abs() as f64 / 10f64.powi(token1_decimals as i32);
        let volume = amount0_scaled + amount1_scaled;
        let readable_amount0 = make_amount_readable(amount0, token0_decimals);
        let readable_amount1 = make_amount_readable(amount1, token1_decimals);
        let swap_store = <[_]>::into_vec(
            ::alloc::boxed::box_new([(readable_amount0, readable_amount1, timestamp)]),
        );
        let simplified_network = simplify_network_name(network);
        let new_pool = PoolInfo::new(
            simplified_network.to_string(),
            pool_address,
            token0_address,
            token1_address,
            token_info_map.get(&token0_address).unwrap().clone(),
            token_info_map.get(&token1_address).unwrap().clone(),
            1,
            fee,
            current_price,
            0.0,
            liquidity,
            tick_range,
            apr,
            volume,
            swap_store,
        );
        pool_info_map.insert(pool_address, new_pool.clone());
        Ok(new_pool)
    }
    fn update_existing_pool(
        network: &str,
        pool_address: Address,
        pool_info_map: &mut HashMap<Address, PoolInfo>,
        amount0: i128,
        amount1: i128,
        sqrt_price_x96: U160,
        liquidity: u128,
        timestamp: String,
    ) -> Result<PoolInfo> {
        if let Some(pool) = pool_info_map.get_mut(&pool_address) {
            pool.increment_swap_count();
            let simplified_network = simplify_network_name(network);
            if !pool.networks.contains(&simplified_network.to_string()) {
                pool.networks.push(simplified_network.to_string());
            }
            let token0_decimals = pool.get_token0_decimals();
            let token1_decimals = pool.get_token1_decimals();
            let new_price = calculate_price_from_sqrt_price_x96(
                sqrt_price_x96,
                token0_decimals,
                token1_decimals,
            );
            let new_apr = calculate_apr(pool.fee, pool.swaps_tracked, pool.liquidity);
            pool.update_current_price(new_price);
            pool.update_liquidity(liquidity);
            pool.update_current_apr(new_apr);
            let amount0_scaled = amount0.abs() as f64
                / 10f64.powi(token0_decimals as i32);
            let amount1_scaled = amount1.abs() as f64
                / 10f64.powi(token1_decimals as i32);
            let volume = amount0_scaled + amount1_scaled;
            pool.add_volume(volume);
            let readable_amount0 = make_amount_readable(amount0, token0_decimals);
            let readable_amount1 = make_amount_readable(amount1, token1_decimals);
            pool.add_swap_store(readable_amount0, readable_amount1, timestamp);
            return Ok(pool.clone());
        } else {
            {
                ::std::io::_print(
                    format_args!("Error: Pool found in HashMap but no index available\n"),
                );
            };
        }
        Err(
            ::anyhow::__private::must_use({
                let error = ::anyhow::__private::format_err(
                    format_args!("Failed to update pool"),
                );
                error
            }),
        )
    }
    pub fn calculate_price_from_sqrt_price_x96(
        sqrt_price_x96: U160,
        decimals_token0: u8,
        decimals_token1: u8,
    ) -> f64 {
        if sqrt_price_x96 == U160::ZERO {
            return 0.0;
        }
        let mut limbs = [0u64; 4];
        let u160_limbs = sqrt_price_x96.into_limbs();
        limbs[0] = u160_limbs[0];
        limbs[1] = u160_limbs[1];
        limbs[2] = u160_limbs[2];
        let sqrt_price_u256 = U256::from_limbs(limbs);
        let sqrt_price_f64 = f64::from_str(&sqrt_price_u256.to_string()).unwrap_or(0.0);
        let sqrt_scale = 2f64.powi(96);
        let price = (sqrt_price_f64 / sqrt_scale).powi(2);
        let decimal_adjustment = 10f64
            .powi((decimals_token0 as i32) - (decimals_token1 as i32));
        let adjusted_price = price * decimal_adjustment;
        if !adjusted_price.is_finite() || adjusted_price > 1e10 || adjusted_price < 1e-10
        {
            if let Ok(mut file) = OpenOptions::new()
                .create(true)
                .append(true)
                .open("price_calculation.log")
            {
                let _ = file
                    .write_fmt(
                        format_args!(
                            "Extreme price: {0} (sqrt: {1}, raw: {2}, adjustment: {3})\n",
                            adjusted_price,
                            sqrt_price_u256,
                            price,
                            decimal_adjustment,
                        ),
                    );
            }
            if adjusted_price > 1e10 {
                return 1e6;
            } else if adjusted_price < 1e-10 && adjusted_price > 0.0 {
                return 1e-6;
            } else {
                return 1.0;
            }
        }
        adjusted_price
    }
    pub fn calculate_tick_range(tick: i32, tick_spacing: i32) -> (i32, i32) {
        let lower_tick = tick - (tick % tick_spacing);
        let upper_tick = lower_tick + tick_spacing;
        (lower_tick, upper_tick)
    }
    pub fn calculate_daily_fees(fee: u32, swaps_tracked: usize) -> f64 {
        let fee_tier = fee as f64 / 1_000_000.0;
        let total_fees = swaps_tracked as f64 * fee_tier;
        total_fees
    }
    pub fn calculate_apr(fee: u32, swaps_tracked: usize, liquidity: u128) -> f64 {
        let daily_fees = calculate_daily_fees(fee, swaps_tracked);
        if liquidity == 0 {
            return 0.0;
        }
        let scaled_liquidity = liquidity as f64 / 1e18;
        let apr = (daily_fees / scaled_liquidity as f64) * 365.0 * 100.0;
        apr
    }
    pub fn make_amount_readable(amount: i128, decimals: u8) -> f64 {
        let divisor = 10f64.powi(decimals as i32);
        amount as f64 / divisor
    }
    pub fn simplify_network_name(network: &str) -> &str {
        match network {
            "eth-mainnet" => "eth",
            "base-mainnet" => "base",
            "arb-mainnet" => "arb",
            _ => network,
        }
    }
}
mod backend {
    use std::{fmt::Debug, io::{self, Write}};
    use alloy::{
        core::primitives::Address, network::Ethereum,
        providers::{
            Identity, Provider, ProviderBuilder, RootProvider, WsConnect,
            fillers::{
                BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
                NonceFiller,
            },
        },
        rpc::types::Filter, sol_types::SolEvent,
    };
    use anyhow::Result;
    use dotenv::dotenv;
    use std::{env, collections::HashMap};
    use amms::amms::uniswap_v3::IUniswapV3PoolEvents::Swap;
    use tokio::sync::mpsc;
    use std::fs::OpenOptions;
    use std::io::BufWriter;
    use futures::stream::{self, StreamExt};
    use crate::pool_info::PoolInfo;
    use crate::swap_processor::process_swap_event;
    use crate::token_info::TokenInfo;
    use crate::backend_update::BackendUpdate;
    use serde::{Deserialize, Serialize};
    struct TokenPriceResponse {
        _price: Option<String>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenPriceResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "_price" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"_price" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TokenPriceResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenPriceResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TokenPriceResponse",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TokenPriceResponse with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TokenPriceResponse {
                            _price: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("_price"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("_price")?
                            }
                        };
                        _serde::__private::Ok(TokenPriceResponse {
                            _price: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["_price"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TokenPriceResponse",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TokenPriceResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPriceResponse {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TokenPriceResponse",
                "_price",
                &&self._price,
            )
        }
    }
    #[serde(rename_all = "kebab-case")]
    pub enum AlchemyNetwork {
        BaseMainnet,
        ArbMainnet,
        EthMainnet,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for AlchemyNetwork {
        #[inline]
        fn clone(&self) -> AlchemyNetwork {
            match self {
                AlchemyNetwork::BaseMainnet => AlchemyNetwork::BaseMainnet,
                AlchemyNetwork::ArbMainnet => AlchemyNetwork::ArbMainnet,
                AlchemyNetwork::EthMainnet => AlchemyNetwork::EthMainnet,
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for AlchemyNetwork {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    AlchemyNetwork::BaseMainnet => "BaseMainnet",
                    AlchemyNetwork::ArbMainnet => "ArbMainnet",
                    AlchemyNetwork::EthMainnet => "EthMainnet",
                },
            )
        }
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for AlchemyNetwork {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                match *self {
                    AlchemyNetwork::BaseMainnet => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AlchemyNetwork",
                            0u32,
                            "base-mainnet",
                        )
                    }
                    AlchemyNetwork::ArbMainnet => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AlchemyNetwork",
                            1u32,
                            "arb-mainnet",
                        )
                    }
                    AlchemyNetwork::EthMainnet => {
                        _serde::Serializer::serialize_unit_variant(
                            __serializer,
                            "AlchemyNetwork",
                            2u32,
                            "eth-mainnet",
                        )
                    }
                }
            }
        }
    };
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for AlchemyNetwork {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 3",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "base-mainnet" => _serde::__private::Ok(__Field::__field0),
                            "arb-mainnet" => _serde::__private::Ok(__Field::__field1),
                            "eth-mainnet" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"base-mainnet" => _serde::__private::Ok(__Field::__field0),
                            b"arb-mainnet" => _serde::__private::Ok(__Field::__field1),
                            b"eth-mainnet" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<AlchemyNetwork>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = AlchemyNetwork;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum AlchemyNetwork",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(AlchemyNetwork::BaseMainnet)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(AlchemyNetwork::ArbMainnet)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(AlchemyNetwork::EthMainnet)
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "base-mainnet",
                    "arb-mainnet",
                    "eth-mainnet",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "AlchemyNetwork",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<AlchemyNetwork>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::hash::Hash for AlchemyNetwork {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for AlchemyNetwork {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for AlchemyNetwork {
        #[inline]
        fn eq(&self, other: &AlchemyNetwork) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for AlchemyNetwork {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    type AlchemyProvider = FillProvider<
        JoinFill<
            Identity,
            JoinFill<
                GasFiller,
                JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
            >,
        >,
        RootProvider,
    >;
    async fn build_clients(
        networks: Vec<AlchemyNetwork>,
        api_key: &str,
    ) -> anyhow::Result<Vec<(AlchemyNetwork, AlchemyProvider)>> {
        let mut providers = ::alloc::vec::Vec::new();
        for network in networks {
            let http_url = ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!("https://{0:?}.g.alchemy.com/v2/{1}", network, api_key),
                )
            });
            {
                ::std::io::_print(format_args!("{0:?}\n", http_url));
            };
            let http_provider = ProviderBuilder::new()
                .network::<Ethereum>()
                .connect_http(http_url.parse()?);
            providers.push((network, http_provider));
        }
        Ok(providers)
    }
    pub async fn run_ws_backend(tx: mpsc::Sender<BackendUpdate>) -> Result<()> {
        dotenv().ok();
        let api_key = match env::var("ALCHEMY_API_KEY") {
            Ok(key) => key,
            Err(e) => {
                {
                    ::std::io::_print(format_args!("Error loading API key: {0}\n", e));
                };
                return Err(
                    ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!("Failed to load ALCHEMY_API_KEY from .env file"),
                        );
                        error
                    }),
                );
            }
        };
        let events = <[_]>::into_vec(::alloc::boxed::box_new([Swap::SIGNATURE_HASH]));
        let swap_filter = Filter::new().event_signature(events);
        let chain_prefixes: Vec<AlchemyNetwork> = <[_]>::into_vec(
            ::alloc::boxed::box_new([
                AlchemyNetwork::EthMainnet,
                AlchemyNetwork::BaseMainnet,
                AlchemyNetwork::ArbMainnet,
            ]),
        );
        let providers = build_clients(chain_prefixes, &api_key)
            .await
            .expect("Should build clients");
        let mut provider_map = HashMap::new();
        for (network, http) in &providers {
            provider_map.insert(network.clone(), http);
        }
        let eth_url = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("https://eth-mainnet.g.alchemy.com/v2/{0}", api_key),
            )
        });
        let eth_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_http(eth_url.parse()?);
        let eth_ws_url = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("wss://eth-mainnet.g.alchemy.com/v2/{0}", api_key),
            )
        });
        let eth_ws_connect = WsConnect::new(&eth_ws_url);
        let eth_ws_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_ws(eth_ws_connect)
            .await?;
        let eth_ws_subcription = eth_ws_provider.subscribe_logs(&swap_filter).await?;
        let eth_ws_stream = eth_ws_subcription
            .into_stream()
            .map(|log| ("eth-mainnet", log))
            .boxed();
        let base_ws_url = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("wss://base-mainnet.g.alchemy.com/v2/{0}", api_key),
            )
        });
        let base_ws_connect = WsConnect::new(&base_ws_url);
        let base_ws_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_ws(base_ws_connect)
            .await?;
        let base_ws_subscription = base_ws_provider.subscribe_logs(&swap_filter).await?;
        let base_ws_stream = base_ws_subscription
            .into_stream()
            .map(|log| ("base-mainnet", log))
            .boxed();
        let arb_ws_url = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!("wss://arb-mainnet.g.alchemy.com/v2/{0}", api_key),
            )
        });
        let arb_ws_connect = WsConnect::new(&arb_ws_url);
        let arb_ws_provider = ProviderBuilder::new()
            .network::<Ethereum>()
            .connect_ws(arb_ws_connect)
            .await?;
        let arb_ws_subscription = arb_ws_provider.subscribe_logs(&swap_filter).await?;
        let arb_ws_stream = arb_ws_subscription
            .into_stream()
            .map(|log: alloy::rpc::types::Log| ("arb-mainnet", log))
            .boxed();
        let mut merged_stream = stream::select_all(
            <[_]>::into_vec(
                ::alloc::boxed::box_new([eth_ws_stream, base_ws_stream, arb_ws_stream]),
            ),
        );
        io::stdout().flush()?;
        let mut token_info_map: HashMap<Address, TokenInfo> = HashMap::new();
        let mut pool_info_map: HashMap<Address, PoolInfo> = HashMap::new();
        let mut eth_swaps = 0;
        let mut base_swaps = 0;
        let mut arb_swaps = 0;
        while let Some((network, log)) = merged_stream.next().await {
            if let Ok(_decode) = log.log_decode::<Swap>() {
                let provider = match network {
                    "eth-mainnet" => {
                        eth_swaps += 1;
                        provider_map
                            .get(&AlchemyNetwork::EthMainnet)
                            .expect("Shold get provider")
                            .clone()
                    }
                    "base-mainnet" => {
                        base_swaps += 1;
                        provider_map
                            .get(&AlchemyNetwork::BaseMainnet)
                            .expect("Shold get provider")
                            .clone()
                    }
                    "arb-mainnet" => {
                        arb_swaps += 1;
                        provider_map
                            .get(&AlchemyNetwork::ArbMainnet)
                            .expect("Shold get provider")
                            .clone()
                    }
                    _ => {
                        continue;
                    }
                };
                match process_swap_event(
                        &log,
                        provider,
                        network,
                        &mut token_info_map,
                        &mut pool_info_map,
                        &api_key,
                    )
                    .await
                {
                    Ok((updated_pool, updated_token0, updated_token1)) => {
                        if let Some(pool) = updated_pool {
                            tx.send(BackendUpdate::PoolUpdated(pool)).await?;
                        }
                        if let Some(token0) = updated_token0 {
                            tx.send(BackendUpdate::TokenUpdated(token0)).await?;
                        }
                        if let Some(token1) = updated_token1 {
                            tx.send(BackendUpdate::TokenUpdated(token1)).await?;
                        }
                    }
                    Err(e) => {
                        let file_result = OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open("swap_errors.log");
                        match file_result {
                            Ok(file) => {
                                let mut writer = BufWriter::new(file);
                                writer
                                    .write_fmt(format_args!("Error processing swap: {0}\n", e))
                                    .ok();
                            }
                            Err(file_err) => {
                                {
                                    ::std::io::_eprint(
                                        format_args!(
                                            "Error processing swap: {0} (couldn\'t open log: {1})\n",
                                            e,
                                            file_err,
                                        ),
                                    );
                                };
                            }
                        }
                    }
                }
                tx.send(BackendUpdate::ChainStats {
                        eth_swaps,
                        base_swaps,
                        arb_swaps,
                    })
                    .await?;
            }
        }
        Ok(())
    }
}
mod token_info {
    use alloy::core::primitives::Address;
    pub struct TokenInfo {
        pub address: Address,
        pub symbol: String,
        pub decimals: u8,
        pub value: Option<String>,
        pub prev_value: Option<String>,
        pub last_updated: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenInfo {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "address",
                "symbol",
                "decimals",
                "value",
                "prev_value",
                "last_updated",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.address,
                &self.symbol,
                &self.decimals,
                &self.value,
                &self.prev_value,
                &&self.last_updated,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "TokenInfo",
                names,
                values,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TokenInfo {
        #[inline]
        fn clone(&self) -> TokenInfo {
            TokenInfo {
                address: ::core::clone::Clone::clone(&self.address),
                symbol: ::core::clone::Clone::clone(&self.symbol),
                decimals: ::core::clone::Clone::clone(&self.decimals),
                value: ::core::clone::Clone::clone(&self.value),
                prev_value: ::core::clone::Clone::clone(&self.prev_value),
                last_updated: ::core::clone::Clone::clone(&self.last_updated),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for TokenInfo {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for TokenInfo {
        #[inline]
        fn eq(&self, other: &TokenInfo) -> bool {
            self.address == other.address && self.symbol == other.symbol
                && self.decimals == other.decimals && self.value == other.value
                && self.prev_value == other.prev_value
                && self.last_updated == other.last_updated
        }
    }
    impl TokenInfo {
        pub fn new(
            address: Address,
            symbol: String,
            decimals: u8,
            value: Option<String>,
        ) -> Self {
            Self {
                address,
                symbol,
                decimals,
                value,
                prev_value: None,
                last_updated: String::new(),
            }
        }
        pub fn update_value(&mut self, new_value: Option<String>) {
            self.prev_value = self.value.clone();
            self.value = new_value;
        }
        pub fn get_price_change_color(&self) -> ratatui::style::Color {
            if let (Some(current), Some(previous)) = (&self.value, &self.prev_value) {
                if let (Ok(current), Ok(previous)) = (
                    current.parse::<f64>(),
                    previous.parse::<f64>(),
                ) {
                    if current > previous {
                        return ratatui::style::Color::Green;
                    } else if current < previous {
                        return ratatui::style::Color::Red;
                    }
                }
            }
            ratatui::style::Color::White
        }
    }
}
mod prices {
    use alloy::core::primitives::Address;
    use serde::{Deserialize, Serialize};
    use reqwest::Client;
    use anyhow::Result;
    struct TokenPriceRequest {
        addresses: Vec<TokenAddress>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TokenPriceRequest {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TokenPriceRequest",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "addresses",
                    &self.addresses,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPriceRequest {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TokenPriceRequest",
                "addresses",
                &&self.addresses,
            )
        }
    }
    struct TokenAddress {
        network: String,
        address: String,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for TokenAddress {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "TokenAddress",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "network",
                    &self.network,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "address",
                    &self.address,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenAddress {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TokenAddress",
                "network",
                &self.network,
                "address",
                &&self.address,
            )
        }
    }
    struct TokenPriceResponse {
        data: Vec<TokenPriceData>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenPriceResponse {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "data" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"data" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TokenPriceResponse>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenPriceResponse;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TokenPriceResponse",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Vec<TokenPriceData>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TokenPriceResponse with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TokenPriceResponse {
                            data: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<
                            Vec<TokenPriceData>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("data"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<TokenPriceData>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("data")?
                            }
                        };
                        _serde::__private::Ok(TokenPriceResponse {
                            data: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["data"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TokenPriceResponse",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TokenPriceResponse>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPriceResponse {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TokenPriceResponse",
                "data",
                &&self.data,
            )
        }
    }
    struct TokenPriceError {
        _message: String,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenPriceError {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "_message" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"_message" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TokenPriceError>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenPriceError;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TokenPriceError",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TokenPriceError with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TokenPriceError {
                            _message: __field0,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "_message",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("_message")?
                            }
                        };
                        _serde::__private::Ok(TokenPriceError {
                            _message: __field0,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["_message"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TokenPriceError",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TokenPriceError>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPriceError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TokenPriceError",
                "_message",
                &&self._message,
            )
        }
    }
    struct TokenPriceData {
        network: String,
        address: String,
        prices: Vec<TokenPrice>,
        error: Option<TokenPriceError>,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenPriceData {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "network" => _serde::__private::Ok(__Field::__field0),
                            "address" => _serde::__private::Ok(__Field::__field1),
                            "prices" => _serde::__private::Ok(__Field::__field2),
                            "error" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"network" => _serde::__private::Ok(__Field::__field0),
                            b"address" => _serde::__private::Ok(__Field::__field1),
                            b"prices" => _serde::__private::Ok(__Field::__field2),
                            b"error" => _serde::__private::Ok(__Field::__field3),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TokenPriceData>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenPriceData;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TokenPriceData",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TokenPriceData with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TokenPriceData with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Vec<TokenPrice>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct TokenPriceData with 4 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<TokenPriceError>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct TokenPriceData with 4 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TokenPriceData {
                            network: __field0,
                            address: __field1,
                            prices: __field2,
                            error: __field3,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Vec<TokenPrice>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<
                            Option<TokenPriceError>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "network",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "address",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("prices"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Vec<TokenPrice>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("error"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<TokenPriceError>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("network")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("address")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("prices")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("error")?
                            }
                        };
                        _serde::__private::Ok(TokenPriceData {
                            network: __field0,
                            address: __field1,
                            prices: __field2,
                            error: __field3,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "network",
                    "address",
                    "prices",
                    "error",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TokenPriceData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TokenPriceData>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPriceData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "TokenPriceData",
                "network",
                &self.network,
                "address",
                &self.address,
                "prices",
                &self.prices,
                "error",
                &&self.error,
            )
        }
    }
    struct TokenPrice {
        currency: String,
        value: String,
        lastUpdatedAt: String,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TokenPrice {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "currency" => _serde::__private::Ok(__Field::__field0),
                            "value" => _serde::__private::Ok(__Field::__field1),
                            "lastUpdatedAt" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"currency" => _serde::__private::Ok(__Field::__field0),
                            b"value" => _serde::__private::Ok(__Field::__field1),
                            b"lastUpdatedAt" => _serde::__private::Ok(__Field::__field2),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TokenPrice>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TokenPrice;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TokenPrice",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct TokenPrice with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct TokenPrice with 3 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct TokenPrice with 3 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(TokenPrice {
                            currency: __field0,
                            value: __field1,
                            lastUpdatedAt: __field2,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "currency",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("value"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "lastUpdatedAt",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("currency")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("value")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("lastUpdatedAt")?
                            }
                        };
                        _serde::__private::Ok(TokenPrice {
                            currency: __field0,
                            value: __field1,
                            lastUpdatedAt: __field2,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "currency",
                    "value",
                    "lastUpdatedAt",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "TokenPrice",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TokenPrice>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::fmt::Debug for TokenPrice {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TokenPrice",
                "currency",
                &self.currency,
                "value",
                &self.value,
                "lastUpdatedAt",
                &&self.lastUpdatedAt,
            )
        }
    }
    pub async fn get_token_price(
        network: String,
        token_address: Address,
        api_key: &str,
    ) -> Result<Option<String>> {
        let price_url = ::alloc::__export::must_use({
            ::alloc::fmt::format(
                format_args!(
                    "https://api.g.alchemy.com/prices/v1/{0}/tokens/by-address",
                    api_key,
                ),
            )
        });
        let request_body = TokenPriceRequest {
            addresses: <[_]>::into_vec(
                ::alloc::boxed::box_new([
                    TokenAddress {
                        network,
                        address: ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0:?}", token_address))
                        }),
                    },
                ]),
            ),
        };
        let client = Client::new();
        let response = client.post(&price_url).json(&request_body).send().await?;
        if !response.status().is_success() {
            {
                ::std::io::_print(
                    format_args!("Failed to fetch token price: {0}\n", response.status()),
                );
            };
            return Ok(None);
        }
        let body_text = response.text().await?;
        let price_response: TokenPriceResponse = serde_json::from_str(&body_text)?;
        if let Some(price_data) = price_response.data.first() {
            if let Some(_error_obj) = &price_data.error {
                return Ok(None);
            }
            if price_data.prices.is_empty() {
                return Ok(None);
            }
            if let Some(price) = price_data.prices.first() {
                return Ok(Some(price.value.clone()));
            }
        }
        Ok(None)
    }
}
use token_info::TokenInfo;
mod backend_update {
    use crate::PoolInfo;
    use crate::TokenInfo;
    pub enum BackendUpdate {
        PoolUpdated(PoolInfo),
        TokenUpdated(TokenInfo),
        ChainStats { eth_swaps: usize, base_swaps: usize, arb_swaps: usize },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for BackendUpdate {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BackendUpdate::PoolUpdated(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PoolUpdated",
                        &__self_0,
                    )
                }
                BackendUpdate::TokenUpdated(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "TokenUpdated",
                        &__self_0,
                    )
                }
                BackendUpdate::ChainStats {
                    eth_swaps: __self_0,
                    base_swaps: __self_1,
                    arb_swaps: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "ChainStats",
                        "eth_swaps",
                        __self_0,
                        "base_swaps",
                        __self_1,
                        "arb_swaps",
                        &__self_2,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for BackendUpdate {
        #[inline]
        fn clone(&self) -> BackendUpdate {
            match self {
                BackendUpdate::PoolUpdated(__self_0) => {
                    BackendUpdate::PoolUpdated(::core::clone::Clone::clone(__self_0))
                }
                BackendUpdate::TokenUpdated(__self_0) => {
                    BackendUpdate::TokenUpdated(::core::clone::Clone::clone(__self_0))
                }
                BackendUpdate::ChainStats {
                    eth_swaps: __self_0,
                    base_swaps: __self_1,
                    arb_swaps: __self_2,
                } => {
                    BackendUpdate::ChainStats {
                        eth_swaps: ::core::clone::Clone::clone(__self_0),
                        base_swaps: ::core::clone::Clone::clone(__self_1),
                        arb_swaps: ::core::clone::Clone::clone(__self_2),
                    }
                }
            }
        }
    }
}
fn main() -> Result<()> {
    let body = async {
        let (tx, rx) = mpsc::channel::<BackendUpdate>(100);
        match crate::backend::run_ws_backend(tx).await {
            Ok(_) => {
                ::std::io::_print(format_args!("Backend finished successfully\n"));
            }
            Err(e) => {
                ::std::io::_eprint(format_args!("Backend error: {0}\n", e));
            }
        };
        Ok(())
    };
    #[allow(
        clippy::expect_used,
        clippy::diverging_sub_expression,
        clippy::needless_return
    )]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
