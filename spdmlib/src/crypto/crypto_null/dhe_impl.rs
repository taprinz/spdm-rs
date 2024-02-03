// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: Apache-2.0 or MIT

extern crate alloc;
use alloc::boxed::Box;

use crate::crypto::{SpdmDhe, SpdmDheKeyExchange};
use crate::protocol::{SpdmDheAlgo, SpdmDheExchangeStruct, SpdmDheFinalKeyStruct};
use bytes::{BufMut, BytesMut};

pub static DEFAULT: SpdmDhe = SpdmDhe {
    generate_key_pair_cb: generate_key_pair,
};

fn generate_key_pair(
    dhe_algo: SpdmDheAlgo,
) -> Option<(SpdmDheExchangeStruct, Box<dyn SpdmDheKeyExchange + Send>)> {
    unimplemented!()
}

impl SpdmDheKeyExchange for SpdmDheKeyExchangeP256 {
    fn compute_final_key(
        self: Box<Self>,
        peer_pub_key: &SpdmDheExchangeStruct,
    ) -> Option<SpdmDheFinalKeyStruct> {
        unimplemented!()
    }
}

struct SpdmDheKeyExchangeP256();

impl SpdmDheKeyExchangeP256 {
    fn generate_key_pair() -> Option<(SpdmDheExchangeStruct, Box<dyn SpdmDheKeyExchange + Send>)> {
        unimplemented!()
    }
}

struct SpdmDheKeyExchangeP384();

impl SpdmDheKeyExchange for SpdmDheKeyExchangeP384 {
    fn compute_final_key(
        self: Box<Self>,
        peer_pub_key: &SpdmDheExchangeStruct,
    ) -> Option<SpdmDheFinalKeyStruct> {
        unimplemented!()
    }
}

impl SpdmDheKeyExchangeP384 {
    fn generate_key_pair() -> Option<(SpdmDheExchangeStruct, Box<dyn SpdmDheKeyExchange + Send>)> {
        unimplemented!()
    }
}
