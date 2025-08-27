macro_rules! ink {
    () => {
        #![cfg_attr(not(feature = "std"), no_std)]
        #![cfg_attr(not(feature = "std"), no_main)]

        mod ink {
            pub use ::ink::prelude::vec::Drain as VecDrain;
            pub use ::ink::prelude::vec::ExtractIf as VecExtractIf;
            pub use ::ink::prelude::vec::IntoIter as VecIntoIter;
            pub use ::ink::prelude::vec::Splice as VecSplice;
            pub use ::ink::prelude::vec::Vec;
            pub use ::ink::prelude::string::Drain as StringDrain;
            pub use ::ink::prelude::string::FromUtf16Error;
            pub use ::ink::prelude::string::FromUtf8Error;
            pub use ::ink::prelude::string::ParseError;
            pub use ::ink::prelude::string::String;
            pub use ::ink::prelude::string::ToString;
            pub use ::ink::prelude::boxed::Box;
            pub use ::ink::primitives::AccountId as Address;
            pub use ::ink::contract_ref;
            pub use ::ink::contract;
            pub use ::ink::storage::Lazy;
            pub use ::ink::storage::Mapping;
            pub use ::ink::storage::StorageVec;
            pub use ::ink::env::DefaultEnvironment;
            pub use ::ink::env::Environment;
            pub use ::ink::env::emit_event as emit;
            pub use ::ink::trait_definition as interface;
            pub use ::ink::scale::Codec;
            pub use ::ink::scale::Encode;
            pub use ::ink::scale::Decode;
            pub use ::ink::scale_info::TypeInfo;
        }       
    };
}