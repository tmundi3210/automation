//! Integration tests for the crypto primitives through the public API
//! (docs/DESIGN.md §1 rows `crypto/*`; §10 "ed25519 strict (vectors:
//! small-order keys, non-canonical R/s/A are rejected; identical verdict table
//! checked into `tests/vectors/ed25519_strict.json` and re-run through the
//! Kotlin JVM smoke test)").
//!
//! `tests/vectors/ed25519_strict.json` is the cross-platform conformance
//! table required by BRIEF_v2 §5.1 ("one strict signature verifier behaves
//! identically on iOS and Android"). Every case records this crate's verdict
//! (`expect`), the stage that produced it (`stage`: `admission`, `signature`
//! or `accepted`) and, for context, the verdict of a lax verifier
//! (`lax_verdict`: `ed25519-dalek`'s non-strict `verify` after a plain
//! `from_bytes`). The test below asserts all three columns.
//!
//! Regenerate the table after an *intentional* rule change with
//! `cargo test -p meshcore --test crypto regenerate_ed25519_strict_vectors -- --ignored`
//! and review the diff; the generator re-asserts every verdict while writing.

use ed25519_dalek::hazmat::ExpandedSecretKey;
use ed25519_dalek::{Signature, SigningKey, Verifier, VerifyingKey};
use meshcore::crypto::aead::{self, Key32, TAG_LEN};
use meshcore::crypto::ed25519::{
    IdentitySigner, KeyReject, SigError, admit_public_key, verify, verify_raw,
};
use meshcore::crypto::hash::{Hash32, hkdf32, object_hash, prf16, sha256};
use meshcore::crypto::keycard::{KEYCARD_DOMAIN, KEYCARD_LEN, KeyCard, KeyCardError};
use proptest::prelude::*;
use serde_json::{Value, json};

const VECTORS_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/vectors/ed25519_strict.json"
);

/// Case names the table must contain (the task's required coverage).
const REQUIRED_CASES: &[&str] = &[
    "rfc8032_test1_raw",
    "rfc8032_test2_raw",
    "rfc8032_test3_raw",
    "valid_from_seed",
    "valid_empty_message",
    "tampered_message",
    "wrong_domain",
    "sig_s_plus_l",
    "sig_s_equals_l",
    "sig_r_identity",
    "sig_r_not_on_curve",
    "sig_all_zero",
    "sig_r_identity_s_consistent",
    "sig_r_identity_noncanonical_y_p_plus_1",
    "sig_r_identity_sign_bit",
    "sig_r_order8_s_consistent",
    "small_order_1",
    "small_order_2",
    "small_order_3",
    "small_order_4",
    "small_order_5",
    "small_order_6",
    "small_order_7",
    "small_order_8",
    "noncanonical_y_eq_p",
    "noncanonical_y_eq_p_plus_1",
    "noncanonical_identity_sign_bit",
    "noncanonical_large_order",
    "pubkey_not_on_curve",
];

/// The eight small-order points, canonical encodings (orders 1, 2, 4, 4, 8, 8, 8, 8).
const SMALL_ORDER: [&str; 8] = [
    "0100000000000000000000000000000000000000000000000000000000000000",
    "ecffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f",
    "0000000000000000000000000000000000000000000000000000000000000080",
    "0000000000000000000000000000000000000000000000000000000000000000",
    "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac037a",
    "c7176a703d4dd84fba3c0b760d10670f2a2053fa2c39ccc64ec7fd7792ac03fa",
    "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc05",
    "26e8958fc2b227b045c3f489f2ef98f0d5dfac05d3c63339b13802886d53fc85",
];

/// The group order L, little-endian (RFC 8032 §5.1).
const L_LE: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x10,
];

fn hex32(s: &str) -> [u8; 32] {
    hex::decode(s).unwrap().try_into().unwrap()
}

fn hex64(s: &str) -> [u8; 64] {
    hex::decode(s).unwrap().try_into().unwrap()
}

