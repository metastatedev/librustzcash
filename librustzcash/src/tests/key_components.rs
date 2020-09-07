use ff::{PrimeField, PrimeFieldRepr};
use pairing::bls12_381::Bls12;
use zcash_primitives::{
    jubjub::{fs::FsRepr, FixedGenerators, JubjubEngine, JubjubParams},
    primitives::{AssetType, Diversifier, ProofGenerationKey},
    ASSET_TYPE_DEFAULT,
};

use super::JUBJUB;

use crate::{
    librustzcash_ask_to_ak, librustzcash_check_diversifier, librustzcash_crh_ivk,
    librustzcash_ivk_to_pkd, librustzcash_nsk_to_nk,
};

#[test]
fn key_components() {
    #![allow(dead_code)]
    struct TestVector {
        sk: [u8; 32],
        ask: [u8; 32],
        nsk: [u8; 32],
        ovk: [u8; 32],
        ak: [u8; 32],
        nk: [u8; 32],
        ivk: [u8; 32],
        default_d: [u8; 11],
        default_pk_d: [u8; 32],
        note_v: u64,
        note_r: [u8; 32],
        note_cm: [u8; 32],
        note_pos: u64,
        note_nf: [u8; 32],
    };

    // From https://github.com/zcash-hackworks/zcash-test-vectors/blob/master/sapling_key_components.py
    let test_vectors = vec![
        TestVector {
            sk: [
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            ask: [
                0x85, 0x48, 0xa1, 0x4a, 0x47, 0x3e, 0xa5, 0x47, 0xaa, 0x23, 0x78, 0x40, 0x20, 0x44,
                0xf8, 0x18, 0xcf, 0x19, 0x11, 0xcf, 0x5d, 0xd2, 0x05, 0x4f, 0x67, 0x83, 0x45, 0xf0,
                0x0d, 0x0e, 0x88, 0x06,
            ],
            nsk: [
                0x30, 0x11, 0x4e, 0xa0, 0xdd, 0x0b, 0xb6, 0x1c, 0xf0, 0xea, 0xea, 0xb6, 0xec, 0x33,
                0x31, 0xf5, 0x81, 0xb0, 0x42, 0x5e, 0x27, 0x33, 0x85, 0x01, 0x26, 0x2d, 0x7e, 0xac,
                0x74, 0x5e, 0x6e, 0x05,
            ],
            ovk: [
                0x98, 0xd1, 0x69, 0x13, 0xd9, 0x9b, 0x04, 0x17, 0x7c, 0xab, 0xa4, 0x4f, 0x6e, 0x4d,
                0x22, 0x4e, 0x03, 0xb5, 0xac, 0x03, 0x1d, 0x7c, 0xe4, 0x5e, 0x86, 0x51, 0x38, 0xe1,
                0xb9, 0x96, 0xd6, 0x3b,
            ],
            ak: [
                0xf3, 0x44, 0xec, 0x38, 0x0f, 0xe1, 0x27, 0x3e, 0x30, 0x98, 0xc2, 0x58, 0x8c, 0x5d,
                0x3a, 0x79, 0x1f, 0xd7, 0xba, 0x95, 0x80, 0x32, 0x76, 0x07, 0x77, 0xfd, 0x0e, 0xfa,
                0x8e, 0xf1, 0x16, 0x20,
            ],
            nk: [
                0xf7, 0xcf, 0x9e, 0x77, 0xf2, 0xe5, 0x86, 0x83, 0x38, 0x3c, 0x15, 0x19, 0xac, 0x7b,
                0x06, 0x2d, 0x30, 0x04, 0x0e, 0x27, 0xa7, 0x25, 0xfb, 0x88, 0xfb, 0x19, 0xa9, 0x78,
                0xbd, 0x3f, 0xd6, 0xba,
            ],
            ivk: [
                0xb7, 0x0b, 0x7c, 0xd0, 0xed, 0x03, 0xcb, 0xdf, 0xd7, 0xad, 0xa9, 0x50, 0x2e, 0xe2,
                0x45, 0xb1, 0x3e, 0x56, 0x9d, 0x54, 0xa5, 0x71, 0x9d, 0x2d, 0xaa, 0x0f, 0x5f, 0x14,
                0x51, 0x47, 0x92, 0x04,
            ],
            default_d: [
                0xf1, 0x9d, 0x9b, 0x79, 0x7e, 0x39, 0xf3, 0x37, 0x44, 0x58, 0x39,
            ],
            default_pk_d: [
                0xdb, 0x4c, 0xd2, 0xb0, 0xaa, 0xc4, 0xf7, 0xeb, 0x8c, 0xa1, 0x31, 0xf1, 0x65, 0x67,
                0xc4, 0x45, 0xa9, 0x55, 0x51, 0x26, 0xd3, 0xc2, 0x9f, 0x14, 0xe3, 0xd7, 0x76, 0xe8,
                0x41, 0xae, 0x74, 0x15,
            ],
            note_v: 0,
            note_r: [
                0x39, 0x17, 0x6d, 0xac, 0x39, 0xac, 0xe4, 0x98, 0x0e, 0xcc, 0x8d, 0x77, 0x8e, 0x89,
                0x86, 0x02, 0x55, 0xec, 0x36, 0x15, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
            note_cm: [
                0xcb, 0x3c, 0xf9, 0x15, 0x32, 0x70, 0xd5, 0x7e, 0xb9, 0x14, 0xc6, 0xc2, 0xbc, 0xc0,
                0x18, 0x50, 0xc9, 0xfe, 0xd4, 0x4f, 0xce, 0x08, 0x06, 0x27, 0x8f, 0x08, 0x3e, 0xf2,
                0xdd, 0x07, 0x64, 0x39,
            ],
            note_pos: 0,
            note_nf: [
                0x44, 0xfa, 0xd6, 0x56, 0x4f, 0xfd, 0xec, 0x9f, 0xa1, 0x9c, 0x43, 0xa2, 0x8f, 0x86,
                0x1d, 0x5e, 0xbf, 0x60, 0x23, 0x46, 0x00, 0x7d, 0xe7, 0x62, 0x67, 0xd9, 0x75, 0x27,
                0x47, 0xab, 0x40, 0x63,
            ],
        },
        TestVector {
            sk: [
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01,
                0x01, 0x01, 0x01, 0x01,
            ],
            ask: [
                0xc9, 0x43, 0x56, 0x29, 0xbf, 0x8b, 0xff, 0xe5, 0x5e, 0x73, 0x35, 0xec, 0x07, 0x77,
                0x18, 0xba, 0x60, 0xba, 0x28, 0xd7, 0xac, 0x37, 0x94, 0xb7, 0x4f, 0x51, 0x2c, 0x31,
                0xaf, 0x0a, 0x53, 0x04,
            ],
            nsk: [
                0x11, 0xac, 0xc2, 0xea, 0xd0, 0x7b, 0x5f, 0x00, 0x8c, 0x1f, 0x0f, 0x09, 0x0c, 0xc8,
                0xdd, 0xf3, 0x35, 0x23, 0x6f, 0xf4, 0xb2, 0x53, 0xc6, 0x49, 0x56, 0x95, 0xe9, 0xd6,
                0x39, 0xda, 0xcd, 0x08,
            ],
            ovk: [
                0x3b, 0x94, 0x62, 0x10, 0xce, 0x6d, 0x1b, 0x16, 0x92, 0xd7, 0x39, 0x2a, 0xc8, 0x4a,
                0x8b, 0xc8, 0xf0, 0x3b, 0x72, 0x72, 0x3c, 0x7d, 0x36, 0x72, 0x1b, 0x80, 0x9a, 0x79,
                0xc9, 0xd6, 0xe4, 0x5b,
            ],
            ak: [
                0x82, 0xff, 0x5e, 0xff, 0xc5, 0x27, 0xae, 0x84, 0x02, 0x0b, 0xf2, 0xd3, 0x52, 0x01,
                0xc1, 0x02, 0x19, 0x13, 0x19, 0x47, 0xff, 0x4b, 0x96, 0xf8, 0x81, 0xa4, 0x5f, 0x2e,
                0x8a, 0xe3, 0x05, 0x18,
            ],
            nk: [
                0xc4, 0x53, 0x4d, 0x84, 0x8b, 0xb9, 0x18, 0xcf, 0x4a, 0x7f, 0x8b, 0x98, 0x74, 0x0a,
                0xb3, 0xcc, 0xee, 0x58, 0x67, 0x95, 0xff, 0x4d, 0xf6, 0x45, 0x47, 0xa8, 0x88, 0x8a,
                0x6c, 0x74, 0x15, 0xd2,
            ],
            ivk: [
                0xc5, 0x18, 0x38, 0x44, 0x66, 0xb2, 0x69, 0x88, 0xb5, 0x10, 0x90, 0x67, 0x41, 0x8d,
                0x19, 0x2d, 0x9d, 0x6b, 0xd0, 0xd9, 0x23, 0x22, 0x05, 0xd7, 0x74, 0x18, 0xc2, 0x40,
                0xfc, 0x68, 0xa4, 0x06,
            ],
            default_d: [
                0xae, 0xf1, 0x80, 0xf6, 0xe3, 0x4e, 0x35, 0x4b, 0x88, 0x8f, 0x81,
            ],
            default_pk_d: [
                0xa6, 0xb1, 0x3e, 0xa3, 0x36, 0xdd, 0xb7, 0xa6, 0x7b, 0xb0, 0x9a, 0x0e, 0x68, 0xe9,
                0xd3, 0xcf, 0xb3, 0x92, 0x10, 0x83, 0x1e, 0xa3, 0xa2, 0x96, 0xba, 0x09, 0xa9, 0x22,
                0x06, 0x0f, 0xd3, 0x8b,
            ],
            note_v: 12227227834928555328,
            note_r: [
                0x47, 0x8b, 0xa0, 0xee, 0x6e, 0x1a, 0x75, 0xb6, 0x00, 0x03, 0x6f, 0x26, 0xf1, 0x8b,
                0x70, 0x15, 0xab, 0x55, 0x6b, 0xed, 0xdf, 0x8b, 0x96, 0x02, 0x38, 0x86, 0x9f, 0x89,
                0xdd, 0x80, 0x4e, 0x06,
            ],
            note_cm: [
                0xb5, 0x78, 0x93, 0x50, 0x0b, 0xfb, 0x85, 0xdf, 0x2e, 0x8b, 0x01, 0xac, 0x45, 0x2f,
                0x89, 0xe1, 0x0e, 0x26, 0x6b, 0xcf, 0xa3, 0x1c, 0x31, 0xb2, 0x9a, 0x53, 0xae, 0x72,
                0xca, 0xd4, 0x69, 0x50,
            ],
            note_pos: 763714296,
            note_nf: [
                0x67, 0x9e, 0xb0, 0xc3, 0xa7, 0x57, 0xe2, 0xae, 0x83, 0xcd, 0xb4, 0x2a, 0x1a, 0xb2,
                0x59, 0xd7, 0x83, 0x88, 0x31, 0x54, 0x19, 0xad, 0xc7, 0x1d, 0x2e, 0x37, 0x63, 0x17,
                0x4c, 0x2e, 0x9d, 0x93,
            ],
        },
        TestVector {
            sk: [
                0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02, 0x02,
                0x02, 0x02, 0x02, 0x02,
            ],
            ask: [
                0xee, 0x1c, 0x3d, 0x7e, 0xfe, 0x0a, 0x78, 0x06, 0x3d, 0x6a, 0xf3, 0xd9, 0xd8, 0x12,
                0x12, 0xaf, 0x47, 0xb7, 0xc1, 0xb7, 0x61, 0xf8, 0x5c, 0xcb, 0x06, 0x6f, 0xc1, 0x1a,
                0x6a, 0x42, 0x17, 0x03,
            ],
            nsk: [
                0x1d, 0x3b, 0x71, 0x37, 0x55, 0xd7, 0x48, 0x75, 0xe8, 0xea, 0x38, 0xfd, 0x16, 0x6e,
                0x76, 0xc6, 0x2a, 0x42, 0x50, 0x21, 0x6e, 0x6b, 0xbf, 0xe4, 0x8a, 0x5e, 0x2e, 0xab,
                0xad, 0x11, 0x7f, 0x0b,
            ],
            ovk: [
                0x8b, 0xf4, 0x39, 0x0e, 0x28, 0xdd, 0xc9, 0x5b, 0x83, 0x02, 0xc3, 0x81, 0xd5, 0x81,
                0x0b, 0x84, 0xba, 0x8e, 0x60, 0x96, 0xe5, 0xa7, 0x68, 0x22, 0x77, 0x4f, 0xd4, 0x9f,
                0x49, 0x1e, 0x8f, 0x49,
            ],
            ak: [
                0xab, 0x83, 0x57, 0x4e, 0xb5, 0xde, 0x85, 0x9a, 0x0a, 0xb8, 0x62, 0x9d, 0xec, 0x34,
                0xc7, 0xbe, 0xe8, 0xc3, 0xfc, 0x74, 0xdf, 0xa0, 0xb1, 0x9a, 0x3a, 0x74, 0x68, 0xd1,
                0x5d, 0xca, 0x64, 0xc6,
            ],
            nk: [
                0x95, 0xd5, 0x80, 0x53, 0xe0, 0x59, 0x2e, 0x4a, 0x16, 0x9c, 0xc0, 0xb7, 0x92, 0x8a,
                0xaa, 0xc3, 0xde, 0x24, 0xef, 0x15, 0x31, 0xaa, 0x9e, 0xb6, 0xf4, 0xab, 0x93, 0x91,
                0x4d, 0xa8, 0xa0, 0x6e,
            ],
            ivk: [
                0x47, 0x1c, 0x24, 0xa3, 0xdc, 0x87, 0x30, 0xe7, 0x50, 0x36, 0xc0, 0xa9, 0x5f, 0x3e,
                0x2f, 0x7d, 0xd1, 0xbe, 0x6f, 0xb9, 0x3a, 0xd2, 0x95, 0x92, 0x20, 0x3d, 0xef, 0x30,
                0x41, 0x95, 0x45, 0x05,
            ],
            default_d: [
                0x75, 0x99, 0xf0, 0xbf, 0x9b, 0x57, 0xcd, 0x2d, 0xc2, 0x99, 0xb6,
            ],
            default_pk_d: [
                0x66, 0x14, 0x17, 0x39, 0x51, 0x4b, 0x28, 0xf0, 0x5d, 0xef, 0x8a, 0x18, 0xee, 0xee,
                0x5e, 0xed, 0x4d, 0x44, 0xc6, 0x22, 0x5c, 0x3c, 0x65, 0xd8, 0x8d, 0xd9, 0x90, 0x77,
                0x08, 0x01, 0x2f, 0x5a,
            ],
            note_v: 6007711596147559040,
            note_r: [
                0x14, 0x7c, 0xf2, 0xb5, 0x1b, 0x4c, 0x7c, 0x63, 0xcb, 0x77, 0xb9, 0x9e, 0x8b, 0x78,
                0x3e, 0x5b, 0x51, 0x11, 0xdb, 0x0a, 0x7c, 0xa0, 0x4d, 0x6c, 0x01, 0x4a, 0x1d, 0x7d,
                0xa8, 0x3b, 0xae, 0x0a,
            ],
            note_cm: [
                0xdb, 0x85, 0xa7, 0x0a, 0x98, 0x43, 0x7f, 0x73, 0x16, 0x7f, 0xc3, 0x32, 0xd5, 0xb7,
                0xb7, 0x40, 0x82, 0x96, 0x66, 0x17, 0x70, 0xb1, 0x01, 0xb0, 0xaa, 0x87, 0x83, 0x9f,
                0x4e, 0x55, 0xf1, 0x51,
            ],
            note_pos: 1527428592,
            note_nf: [
                0xe9, 0x8f, 0x6a, 0x8f, 0x34, 0xff, 0x49, 0x80, 0x59, 0xb3, 0xc7, 0x31, 0xb9, 0x1f,
                0x45, 0x11, 0x08, 0xc4, 0x95, 0x4d, 0x91, 0x94, 0x84, 0x36, 0x1c, 0xf9, 0xb4, 0x8f,
                0x59, 0xae, 0x1d, 0x14,
            ],
        },
        TestVector {
            sk: [
                0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
                0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03, 0x03,
                0x03, 0x03, 0x03, 0x03,
            ],
            ask: [
                0x00, 0xc3, 0xa1, 0xe1, 0xca, 0x8f, 0x4e, 0x04, 0x80, 0xee, 0x1e, 0xe9, 0x0c, 0xa7,
                0x51, 0x78, 0x79, 0xd3, 0xfc, 0x5c, 0x81, 0x5c, 0x09, 0x03, 0xe5, 0xee, 0xbc, 0x94,
                0xbb, 0x80, 0x95, 0x03,
            ],
            nsk: [
                0xe6, 0x62, 0x85, 0xa5, 0xe9, 0xb6, 0x5e, 0x15, 0x7a, 0xd2, 0xfc, 0xd5, 0x43, 0xda,
                0xd9, 0x8c, 0x67, 0xa5, 0x8a, 0xbd, 0xf2, 0x87, 0xe0, 0x55, 0x06, 0xbd, 0x1c, 0x2e,
                0x59, 0xb0, 0x72, 0x0b,
            ],
            ovk: [
                0x14, 0x76, 0x78, 0xe0, 0x55, 0x3b, 0x97, 0x82, 0x93, 0x47, 0x64, 0x7c, 0x5b, 0xc7,
                0xda, 0xb4, 0xcc, 0x22, 0x02, 0xb5, 0x4e, 0xc2, 0x9f, 0xd3, 0x1a, 0x3d, 0xe6, 0xbe,
                0x08, 0x25, 0xfc, 0x5e,
            ],
            ak: [
                0x3c, 0x9c, 0xde, 0x7e, 0x5d, 0x0d, 0x38, 0xa8, 0x61, 0x0f, 0xaa, 0xdb, 0xcf, 0x4c,
                0x34, 0x3f, 0x5d, 0x3c, 0xfa, 0x31, 0x55, 0xa5, 0xb9, 0x46, 0x61, 0xa6, 0x75, 0x3e,
                0x96, 0xe8, 0x84, 0xea,
            ],
            nk: [
                0xb7, 0x7d, 0x36, 0xf5, 0x08, 0x94, 0x1d, 0xbd, 0x61, 0xcf, 0xd0, 0xf1, 0x59, 0xee,
                0x05, 0xcf, 0xaa, 0x78, 0xa2, 0x6c, 0x94, 0x92, 0x90, 0x38, 0x06, 0xd8, 0x3b, 0x59,
                0x8d, 0x3c, 0x1c, 0x2a,
            ],
            ivk: [
                0x63, 0x6a, 0xa9, 0x64, 0xbf, 0xc2, 0x3c, 0xe4, 0xb1, 0xfc, 0xf7, 0xdf, 0xc9, 0x91,
                0x79, 0xdd, 0xc4, 0x06, 0xff, 0x55, 0x40, 0x0c, 0x92, 0x95, 0xac, 0xfc, 0x14, 0xf0,
                0x31, 0xc7, 0x26, 0x00,
            ],
            default_d: [
                0x1b, 0x81, 0x61, 0x4f, 0x1d, 0xad, 0xea, 0x0f, 0x8d, 0x0a, 0x58,
            ],
            default_pk_d: [
                0x25, 0xeb, 0x55, 0xfc, 0xcf, 0x76, 0x1f, 0xc6, 0x4e, 0x85, 0xa5, 0x88, 0xef, 0xe6,
                0xea, 0xd7, 0x83, 0x2f, 0xb1, 0xf0, 0xf7, 0xa8, 0x31, 0x65, 0x89, 0x5b, 0xdf, 0xf9,
                0x42, 0x92, 0x5f, 0x5c,
            ],
            note_v: 18234939431076114368,
            note_r: [
                0x34, 0xa4, 0xb2, 0xa9, 0x14, 0x4f, 0xf5, 0xea, 0x54, 0xef, 0xee, 0x87, 0xcf, 0x90,
                0x1b, 0x5b, 0xed, 0x5e, 0x35, 0xd2, 0x1f, 0xbb, 0xd7, 0x88, 0xd5, 0xbd, 0x9d, 0x83,
                0x3e, 0x11, 0x28, 0x04,
            ],
            note_cm: [
                0xe0, 0x8c, 0xe4, 0x82, 0xb3, 0xa8, 0xfb, 0x3b, 0x35, 0xcc, 0xdb, 0xe3, 0x43, 0x37,
                0xbd, 0x10, 0x5d, 0x88, 0x39, 0x21, 0x2e, 0x0d, 0x16, 0x44, 0xb9, 0xd5, 0x5c, 0xaa,
                0x60, 0xd1, 0x9b, 0x6c,
            ],
            note_pos: 2291142888,
            note_nf: [
                0x55, 0x47, 0xaa, 0x12, 0xff, 0x80, 0xa6, 0xb3, 0x30, 0x4e, 0x3b, 0x05, 0x86, 0x56,
                0x47, 0x2a, 0xbd, 0x2c, 0x81, 0x83, 0xb5, 0x9d, 0x07, 0x37, 0xb9, 0x3c, 0xee, 0x75,
                0x8b, 0xec, 0x47, 0xa1,
            ],
        },
        TestVector {
            sk: [
                0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04, 0x04,
                0x04, 0x04, 0x04, 0x04,
            ],
            ask: [
                0x82, 0x36, 0xd1, 0x9d, 0x32, 0x05, 0xd8, 0x55, 0x43, 0xa0, 0x68, 0x11, 0x34, 0x3f,
                0x82, 0x7b, 0x65, 0x63, 0x77, 0x0a, 0x49, 0xaa, 0x4d, 0x0c, 0xa0, 0x08, 0x18, 0x05,
                0xd4, 0xc8, 0xea, 0x0d,
            ],
            nsk: [
                0x7e, 0xc1, 0xef, 0x0b, 0xed, 0x82, 0x71, 0x82, 0x72, 0xf0, 0xf4, 0x4f, 0x01, 0x7c,
                0x48, 0x41, 0x74, 0x51, 0x3d, 0x66, 0x1d, 0xd1, 0x68, 0xaf, 0x02, 0xd2, 0x09, 0x2a,
                0x1d, 0x8a, 0x05, 0x07,
            ],
            ovk: [
                0x1b, 0x6e, 0x75, 0xec, 0xe3, 0xac, 0xe8, 0xdb, 0xa6, 0xa5, 0x41, 0x0d, 0x9a, 0xd4,
                0x75, 0x56, 0x68, 0xe4, 0xb3, 0x95, 0x85, 0xd6, 0x35, 0xec, 0x1d, 0xa7, 0xc8, 0xdc,
                0xfd, 0x5f, 0xc4, 0xed,
            ],
            ak: [
                0x55, 0xe8, 0x83, 0x89, 0xbb, 0x7e, 0x41, 0xde, 0x13, 0x0c, 0xfa, 0x51, 0xa8, 0x71,
                0x5f, 0xde, 0x01, 0xff, 0x9c, 0x68, 0x76, 0x64, 0x7f, 0x01, 0x75, 0xad, 0x34, 0xf0,
                0x58, 0xdd, 0xe0, 0x1a,
            ],
            nk: [
                0x72, 0x5d, 0x4a, 0xd6, 0xa1, 0x50, 0x21, 0xcd, 0x1c, 0x48, 0xc5, 0xee, 0x19, 0xde,
                0x6c, 0x1e, 0x76, 0x8a, 0x2c, 0xc0, 0xa9, 0xa7, 0x30, 0xa0, 0x1b, 0xb2, 0x1c, 0x95,
                0xe3, 0xd9, 0xe4, 0x3c,
            ],
            ivk: [
                0x67, 0xfa, 0x2b, 0xf7, 0xc6, 0x7d, 0x46, 0x58, 0x24, 0x3c, 0x31, 0x7c, 0x0c, 0xb4,
                0x1f, 0xd3, 0x20, 0x64, 0xdf, 0xd3, 0x70, 0x9f, 0xe0, 0xdc, 0xb7, 0x24, 0xf1, 0x4b,
                0xb0, 0x1a, 0x1d, 0x04,
            ],
            default_d: [
                0xfc, 0xfb, 0x68, 0xa4, 0x0d, 0x4b, 0xc6, 0xa0, 0x4b, 0x09, 0xc4,
            ],
            default_pk_d: [
                0x8b, 0x2a, 0x33, 0x7f, 0x03, 0x62, 0x2c, 0x24, 0xff, 0x38, 0x1d, 0x4c, 0x54, 0x6f,
                0x69, 0x77, 0xf9, 0x05, 0x22, 0xe9, 0x2f, 0xde, 0x44, 0xc9, 0xd1, 0xbb, 0x09, 0x97,
                0x14, 0xb9, 0xdb, 0x2b,
            ],
            note_v: 12015423192295118080,
            note_r: [
                0xe5, 0x57, 0x85, 0x13, 0x55, 0x74, 0x7c, 0x09, 0xac, 0x59, 0x01, 0x3c, 0xbd, 0xe8,
                0x59, 0x80, 0x96, 0x4e, 0xc1, 0x84, 0x4d, 0x9c, 0x69, 0x67, 0xca, 0x0c, 0x02, 0x9c,
                0x84, 0x57, 0xbb, 0x04,
            ],
            note_cm: [
                0xbd, 0xc8, 0x54, 0xbf, 0x3e, 0x7b, 0x00, 0x82, 0x1f, 0x3b, 0x8b, 0x85, 0x23, 0x8c,
                0xcf, 0x1e, 0x67, 0x15, 0xbf, 0xe7, 0x0b, 0x63, 0x2d, 0x04, 0x4b, 0x26, 0xfb, 0x2b,
                0xc7, 0x1b, 0x7f, 0x36,
            ],
            note_pos: 3054857184,
            note_nf: [
                0x8a, 0x9a, 0xbd, 0xa3, 0xd4, 0xef, 0x85, 0xca, 0xf2, 0x2b, 0xfa, 0xf2, 0xc4, 0x8f,
                0x62, 0x38, 0x2a, 0x73, 0xa1, 0x62, 0x4e, 0xb8, 0xeb, 0x2b, 0xd0, 0x0d, 0x27, 0x03,
                0x01, 0xbf, 0x3d, 0x13,
            ],
        },
        TestVector {
            sk: [
                0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05, 0x05,
                0x05, 0x05, 0x05, 0x05,
            ],
            ask: [
                0xea, 0xe6, 0x88, 0x4d, 0x76, 0x4a, 0x05, 0x40, 0x61, 0xa8, 0xf1, 0xc0, 0x07, 0x6c,
                0x62, 0x4d, 0xcb, 0x73, 0x87, 0x89, 0xf7, 0xad, 0x1e, 0x74, 0x08, 0xe3, 0x1f, 0x24,
                0xdf, 0xc8, 0x26, 0x07,
            ],
            nsk: [
                0xfb, 0xe6, 0x10, 0xf4, 0x2a, 0x41, 0x74, 0x9f, 0x9b, 0x6e, 0x6e, 0x4a, 0x54, 0xb5,
                0xa3, 0x2e, 0xbf, 0xe8, 0xf4, 0x38, 0x00, 0x88, 0x1b, 0xa6, 0xcd, 0x13, 0xed, 0x0b,
                0x05, 0x29, 0x46, 0x01,
            ],
            ovk: [
                0xc6, 0xbc, 0x1f, 0x39, 0xf0, 0xd7, 0x86, 0x31, 0x4c, 0xb2, 0x0b, 0xf9, 0xab, 0x22,
                0x85, 0x40, 0x91, 0x35, 0x55, 0xf9, 0x70, 0x69, 0x6b, 0x6d, 0x7c, 0x77, 0xbb, 0x33,
                0x23, 0x28, 0x37, 0x2a,
            ],
            ak: [
                0xe6, 0x82, 0x76, 0x59, 0x14, 0xe3, 0x86, 0x4c, 0x33, 0x9e, 0x57, 0x82, 0xb8, 0x55,
                0xc0, 0xfd, 0xf4, 0x0e, 0x0d, 0xfc, 0xed, 0xb9, 0xe7, 0xb4, 0x7b, 0xc9, 0x4b, 0x90,
                0xb3, 0xa4, 0xc9, 0x88,
            ],
            nk: [
                0x82, 0x25, 0x6b, 0x95, 0x62, 0x3c, 0x67, 0x02, 0x4b, 0x44, 0x24, 0xd9, 0x14, 0x00,
                0xa3, 0x70, 0xe7, 0xac, 0x8e, 0x4d, 0x15, 0x48, 0x2a, 0x37, 0x59, 0xe0, 0x0d, 0x21,
                0x97, 0x49, 0xda, 0xee,
            ],
            ivk: [
                0xea, 0x3f, 0x1d, 0x80, 0xe4, 0x30, 0x7c, 0xa7, 0x3b, 0x9f, 0x37, 0x80, 0x1f, 0x91,
                0xfb, 0xa8, 0x10, 0xcc, 0x41, 0xd2, 0x79, 0xfc, 0x29, 0xf5, 0x64, 0x23, 0x56, 0x54,
                0xa2, 0x17, 0x8e, 0x03,
            ],
            default_d: [
                0xeb, 0x51, 0x98, 0x82, 0xad, 0x1e, 0x5c, 0xc6, 0x54, 0xcd, 0x59,
            ],
            default_pk_d: [
                0x6b, 0x27, 0xda, 0xcc, 0xb5, 0xa8, 0x20, 0x7f, 0x53, 0x2d, 0x10, 0xca, 0x23, 0x8f,
                0x97, 0x86, 0x64, 0x8a, 0x11, 0xb5, 0x96, 0x6e, 0x51, 0xa2, 0xf7, 0xd8, 0x9e, 0x15,
                0xd2, 0x9b, 0x8f, 0xdf,
            ],
            note_v: 5795906953514121792,
            note_r: [
                0x68, 0xf0, 0x61, 0x04, 0x60, 0x6b, 0x0c, 0x54, 0x49, 0x84, 0x5f, 0xf4, 0xc6, 0x5f,
                0x73, 0xe9, 0x0f, 0x45, 0xef, 0x5a, 0x43, 0xc9, 0xd7, 0x4c, 0xb2, 0xc8, 0x5c, 0xf5,
                0x6c, 0x94, 0xc0, 0x02,
            ],
            note_cm: [
                0xe8, 0x26, 0x7d, 0x30, 0xac, 0x11, 0xc1, 0x00, 0xbc, 0x7a, 0x0f, 0xdf, 0x91, 0xf7,
                0x1d, 0x74, 0xc5, 0xbc, 0xf2, 0xe1, 0xef, 0x95, 0x66, 0x90, 0x44, 0x73, 0x01, 0x69,
                0xde, 0x1a, 0x5b, 0x4c,
            ],
            note_pos: 3818571480,
            note_nf: [
                0x33, 0x2a, 0xd9, 0x9e, 0xb9, 0xe9, 0x77, 0xeb, 0x62, 0x7a, 0x12, 0x2d, 0xbf, 0xb2,
                0xf2, 0x5f, 0xe5, 0x88, 0xe5, 0x97, 0x75, 0x3e, 0xc5, 0x58, 0x0f, 0xf2, 0xbe, 0x20,
                0xb6, 0xc9, 0xa7, 0xe1,
            ],
        },
        TestVector {
            sk: [
                0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
                0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
                0x06, 0x06, 0x06, 0x06,
            ],
            ask: [
                0xe8, 0xf8, 0x16, 0xb4, 0xbc, 0x08, 0xa7, 0xe5, 0x66, 0x75, 0x0c, 0xc2, 0x8a, 0xfe,
                0x82, 0xa4, 0xce, 0xa9, 0xc2, 0xbe, 0xf2, 0x44, 0xfa, 0x4b, 0x13, 0xc4, 0x73, 0x9b,
                0x28, 0x07, 0x4c, 0x0d,
            ],
            nsk: [
                0x32, 0x61, 0x5b, 0x13, 0x7f, 0x28, 0x01, 0xed, 0x44, 0x6e, 0x48, 0x78, 0x1a, 0xb0,
                0x63, 0x45, 0x72, 0xe1, 0x8c, 0xfb, 0x06, 0x93, 0x72, 0x1b, 0x88, 0x03, 0xc0, 0x5b,
                0x82, 0x27, 0xd1, 0x07,
            ],
            ovk: [
                0xf6, 0x2c, 0x05, 0xe8, 0x48, 0xa8, 0x73, 0xef, 0x88, 0x5e, 0x12, 0xb0, 0x8c, 0x5e,
                0x7c, 0xa2, 0xf3, 0x24, 0x24, 0xba, 0xcc, 0x75, 0x4c, 0xb6, 0x97, 0x50, 0x44, 0x4d,
                0x35, 0x5f, 0x51, 0x06,
            ],
            ak: [
                0xff, 0x27, 0xdb, 0x07, 0x51, 0x94, 0x5d, 0x3e, 0xe4, 0xbe, 0x9c, 0xf1, 0x5c, 0x2e,
                0xa2, 0x11, 0xb2, 0x4b, 0x16, 0x4d, 0x5f, 0x2d, 0x7d, 0xdf, 0xf5, 0xe4, 0xa0, 0x70,
                0x8f, 0x10, 0xb9, 0x5e,
            ],
            nk: [
                0x94, 0x38, 0x85, 0x95, 0x9d, 0x4e, 0xf8, 0xa9, 0xcf, 0xca, 0x07, 0xc4, 0x57, 0xf0,
                0x9e, 0xc7, 0x4b, 0x96, 0xf9, 0x93, 0xd8, 0xe0, 0xfa, 0x32, 0xb1, 0x9c, 0x03, 0xe3,
                0xb0, 0x7a, 0x42, 0x0f,
            ],
            ivk: [
                0xb5, 0xc5, 0x89, 0x49, 0x43, 0x95, 0x69, 0x33, 0xc0, 0xe5, 0xc1, 0x2d, 0x31, 0x1f,
                0xc1, 0x2c, 0xba, 0x58, 0x35, 0x4b, 0x5c, 0x38, 0x9e, 0xdc, 0x03, 0xda, 0x55, 0x08,
                0x4f, 0x74, 0xc2, 0x05,
            ],
            default_d: [
                0xbe, 0xbb, 0x0f, 0xb4, 0x6b, 0x8a, 0xaf, 0xf8, 0x90, 0x40, 0xf6,
            ],
            default_pk_d: [
                0xd1, 0x1d, 0xa0, 0x1f, 0x0b, 0x43, 0xbd, 0xd5, 0x28, 0x8d, 0x32, 0x38, 0x5b, 0x87,
                0x71, 0xd2, 0x23, 0x49, 0x3c, 0x69, 0x80, 0x25, 0x44, 0x04, 0x3f, 0x77, 0xcf, 0x1d,
                0x71, 0xc1, 0xcb, 0x8c,
            ],
            note_v: 18023134788442677120,
            note_r: [
                0x49, 0xf9, 0x0b, 0x47, 0xfd, 0x52, 0xfe, 0xe7, 0xc1, 0xc8, 0x1f, 0x0d, 0xcb, 0x5b,
                0x74, 0xc3, 0xfb, 0x9b, 0x3e, 0x03, 0x97, 0x6f, 0x8b, 0x75, 0x24, 0xea, 0xba, 0xd0,
                0x08, 0x89, 0x21, 0x07,
            ],
            note_cm: [
                0x57, 0x2b, 0xa2, 0x05, 0x25, 0xb0, 0xac, 0x4d, 0x6d, 0xc0, 0x1a, 0xc2, 0xea, 0x10,
                0x90, 0xb6, 0xe0, 0xf2, 0xf4, 0xbf, 0x4e, 0xc4, 0xa0, 0xdb, 0x5b, 0xbc, 0xcb, 0x5b,
                0x78, 0x3a, 0x1e, 0x55,
            ],
            note_pos: 287318480,
            note_nf: [
                0xfc, 0x74, 0xcd, 0x0e, 0x4b, 0xe0, 0x49, 0x57, 0xb1, 0x96, 0xcf, 0x87, 0x34, 0xae,
                0x99, 0x23, 0x96, 0xaf, 0x4c, 0xfa, 0x8f, 0xec, 0xbb, 0x86, 0xf9, 0x61, 0xe6, 0xb4,
                0x07, 0xd5, 0x1e, 0x11,
            ],
        },
        TestVector {
            sk: [
                0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
                0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07, 0x07,
                0x07, 0x07, 0x07, 0x07,
            ],
            ask: [
                0x74, 0xb4, 0x4a, 0x37, 0xf1, 0x50, 0x23, 0xc0, 0x60, 0x42, 0x7e, 0x1d, 0xae, 0xa3,
                0xf6, 0x43, 0x12, 0xdd, 0x8f, 0xeb, 0x7b, 0x2c, 0xed, 0xf0, 0xdd, 0x55, 0x44, 0x49,
                0x3f, 0x87, 0x2c, 0x06,
            ],
            nsk: [
                0x07, 0x5c, 0x35, 0xdb, 0x8b, 0x1b, 0x25, 0x75, 0x42, 0x23, 0xec, 0xee, 0x34, 0xab,
                0x73, 0x0d, 0xdd, 0xd1, 0xf1, 0x4a, 0x6a, 0x54, 0xf4, 0xc6, 0xf4, 0x68, 0x45, 0x3c,
                0x3c, 0x6e, 0xd6, 0x0b,
            ],
            ovk: [
                0xe9, 0xe0, 0xdc, 0x1e, 0xd3, 0x11, 0xda, 0xed, 0x64, 0xbd, 0x74, 0xda, 0x5d, 0x94,
                0xfe, 0x88, 0xa6, 0xea, 0x41, 0x4b, 0x73, 0x12, 0xde, 0x3d, 0x2a, 0x78, 0xf6, 0x46,
                0x32, 0xbb, 0xe3, 0x73,
            ],
            ak: [
                0x28, 0x3f, 0x9a, 0xaf, 0xa9, 0xbc, 0xb3, 0xe6, 0xce, 0x17, 0xe6, 0x32, 0x12, 0x63,
                0x4c, 0xb3, 0xee, 0x55, 0x0c, 0x47, 0x6b, 0x67, 0x6b, 0xd3, 0x56, 0xa6, 0xdf, 0x8a,
                0xdf, 0x51, 0xd2, 0x5e,
            ],
            nk: [
                0xdc, 0x4c, 0x67, 0xb1, 0x0d, 0x4b, 0x0a, 0x21, 0x8d, 0xc6, 0xe1, 0x48, 0x70, 0x66,
                0x74, 0x0a, 0x40, 0x93, 0x17, 0x86, 0x6c, 0x32, 0xe6, 0x64, 0xb5, 0x0e, 0x39, 0x7a,
                0xa8, 0x03, 0x89, 0xd4,
            ],
            ivk: [
                0x87, 0x16, 0xc8, 0x28, 0x80, 0xe1, 0x36, 0x83, 0xe1, 0xbb, 0x05, 0x9d, 0xd0, 0x6c,
                0x80, 0xc9, 0x01, 0x34, 0xa9, 0x6d, 0x5a, 0xfc, 0xa8, 0xaa, 0xc2, 0xbb, 0xf6, 0x8b,
                0xb0, 0x5f, 0x84, 0x02,
            ],
            default_d: [
                0xad, 0x6e, 0x2e, 0x18, 0x5a, 0x31, 0x00, 0xe3, 0xa6, 0xa8, 0xb3,
            ],
            default_pk_d: [
                0x32, 0xcb, 0x28, 0x06, 0xb8, 0x82, 0xf1, 0x36, 0x8b, 0x0d, 0x4a, 0x89, 0x8f, 0x72,
                0xc4, 0xc8, 0xf7, 0x28, 0x13, 0x2c, 0xc1, 0x24, 0x56, 0x94, 0x6e, 0x7f, 0x4c, 0xb0,
                0xfb, 0x05, 0x8d, 0xa9,
            ],
            note_v: 11803618549661680832,
            note_r: [
                0x51, 0x65, 0xaf, 0xf2, 0x2d, 0xd4, 0xed, 0x56, 0xb4, 0xd8, 0x1d, 0x1f, 0x17, 0x1c,
                0xc3, 0xd6, 0x43, 0x2f, 0xed, 0x1b, 0xeb, 0xf2, 0x0a, 0x7b, 0xea, 0xb1, 0x2d, 0xb1,
                0x42, 0xf9, 0x4a, 0x0c,
            ],
            note_cm: [
                0xab, 0x7f, 0xc5, 0x66, 0x87, 0x3c, 0xcd, 0xe6, 0x71, 0xf5, 0x98, 0x27, 0x67, 0x85,
                0x60, 0xa0, 0x06, 0xf8, 0x2b, 0xb7, 0xad, 0xcd, 0x75, 0x22, 0x3f, 0xa8, 0x59, 0x36,
                0xf7, 0x8c, 0x2b, 0x23,
            ],
            note_pos: 1051032776,
            note_nf: [
                0xd2, 0xe8, 0x87, 0xbd, 0x85, 0x4a, 0x80, 0x2b, 0xce, 0x85, 0x70, 0x53, 0x02, 0x0f,
                0x5d, 0x3e, 0x7c, 0x8a, 0xe5, 0x26, 0x7c, 0x5b, 0x65, 0x83, 0xb3, 0xd2, 0x12, 0xcc,
                0x8b, 0xb6, 0x98, 0x90,
            ],
        },
        TestVector {
            sk: [
                0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
                0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08,
                0x08, 0x08, 0x08, 0x08,
            ],
            ask: [
                0x03, 0x9d, 0xd9, 0x3d, 0xf3, 0x11, 0xff, 0x8f, 0xba, 0xb3, 0xfe, 0x23, 0x02, 0x19,
                0xcd, 0x42, 0xac, 0x87, 0x94, 0x84, 0xf3, 0x0b, 0x90, 0x3a, 0x3c, 0x1e, 0x67, 0xcc,
                0xca, 0x5a, 0x7b, 0x0d,
            ],
            nsk: [
                0x04, 0x9f, 0xa1, 0x4f, 0x48, 0x6c, 0x75, 0xb9, 0xfa, 0xd7, 0xe3, 0xb6, 0x73, 0xa4,
                0x43, 0xdd, 0x07, 0x4e, 0xaa, 0x96, 0xed, 0xcb, 0x2a, 0x53, 0xea, 0xaa, 0xbd, 0xaf,
                0x70, 0xff, 0xbb, 0x08,
            ],
            ovk: [
                0x14, 0x7d, 0xd1, 0x1d, 0x77, 0xeb, 0xa1, 0xb1, 0x63, 0x6f, 0xd6, 0x19, 0x0c, 0x62,
                0xb9, 0xa5, 0xd0, 0x48, 0x1b, 0xee, 0x7e, 0x91, 0x7f, 0xab, 0x02, 0xe2, 0x18, 0x58,
                0x06, 0x3a, 0xb5, 0x04,
            ],
            ak: [
                0x36, 0x40, 0x48, 0xee, 0xdb, 0xe8, 0xca, 0x20, 0x5e, 0xb7, 0xe7, 0xba, 0x0a, 0x90,
                0x12, 0x16, 0x6c, 0x7c, 0x7b, 0xd9, 0xeb, 0x22, 0x8e, 0x08, 0x48, 0x14, 0x48, 0xc4,
                0x88, 0xaa, 0x21, 0xd2,
            ],
            nk: [
                0xed, 0x60, 0xaf, 0x1c, 0xe7, 0xdf, 0x38, 0x07, 0x0d, 0x38, 0x51, 0x43, 0x2a, 0x96,
                0x48, 0x0d, 0xb0, 0xb4, 0x17, 0xc3, 0x68, 0x2a, 0x1d, 0x68, 0xe3, 0xe8, 0x93, 0x34,
                0x23, 0x5c, 0x0b, 0xdf,
            ],
            ivk: [
                0x99, 0xc9, 0xb4, 0xb8, 0x4f, 0x4b, 0x4e, 0x35, 0x0f, 0x78, 0x7d, 0x1c, 0xf7, 0x05,
                0x1d, 0x50, 0xec, 0xc3, 0x4b, 0x1a, 0x5b, 0x20, 0xd2, 0xd2, 0x13, 0x9b, 0x4a, 0xf1,
                0xf1, 0x60, 0xe0, 0x01,
            ],
            default_d: [
                0x21, 0xc9, 0x0e, 0x1c, 0x65, 0x8b, 0x3e, 0xfe, 0x86, 0xaf, 0x58,
            ],
            default_pk_d: [
                0x9e, 0x64, 0x17, 0x4b, 0x4a, 0xb9, 0x81, 0x40, 0x5c, 0x32, 0x3b, 0x5e, 0x12, 0x47,
                0x59, 0x45, 0xa4, 0x6d, 0x4f, 0xed, 0xf8, 0x06, 0x08, 0x28, 0x04, 0x1c, 0xd2, 0x0e,
                0x62, 0xfd, 0x2c, 0xef,
            ],
            note_v: 5584102310880684544,
            note_r: [
                0x8c, 0x3e, 0x56, 0x44, 0x9d, 0xc8, 0x63, 0x54, 0xd3, 0x3b, 0x02, 0x5e, 0xf2, 0x79,
                0x34, 0x60, 0xbc, 0xb1, 0x69, 0xf3, 0x32, 0x4e, 0x4a, 0x6b, 0x64, 0xba, 0xa6, 0x08,
                0x32, 0x31, 0x57, 0x04,
            ],
            note_cm: [
                0x7b, 0x48, 0xa8, 0x37, 0x5d, 0x3e, 0xbd, 0x56, 0xbc, 0x64, 0x9b, 0xb5, 0xb5, 0x24,
                0x23, 0x36, 0xc2, 0xa0, 0x5a, 0x08, 0x03, 0x23, 0x9b, 0x5b, 0x88, 0xfd, 0x92, 0x07,
                0x8f, 0xea, 0x4d, 0x04,
            ],
            note_pos: 1814747072,
            note_nf: [
                0xa8, 0x2f, 0x17, 0x50, 0xcc, 0x5b, 0x2b, 0xee, 0x64, 0x9a, 0x36, 0x5c, 0x04, 0x20,
                0xed, 0x87, 0x07, 0x5b, 0x88, 0x71, 0xfd, 0xa4, 0xa7, 0xf5, 0x84, 0x0d, 0x6b, 0xbe,
                0xb1, 0x7c, 0xd6, 0x20,
            ],
        },
        TestVector {
            sk: [
                0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09,
                0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09,
                0x09, 0x09, 0x09, 0x09,
            ],
            ask: [
                0xeb, 0xbb, 0x40, 0xa9, 0x80, 0xba, 0x3b, 0x88, 0x60, 0x94, 0x8d, 0x01, 0x1e, 0x1b,
                0xfb, 0x4a, 0xff, 0xe1, 0x6c, 0x65, 0x2e, 0x90, 0xe9, 0x82, 0x58, 0x30, 0x2f, 0x44,
                0x64, 0xc9, 0x1e, 0x0c,
            ],
            nsk: [
                0x68, 0x43, 0x1b, 0x19, 0x91, 0x04, 0x21, 0x52, 0x00, 0xb9, 0x5e, 0xe5, 0xcb, 0x71,
                0xbf, 0x8b, 0x88, 0x3a, 0x3e, 0x95, 0xb7, 0x98, 0x9c, 0xad, 0x19, 0x70, 0x63, 0x14,
                0x1e, 0xbb, 0xfd, 0x00,
            ],
            ovk: [
                0x57, 0x34, 0x67, 0xa7, 0xb3, 0x0e, 0xad, 0x6c, 0xcc, 0x50, 0x47, 0x44, 0xca, 0x9e,
                0x1a, 0x28, 0x1a, 0x0d, 0x1a, 0x08, 0x73, 0x8b, 0x06, 0xa0, 0x68, 0x4f, 0xea, 0xcd,
                0x1e, 0x9d, 0x12, 0x6d,
            ],
            ak: [
                0x71, 0xc3, 0x52, 0x3e, 0xec, 0xa3, 0x53, 0x11, 0xfb, 0xd5, 0xd7, 0xe7, 0xd7, 0x0b,
                0x70, 0x9d, 0x6c, 0x35, 0xa2, 0x4f, 0x26, 0x2b, 0x34, 0xbf, 0x64, 0x05, 0x9b, 0xf2,
                0xc0, 0x2e, 0x0b, 0xa8,
            ],
            nk: [
                0x62, 0x44, 0x00, 0x10, 0x3b, 0x65, 0x69, 0xb7, 0x35, 0x8f, 0xe8, 0x0f, 0x6f, 0x6c,
                0xad, 0x43, 0x25, 0xde, 0xfd, 0xa9, 0xd9, 0x49, 0x9c, 0x2b, 0x8f, 0x88, 0x6a, 0x62,
                0x69, 0xa2, 0xaa, 0x52,
            ],
            ivk: [
                0xdb, 0x95, 0xea, 0x8b, 0xd9, 0xf9, 0x3d, 0x41, 0xb5, 0xab, 0x2b, 0xeb, 0xc9, 0x1a,
                0x38, 0xed, 0xd5, 0x27, 0x08, 0x3e, 0x2a, 0x6e, 0xf9, 0xf3, 0xc2, 0x97, 0x02, 0xd5,
                0xff, 0x89, 0xed, 0x00,
            ],
            default_d: [
                0x23, 0x3c, 0x4a, 0xb8, 0x86, 0xa5, 0x5e, 0x3b, 0xa3, 0x74, 0xc0,
            ],
            default_pk_d: [
                0xb6, 0x8e, 0x9e, 0xe0, 0xc0, 0x67, 0x8d, 0x7b, 0x30, 0x36, 0x93, 0x1c, 0x83, 0x1a,
                0x25, 0x25, 0x5f, 0x7e, 0xe4, 0x87, 0x38, 0x5a, 0x30, 0x31, 0x6e, 0x15, 0xf6, 0x48,
                0x2b, 0x87, 0x4f, 0xda,
            ],
            note_v: 17811330145809239872,
            note_r: [
                0x6e, 0xbb, 0xed, 0x74, 0x36, 0x19, 0xa2, 0x56, 0xf9, 0xad, 0x2e, 0x85, 0x88, 0x0c,
                0xfa, 0xa9, 0x09, 0x8a, 0x5f, 0xdb, 0x16, 0x29, 0x99, 0x0d, 0x9a, 0x7d, 0x3b, 0xb9,
                0x3f, 0xc9, 0x00, 0x03,
            ],
            note_cm: [
                0xd3, 0x76, 0xa7, 0xbe, 0xe8, 0xce, 0x67, 0xf4, 0xef, 0xde, 0x56, 0xaa, 0x77, 0xcf,
                0x64, 0x41, 0x9b, 0x0e, 0x55, 0x0a, 0xbb, 0xcb, 0x8e, 0x2b, 0xcb, 0xda, 0x8b, 0x63,
                0xe4, 0x1d, 0xeb, 0x37,
            ],
            note_pos: 2578461368,
            note_nf: [
                0x65, 0x36, 0x74, 0x87, 0x3b, 0x3c, 0x67, 0x0c, 0x58, 0x85, 0x84, 0x73, 0xe7, 0xfe,
                0x72, 0x19, 0x72, 0xfb, 0x96, 0xe2, 0x15, 0xb8, 0x73, 0x77, 0xa1, 0x7c, 0xa3, 0x71,
                0x0d, 0x93, 0xc9, 0xe9,
            ],
        },
    ];

    for tv in test_vectors {
        let mut ask_repr = FsRepr::default();
        let mut nsk_repr = FsRepr::default();
        ask_repr.read_le(&tv.ask[..]).unwrap();
        nsk_repr.read_le(&tv.nsk[..]).unwrap();
        let nsk = <Bls12 as JubjubEngine>::Fs::from_repr(nsk_repr).unwrap();

        let ak = JUBJUB
            .generator(FixedGenerators::SpendingKeyGenerator)
            .mul(ask_repr.clone(), &JUBJUB);
        {
            let mut vec = Vec::new();
            ak.write(&mut vec).unwrap();
            assert_eq!(&vec, &tv.ak);
        }
        {
            let mut ak = [0u8; 32];
            librustzcash_ask_to_ak(&tv.ask, &mut ak);
            assert_eq!(&ak, &tv.ak);
        }

        let pgk = ProofGenerationKey { ak, nsk };
        let fvk = pgk.to_viewing_key(&JUBJUB);
        {
            let mut vec = Vec::new();
            fvk.nk.write(&mut vec).unwrap();
            assert_eq!(&vec, &tv.nk);
        }
        {
            let mut nk = [0u8; 32];
            librustzcash_nsk_to_nk(&tv.nsk, &mut nk);
            assert_eq!(&nk, &tv.nk);
        }

        {
            let mut vec = Vec::new();
            fvk.ivk().into_repr().write_le(&mut vec).unwrap();
            assert_eq!(&vec, &tv.ivk);
        }
        {
            let mut ivk = [0u8; 32];
            librustzcash_crh_ivk(&tv.ak, &tv.nk, &mut ivk);
            assert_eq!(&ivk, &tv.ivk);
        }

        let diversifier = Diversifier(tv.default_d);
        assert!(librustzcash_check_diversifier(&tv.default_d));

        let addr = fvk.to_payment_address(diversifier, &JUBJUB).unwrap();
        {
            let mut vec = Vec::new();
            addr.pk_d().write(&mut vec).unwrap();
            assert_eq!(&vec, &tv.default_pk_d);
        }
        {
            let mut default_pk_d = [0u8; 32];
            librustzcash_ivk_to_pkd(&tv.ivk, &tv.default_d, &mut default_pk_d);
            assert_eq!(&default_pk_d, &tv.default_pk_d);
        }

        let mut note_r_repr = FsRepr::default();
        note_r_repr.read_le(&tv.note_r[..]).unwrap();
        let note_r = <Bls12 as JubjubEngine>::Fs::from_repr(note_r_repr).unwrap();
        let note = addr
            .create_note(*ASSET_TYPE_DEFAULT, tv.note_v, note_r, &JUBJUB)
            .unwrap();
        {
            let mut vec = Vec::new();
            note.cm(&JUBJUB).into_repr().write_le(&mut vec).unwrap();
            assert_eq!(&vec, &tv.note_cm);
        }

        assert_eq!(note.nf(&fvk, tv.note_pos, &JUBJUB), tv.note_nf);
    }
}
