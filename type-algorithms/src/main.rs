#![feature(generic_const_exprs)]
#![feature(const_type_id)]

trait Crypto {
    type AeadKey<Alg: AeadTypeAlg>: Sized;

    fn random_key<Alg: AeadTypeAlg>() -> Self::AeadKey<Alg>;
}

trait AeadTypeAlg: 'static {
}

struct AesCcm16_64_128;
impl AeadTypeAlg for AesCcm16_64_128 {
}
struct AnySupportedAead;
impl AeadTypeAlg for AnySupportedAead {
}

struct MyAgileCrypto {
}

impl Crypto for MyAgileCrypto {
    // always agile, no type state
    type AeadKey<Alg: AeadTypeAlg> = MyAnyKey;

    fn random_key<Alg: AeadTypeAlg>() -> Self::AeadKey<Alg> {
        todo!()
    }
}

struct MyOneTrickPonyCrypto {
}

impl Crypto for MyOneTrickPonyCrypto {
    // or err out generating it
    type AeadKey<Alg: AeadTypeAlg> = MyKeyAesCcm16_64_128;

    fn random_key<Alg: AeadTypeAlg>() -> Self::AeadKey<Alg> {
        todo!()
    }
}

struct MyGenericCrypto {
}

impl Crypto for MyGenericCrypto {
    // or err out generating it
    type AeadKey<Alg: AeadTypeAlg> = MyGenericCryptoType<Alg>;

    fn random_key<Alg: AeadTypeAlg>() -> Self::AeadKey<Alg> {
        todo!()
    }
}

enum MyAnyKey {
    AesCcm16_64_128(MyKeyAesCcm16_64_128)
}

struct MyKeyAesCcm16_64_128 {
    _private: u8,
}

enum MyGenericCryptoType<Alg: AeadTypeAlg> {
    Dynamic(MyAnyKey, UnitIfMatchesElseInfallible<AnySupportedAead, Alg>),
    AesCcm16_64_128(MyKeyAesCcm16_64_128, UnitIfMatchesElseInfallible<AesCcm16_64_128, Alg>),
}

// A lot of things makes this not work: the unstable features, but even then we can't const compare
// type IDs.
//
// Nonetheless, we don't really *need* this: If anybody wants to run build-time configuerd
// algorithms but more than one (like, when you don't have crypto agility but have two non-agile
// syustems in there that want to share some algs), one could still build
// MyAlgBuilder<AesCcm16_16_128, Sha256, Ed25519> and MyAlgBuilder<ChaCha20Poly1305, Sha256,
// Ed25519> both and rely that the asymmetric parts are by the compiler unified anyway.
struct UnitIfMatchesElseInfallible<T1: 'static, T2: 'static> {
    // or & or &mut or *const I have no clue yet, just making the compiler happy
    _t1: core::marker::PhantomData<T1>,
    _t2: core::marker::PhantomData<T2>,
    _private: [core::convert::Infallible; (core::any::TypeId::of::<T1>() == core::any::TypeId::of::<T2>()) as usize]
}


fn main() {
    println!("Hello, world!");
}