/// What this crate signs/verifies for a case: `domain ‖ 0x00 ‖ msg`, or the
/// raw message when the case has no domain (RFC vectors).
fn signed_bytes(domain: Option<&str>, msg: &[u8]) -> Vec<u8> {
    match domain {
        Some(d) => [d.as_bytes(), &[0u8], msg].concat(),
        None => msg.to_vec(),
    }
}

/// This crate's verdict and the stage that produced it.
fn strict_verdict(
    domain: Option<&str>,
    pk: &[u8; 32],
    msg: &[u8],
    sig: &[u8; 64],
) -> (&'static str, &'static str) {
    let key = match admit_public_key(pk) {
        Ok(k) => k,
        Err(_) => return ("reject", "admission"),
    };
    let result = match domain {
        Some(d) => verify(d, &key, msg, sig),
        None => verify_raw(&key, msg, sig),
    };
    match result {
        Ok(()) => ("accept", "accepted"),
        Err(SigError::Invalid) => ("reject", "signature"),
        Err(SigError::Key(_)) => ("reject", "admission"),
    }
}

/// A signature with a chosen `R` encoding and `S = k·a mod L`, where
/// `k = SHA-512(R ‖ A ‖ M)` and `a` is the private scalar of `seed`.
///
/// When `R` decodes to the identity, `[S]B = [k]A = R + [k]A`: the bare
/// verification equation holds, so a verifier that checks nothing else accepts
/// the signature (and `a = S/k` leaks). Strict verification rejects every
/// small-order `R`. These are the only inputs on which a lax verifier and this
/// crate disagree at the *signature* stage, so the table needs them to pin the
/// verifier and not only the admission rules.
fn forged_r_signature(seed: &[u8; 32], r_bytes: &[u8; 32], msg: &[u8]) -> [u8; 64] {
    use sha2::{Digest, Sha512};

    let esk = ExpandedSecretKey::from_bytes(&Sha512::digest(seed).into());
    let a = esk.scalar;
    let pk = SigningKey::from_bytes(seed).verifying_key().to_bytes();
    let k_hash: [u8; 64] = Sha512::new()
        .chain_update(r_bytes)
        .chain_update(pk)
        .chain_update(msg)
        .finalize()
        .into();
    // k = k_hash mod L by Horner over 128-bit little-endian limbs; `From<u128>`
    // reduces mod L. `like` only pins the (unnameable here) scalar type.
    fn scalar_like<S: From<u128>>(_like: &S, v: u128) -> S {
        S::from(v)
    }
    let two_128 = scalar_like(&a, 1u128 << 127) * scalar_like(&a, 2);
    let mut k = scalar_like(&a, 0);
    for limb in k_hash.chunks(16).rev() {
        let limb = u128::from_le_bytes(limb.try_into().unwrap());
        k = k * two_128 + scalar_like(&a, limb);
    }
    let s = k * a;
    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(r_bytes);
    sig[32..].copy_from_slice(&s.to_bytes());
    sig
}

/// A lax verifier: `ed25519-dalek` non-strict `verify` after a plain `from_bytes`
/// (no admission, no small-order check on A or R).
fn lax_verdict(pk: &[u8; 32], signed: &[u8], sig: &[u8; 64]) -> &'static str {
    match VerifyingKey::from_bytes(pk) {
        Err(_) => "reject",
        Ok(vk) => {
            if vk.verify(signed, &Signature::from_bytes(sig)).is_ok() {
                "accept"
            } else {
                "reject"
            }
        }
    }
}

fn load_cases() -> Vec<Value> {
    let text = std::fs::read_to_string(VECTORS_PATH)
        .unwrap_or_else(|e| panic!("cannot read {VECTORS_PATH}: {e}"));
    let table: Value = serde_json::from_str(&text).expect("vector file is valid JSON");
    table["cases"]
        .as_array()
        .expect("`cases` is an array")
        .clone()
}

#[test]
fn ed25519_strict_vector_table_verdicts() {
    let cases = load_cases();
    assert!(cases.len() >= 12, "table has {} cases", cases.len());
    let mut names = std::collections::BTreeSet::new();
    let mut accepts = 0;
    let mut rejects = 0;
    for case in &cases {
        let name = case["name"].as_str().expect("name");
        assert!(names.insert(name.to_owned()), "duplicate case name {name}");
        let domain = case["domain"].as_str();
        let pk = hex32(case["pubkey_hex"].as_str().expect("pubkey_hex"));
        let msg = hex::decode(case["msg_hex"].as_str().expect("msg_hex")).unwrap();
        let sig = hex64(case["sig_hex"].as_str().expect("sig_hex"));
        let expect = case["expect"].as_str().expect("expect");
        let stage = case["stage"].as_str().expect("stage");
        let lax = case["lax_verdict"].as_str().expect("lax_verdict");
        let reason = case["reason"].as_str().expect("reason");
        assert!(
            matches!(expect, "accept" | "reject"),
            "{name}: expect = {expect}"
        );
        assert_eq!(
            expect == "accept",
            stage == "accepted",
            "{name}: stage {stage} vs {expect}"
        );

        let (got, got_stage) = strict_verdict(domain, &pk, &msg, &sig);
        assert_eq!(got, expect, "case {name}: {reason}");
        assert_eq!(got_stage, stage, "case {name}: stage; {reason}");
        assert_eq!(
            lax_verdict(&pk, &signed_bytes(domain, &msg), &sig),
            lax,
            "case {name}: lax column"
        );
        match expect {
            "accept" => accepts += 1,
            _ => rejects += 1,
        }
    }
    assert!(
        accepts >= 3 && rejects >= 9,
        "{accepts} accepts, {rejects} rejects"
    );
    // The table must pin the verifier and not only admission: without a case
    // that a lax verifier accepts and this crate rejects *at the signature
    // stage*, swapping `verify_strict` for `verify` passes every vector.
    assert!(
        cases
            .iter()
            .any(|c| c["stage"] == "signature" && c["lax_verdict"] == "accept"),
        "no case distinguishes strict from lax verification at the signature stage"
    );
    for required in REQUIRED_CASES {
        assert!(
            names.contains(*required),
            "missing required case {required}"
        );
    }
    // Every one of the eight small-order encodings is present and rejected at admission.
    for (i, enc) in SMALL_ORDER.iter().enumerate() {
        let case = cases
            .iter()
            .find(|c| c["name"] == format!("small_order_{}", i + 1))
            .expect("small-order case present");
        assert_eq!(case["pubkey_hex"], *enc);
        assert_eq!(case["expect"], "reject");
        assert_eq!(case["stage"], "admission");
        assert_eq!(admit_public_key(&hex32(enc)), Err(KeyReject::SmallOrder));
    }
}

#[test]
fn ed25519_small_order_forgeries_fool_a_lax_verifier_but_not_this_one() {
    // The table's small-order cases carry a forged (R = identity, S = 0)
    // signature that a lax verifier accepts; this is why admission matters.
    for case in load_cases().iter().filter(|c| {
        c["name"]
            .as_str()
            .is_some_and(|n| n.starts_with("small_order_"))
    }) {
        assert_eq!(case["lax_verdict"], "accept", "{}", case["name"]);
        assert_eq!(case["expect"], "reject", "{}", case["name"]);
    }
}

#[test]
fn ed25519_strict_r_rule_is_pinned_by_the_table() {
    // `sig_r_identity_s_consistent` satisfies the bare equation (the lax
    // column says accept) and is rejected here only by the small-order-R rule
    // of strict verification. Lax verifiers on another platform fail this.
    let cases = load_cases();
    let case = cases
        .iter()
        .find(|c| c["name"] == "sig_r_identity_s_consistent")
        .expect("sig_r_identity_s_consistent present");
    assert_eq!(case["expect"], "reject");
    assert_eq!(case["stage"], "signature");
    assert_eq!(case["lax_verdict"], "accept");
    assert_eq!(case["sig_hex"].as_str().unwrap()[..64], *SMALL_ORDER[0]);
    let domain = case["domain"].as_str().expect("domain");
    let key = admit_public_key(&hex32(case["pubkey_hex"].as_str().unwrap())).expect("admits");
    let msg = hex::decode(case["msg_hex"].as_str().unwrap()).unwrap();
    let sig = hex64(case["sig_hex"].as_str().unwrap());
    assert_eq!(verify(domain, &key, &msg, &sig), Err(SigError::Invalid));
    assert_eq!(
        lax_verdict(key.as_bytes(), &signed_bytes(Some(domain), &msg), &sig),
        "accept"
    );
}

/// Maintainer tool: rebuild `tests/vectors/ed25519_strict.json`. Every verdict
/// is asserted while generating so a wrong expectation fails here, not later.
#[test]
#[ignore = "rewrites tests/vectors/ed25519_strict.json; run on purpose"]
fn regenerate_ed25519_strict_vectors() {
    struct Case {
        name: String,
        domain: Option<String>,
        pk: [u8; 32],
        msg: Vec<u8>,
        sig: [u8; 64],
        expect: &'static str,
        stage: &'static str,
        reason: String,
    }
    let mut cases: Vec<Case> = Vec::new();
    let mut push = |name: &str,
                    domain: Option<&str>,
                    pk: [u8; 32],
                    msg: Vec<u8>,
                    sig: [u8; 64],
                    expect: &'static str,
                    stage: &'static str,
                    reason: String| {
        let (got, got_stage) = strict_verdict(domain, &pk, &msg, &sig);
        assert_eq!(
            (got, got_stage),
            (expect, stage),
            "while generating {name}: {reason}"
        );
        cases.push(Case {
            name: name.to_owned(),
            domain: domain.map(str::to_owned),
            pk,
            msg,
            sig,
            expect,
            stage,
            reason,
        });
    };

    // RFC 8032 §7.1 TEST 1-3: raw messages, no domain. Proves the verifier is
    // plain Ed25519 and so identical on every platform.
    let rfc = [
        (
            "rfc8032_test1_raw",
            "d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a",
            "",
            "e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e065224901555fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
        ),
        (
            "rfc8032_test2_raw",
            "3d4017c3e843895a92b70aa74d1b7ebc9c982ccf2ec4968cc0cd55f12af4660c",
            "72",
            "92a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
        ),
        (
            "rfc8032_test3_raw",
            "fc51cd8e6218a1a38da47ed00230f0580816ed13ba3303ac5deb911548908025",
            "af82",
            "6291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a",
        ),
    ];
    for (name, pk, msg, sig) in rfc {
        push(
            name,
            None,
            hex32(pk),
            hex::decode(msg).unwrap(),
            hex64(sig),
            "accept",
            "accepted",
            "RFC 8032 §7.1 test vector over the raw message (no domain prefix)".to_owned(),
        );
    }

    // A key from a fixed seed and a domain-separated signature.
    const DOMAIN: &str = "meshapp.v1.test";
    let seed: [u8; 32] = core::array::from_fn(|i| i as u8);
    let signer = IdentitySigner::from_seed(&seed);
    let pk = signer.public().to_bytes();
    let msg = b"hello mesh".to_vec();
    let sig = signer.sign(DOMAIN, &msg);
    push(
        "valid_from_seed",
        Some(DOMAIN),
        pk,
        msg.clone(),
        sig,
        "accept",
        "accepted",
        "key from seed 000102..1f; signature over domain ‖ 0x00 ‖ msg".to_owned(),
    );
    push(
        "valid_empty_message",
        Some(DOMAIN),
        pk,
        Vec::new(),
        signer.sign(DOMAIN, b""),
        "accept",
        "accepted",
        "same key, empty message (the signed bytes are just domain ‖ 0x00)".to_owned(),
    );
    let mut tampered = msg.clone();
    tampered[0] ^= 0x01;
    push(
        "tampered_message",
        Some(DOMAIN),
        pk,
        tampered,
        sig,
        "reject",
        "signature",
        "valid_from_seed signature over a message with one bit flipped".to_owned(),
    );
    push(
        "wrong_domain",
        Some("meshapp.v1.other"),
        pk,
        msg.clone(),
        sig,
        "reject",
        "signature",
        "valid_from_seed signature verified under another domain string".to_owned(),
    );

    // S + L: the same scalar mod L, so a verifier that reduces S accepts it;
    // RFC 8032 §5.1.7 requires 0 <= S < L.
    let mut s_plus_l = sig;
    let mut carry = 0u16;
    for i in 0..32 {
        let v = sig[32 + i] as u16 + L_LE[i] as u16 + carry;
        s_plus_l[32 + i] = v as u8;
        carry = v >> 8;
    }
    assert_eq!(carry, 0);
    push(
        "sig_s_plus_l",
        Some(DOMAIN),
        pk,
        msg.clone(),
        s_plus_l,
        "reject",
        "signature",
        "valid_from_seed signature with L added to S (S ≥ L, non-canonical scalar); a verifier that reduces S mod L would accept".to_owned(),
    );
    let mut s_eq_l = sig;
    s_eq_l[32..].copy_from_slice(&L_LE);
    push(
        "sig_s_equals_l",
        Some(DOMAIN),
        pk,
        msg.clone(),
        s_eq_l,
        "reject",
        "signature",
        "S = L exactly, which encodes the scalar 0 non-canonically".to_owned(),
    );
    let mut r_identity = sig;
    r_identity[..32].copy_from_slice(&hex32(SMALL_ORDER[0]));
    push(
        "sig_r_identity",
        Some(DOMAIN),
        pk,
        msg.clone(),
        r_identity,
        "reject",
        "signature",
        "R replaced by the identity point (small order); strict verification rejects small-order R"
            .to_owned(),
    );

    // A y that is not on the curve (first one found with a small y).
    let off_curve = (2u8..=40)
        .map(|y| {
            let mut b = [0u8; 32];
            b[0] = y;
            b
        })
        .find(|b| admit_public_key(b) == Err(KeyReject::NotOnCurve))
        .expect("a y in 2..=40 that is not on the curve");
    let mut r_off = sig;
    r_off[..32].copy_from_slice(&off_curve);
    push(
        "sig_r_not_on_curve",
        Some(DOMAIN),
        pk,
        msg.clone(),
        r_off,
        "reject",
        "signature",
        "R replaced by an encoding that is not a curve point".to_owned(),
    );
    push(
        "sig_all_zero",
        Some(DOMAIN),
        pk,
        msg.clone(),
        [0u8; 64],
        "reject",
        "signature",
        "all-zero signature (R = the order-4 point with y = 0, S = 0)".to_owned(),
    );

    // Small-order or non-canonical R with S = k·a mod L chosen by the key
    // holder so that the bare equation [S]B = R + [k]A holds. These are the
    // only cases where a lax verifier and a strict one disagree at the
    // signature stage, so they pin which verifier the port uses.
    let signed = signed_bytes(Some(DOMAIN), &msg);
    let identity = hex32(SMALL_ORDER[0]);
    push(
        "sig_r_identity_s_consistent",
        Some(DOMAIN),
        pk,
        msg.clone(),
        forged_r_signature(&seed, &identity, &signed),
        "reject",
        "signature",
        "R = identity (canonical) and S = k·a mod L, so [S]B = R + [k]A holds and ed25519-dalek's non-strict verify ACCEPTS; strict verification rejects small-order R. Such a signature also leaks the private scalar (a = S/k)".to_owned(),
    );
    push(
        "sig_r_identity_noncanonical_y_p_plus_1",
        Some(DOMAIN),
        pk,
        msg.clone(),
        forged_r_signature(
            &seed,
            &hex32("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f"),
            &signed,
        ),
        "reject",
        "signature",
        "R = identity encoded non-canonically (y = p + 1) with S = k·a mod L over these R bytes; a verifier that compares decoded points (ZIP-215 style) accepts; rejected: non-canonical and small-order R".to_owned(),
    );
    push(
        "sig_r_identity_sign_bit",
        Some(DOMAIN),
        pk,
        msg.clone(),
        forged_r_signature(
            &seed,
            &hex32("0100000000000000000000000000000000000000000000000000000000000080"),
            &signed,
        ),
        "reject",
        "signature",
        "R = identity with the sign bit set although x = 0, S = k·a mod L over these R bytes; a verifier that compares decoded points accepts; rejected: non-canonical and small-order R".to_owned(),
    );
    push(
        "sig_r_order8_s_consistent",
        Some(DOMAIN),
        pk,
        msg.clone(),
        forged_r_signature(&seed, &hex32(SMALL_ORDER[4]), &signed),
        "reject",
        "signature",
        "R = an order-8 point, S = k·a mod L: the cofactored equation [8S]B = [8]R + [8k]A holds, so a cofactored verifier accepts; rejected: small-order R".to_owned(),
    );

    // Small-order keys with the classic forgery R = identity, S = 0: the
    // cofactorless equation [S]B = R + [k]A holds whenever order(A) divides
    // k = H(R ‖ A ‖ M), so a message is searched for which a lax verifier
    // accepts. Admission must reject the key before any of that runs.
    let mut forged = [0u8; 64];
    forged[..32].copy_from_slice(&hex32(SMALL_ORDER[0]));
    let forge_message = |pk: &[u8; 32]| -> Vec<u8> {
        let vk = VerifyingKey::from_bytes(pk).expect("decompresses");
        (0u32..100_000)
            .map(|i| format!("forge {i}").into_bytes())
            .find(|m| {
                vk.verify(
                    &signed_bytes(Some(DOMAIN), m),
                    &Signature::from_bytes(&forged),
                )
                .is_ok()
            })
            .expect("a forgeable message exists for a small-order key")
    };
    let orders = [1, 2, 4, 4, 8, 8, 8, 8];
    for (i, enc) in SMALL_ORDER.iter().enumerate() {
        let key = hex32(enc);
        assert_eq!(admit_public_key(&key), Err(KeyReject::SmallOrder), "{enc}");
        let m = forge_message(&key);
        push(
            &format!("small_order_{}", i + 1),
            Some(DOMAIN),
            key,
            m,
            forged,
            "reject",
            "admission",
            format!(
                "small-order public key (order {}) with the forged signature R = identity, S = 0, which a lax verifier accepts for this message; rejected at admission (KeyReject::SmallOrder) before any signature check",
                orders[i]
            ),
        );
    }

    // Non-canonical encodings: y ≥ p, or the sign bit set while x = 0.
    // ed25519-dalek's from_bytes reduces these silently; admission requires
    // decompress → recompress to reproduce the input.
    let y_eq_p = hex32("edffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
    assert_eq!(admit_public_key(&y_eq_p), Err(KeyReject::NonCanonical));
    push(
        "noncanonical_y_eq_p",
        Some(DOMAIN),
        y_eq_p,
        forge_message(&y_eq_p),
        forged,
        "reject",
        "admission",
        "y = p (non-canonical encoding of the order-4 point y = 0); rejected at admission (KeyReject::NonCanonical); a lax verifier reduces y and accepts the forgery".to_owned(),
    );
    let y_eq_p1 = hex32("eeffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7f");
    assert_eq!(admit_public_key(&y_eq_p1), Err(KeyReject::NonCanonical));
    push(
        "noncanonical_y_eq_p_plus_1",
        Some(DOMAIN),
        y_eq_p1,
        forge_message(&y_eq_p1),
        forged,
        "reject",
        "admission",
        "y = p + 1 (non-canonical encoding of the identity); rejected at admission (KeyReject::NonCanonical)".to_owned(),
    );
    let identity_neg = hex32("0100000000000000000000000000000000000000000000000000000000000080");
    assert_eq!(
        admit_public_key(&identity_neg),
        Err(KeyReject::NonCanonical)
    );
    push(
        "noncanonical_identity_sign_bit",
        Some(DOMAIN),
        identity_neg,
        forge_message(&identity_neg),
        forged,
        "reject",
        "admission",
        "identity point with the sign bit set although x = 0 (RFC 8032 §5.1.3 decoding must fail); rejected at admission (KeyReject::NonCanonical)".to_owned(),
    );
    // A non-canonical encoding of a *large-order* point: y + p for a small y
    // that is on the curve and not weak. Nothing but the canonical-encoding
    // rule rejects it.
    let (large, canonical) = (2u8..=18)
        .find_map(|y| {
            let mut b = [0xffu8; 32];
            b[0] = 0xed + y;
            b[31] = 0x7f;
            let vk = VerifyingKey::from_bytes(&b).ok()?;
            if vk.is_weak() {
                return None;
            }
            Some((b, vk.to_edwards().compress().to_bytes()))
        })
        .expect("a y in 2..=18 whose y + p encoding decompresses to a non-weak point");
    assert_eq!(admit_public_key(&large), Err(KeyReject::NonCanonical));
    assert!(admit_public_key(&canonical).is_ok());
    push(
        "noncanonical_large_order",
        Some(DOMAIN),
        large,
        msg.clone(),
        sig,
        "reject",
        "admission",
        format!(
            "y ≥ p encoding of a large-order point whose canonical encoding is {}; a lax decoder would give one key two identities; rejected at admission (KeyReject::NonCanonical)",
            hex::encode(canonical)
        ),
    );
    push(
        "pubkey_not_on_curve",
        Some(DOMAIN),
        off_curve,
        msg.clone(),
        sig,
        "reject",
        "admission",
        "y coordinate with no square root for x²; rejected at admission (KeyReject::NotOnCurve)"
            .to_owned(),
    );

    assert!(cases.len() >= REQUIRED_CASES.len());
    let json_cases: Vec<Value> = cases
        .iter()
        .map(|c| {
            let signed = signed_bytes(c.domain.as_deref(), &c.msg);
            json!({
                "name": c.name,
                "domain": c.domain,
                "pubkey_hex": hex::encode(c.pk),
                "msg_hex": hex::encode(&c.msg),
                "sig_hex": hex::encode(c.sig),
                "expect": c.expect,
                "stage": c.stage,
                "lax_verdict": lax_verdict(&c.pk, &signed, &c.sig),
                "reason": c.reason,
            })
        })
        .collect();
    let table = json!({
        "description": "Ed25519 strict-verifier conformance table (BRIEF_v2 §5.1; DESIGN §1 crypto/ed25519, §10). The signed bytes are domain ‖ 0x00 ‖ msg, or msg alone when domain is null. expect = this crate's verdict; stage = admission | signature | accepted; lax_verdict = ed25519-dalek non-strict verify after a plain from_bytes, for context only.",
        "generator": "cargo test -p meshcore --test crypto regenerate_ed25519_strict_vectors -- --ignored",
        "cases": json_cases,
    });
    let mut text = serde_json::to_string_pretty(&table).unwrap();
    text.push('\n');
    std::fs::write(VECTORS_PATH, text)
        .unwrap_or_else(|e| panic!("cannot write {VECTORS_PATH}: {e}"));
    eprintln!("wrote {} cases to {VECTORS_PATH}", cases.len());
}

#[test]
fn keycard_issue_encode_decode_and_reject() {
    let signer = IdentitySigner::from_seed(&[0x11u8; 32]);
    let x25519 = [0x22u8; 32];
    let card = KeyCard::issue(&signer, x25519);
    assert_eq!(card.verify(), Ok(()));
    let bytes = card.to_bytes();
    assert_eq!(bytes.len(), KEYCARD_LEN);
    assert_eq!(KeyCard::from_bytes(&bytes), Ok(card));
    // The signature is an ordinary identity signature over the x25519 key.
    assert_eq!(verify(KEYCARD_DOMAIN, &card.ed, &x25519, &card.sig), Ok(()));
    // Tampered x25519 or signature.
    let mut bad = bytes;
    bad[40] ^= 1;
    assert_eq!(KeyCard::from_bytes(&bad), Err(KeyCardError::BadSignature));
    let mut bad = bytes;
    bad[100] ^= 1;
    assert_eq!(KeyCard::from_bytes(&bad), Err(KeyCardError::BadSignature));
    // Weak identity: rejected at admission, whatever the signature says.
    let mut weak = bytes;
    weak[..32].copy_from_slice(&hex32(SMALL_ORDER[7]));
    assert_eq!(
        KeyCard::from_bytes(&weak),
        Err(KeyCardError::Key(KeyReject::SmallOrder))
    );
    // A card whose signature has R = identity and a matching S satisfies the
    // bare equation (a lax verifier accepts it) but not strict verification.
    let forged = KeyCard {
        ed: card.ed,
        x25519,
        sig: forged_r_signature(
            &[0x11u8; 32],
            &hex32(SMALL_ORDER[0]),
            &signed_bytes(Some(KEYCARD_DOMAIN), &x25519),
        ),
    };
    assert_eq!(
        lax_verdict(
            card.ed.as_bytes(),
            &signed_bytes(Some(KEYCARD_DOMAIN), &x25519),
            &forged.sig
        ),
        "accept"
    );
    assert_eq!(forged.verify(), Err(KeyCardError::BadSignature));
    assert_eq!(
        KeyCard::from_bytes(&forged.to_bytes()),
        Err(KeyCardError::BadSignature)
    );
}

#[test]
fn hash_and_aead_public_api() {
    let h = object_hash("meshapp.v1.x", b"obj");
    assert_eq!(h, sha256(b"meshapp.v1.x\x00obj"));
    assert_eq!(h.to_string().len(), 64);
    assert_eq!(Hash32::from(h.to_bytes()), h);
    assert_eq!(prf16(&[0u8; 32], "tag", 1).len(), 16);
    let key = Key32::from_bytes(hkdf32(b"ikm", b"salt", b"cell"));
    let ct = aead::seal(&key, &[0u8; 12], b"hdr", b"body");
    assert_eq!(ct.len(), 4 + TAG_LEN);
    assert_eq!(aead::open(&key, &[0u8; 12], b"hdr", &ct).unwrap(), b"body");
    assert!(aead::open(&key, &[0u8; 12], b"hdx", &ct).is_err());
    let ctx = aead::seal_x(&key, &[0u8; 24], b"", b"blob");
    assert_eq!(aead::open_x(&key, &[0u8; 24], b"", &ctx).unwrap(), b"blob");
}

proptest! {
    #[test]
    fn aead_round_trip_and_any_flip_fails(
        key in any::<[u8; 32]>(),
        nonce in any::<[u8; 12]>(),
        aad in proptest::collection::vec(any::<u8>(), 0..40),
        pt in proptest::collection::vec(any::<u8>(), 0..200),
        flip in any::<u32>(),
    ) {
        let k = Key32::from_bytes(key);
        let ct = aead::seal(&k, &nonce, &aad, &pt);
        prop_assert_eq!(ct.len(), pt.len() + TAG_LEN);
        prop_assert_eq!(aead::open(&k, &nonce, &aad, &ct).unwrap(), pt);
        let bit = (flip as usize) % (ct.len() * 8);
        let mut bad = ct.clone();
        bad[bit / 8] ^= 1 << (bit % 8);
        prop_assert!(aead::open(&k, &nonce, &aad, &bad).is_err());
    }

    #[test]
    fn ed25519_sign_verify_and_tamper(
        seed in any::<[u8; 32]>(),
        msg in proptest::collection::vec(any::<u8>(), 0..100),
        flip in any::<u16>(),
    ) {
        let s = IdentitySigner::from_seed(&seed);
        prop_assert!(admit_public_key(s.public().as_bytes()).is_ok());
        let sig = s.sign("meshapp.v1.prop", &msg);
        prop_assert_eq!(verify("meshapp.v1.prop", &s.public(), &msg, &sig), Ok(()));
        prop_assert_eq!(verify("meshapp.v1.prop2", &s.public(), &msg, &sig), Err(SigError::Invalid));
        let bit = (flip as usize) % 512;
        let mut bad = sig;
        bad[bit / 8] ^= 1 << (bit % 8);
        prop_assert_eq!(verify("meshapp.v1.prop", &s.public(), &msg, &bad), Err(SigError::Invalid));
    }
}
