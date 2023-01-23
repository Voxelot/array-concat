use std::marker::PhantomData;
use std::mem;
use fuel_types::Bytes32;
use uninit::prelude::*;
use uninit::uninit_array;
use uninit::read::ReadIntoUninit;
use rkyv::{AlignedVec, Archive, Serialize};

#[derive(Clone, Debug, Default)]
pub struct MultiKey<K1: AsRef<[u8]>, K2: AsRef<[u8]>> {
    _marker_1: PhantomData<K1>,
    _marker_2: PhantomData<K2>,
    inner: Vec<u8>,
}

impl<K1: AsRef<[u8]>, K2: AsRef<[u8]>> MultiKey<K1, K2> {
    pub fn new(key: &(K1, K2)) -> Self {
        Self {
            _marker_1: Default::default(),
            _marker_2: Default::default(),
            inner: key
                .0
                .as_ref()
                .iter()
                .chain(key.1.as_ref().iter())
                .copied()
                .collect(),
        }
    }
}

impl<K1: AsRef<[u8]>, K2: AsRef<[u8]>> AsRef<[u8]> for MultiKey<K1, K2> {
    fn as_ref(&self) -> &[u8] {
        self.inner.as_slice()
    }
}

impl<K1: AsRef<[u8]>, K2: AsRef<[u8]>> From<MultiKey<K1, K2>> for Vec<u8> {
    fn from(key: MultiKey<K1, K2>) -> Vec<u8> {
        key.inner
    }
}

pub struct NewMultiKey {
    key: [u8; 64],
}

impl NewMultiKey {
    pub fn new(k1: &Bytes32, k2: &Bytes32) -> Self {
        let mut output = uninit_array![u8; 64];
        let buf: Out<[u8]> = output.as_out();

        let mut chain = ReadIntoUninit::chain(k1.as_ref(), k2.as_ref());
        chain.read_into_uninit(buf).unwrap();
        // we can thus soundly `assume_init` our array:
        let key = unsafe { mem::transmute::<[MaybeUninit<u8>; 64], [u8; 64]>(output) };
        Self { key }
    }
}

impl AsRef<[u8]> for NewMultiKey {
    fn as_ref(&self) -> &[u8] {
        self.key.as_ref()
    }
}

#[derive(Serialize, Archive)]
pub struct CustomKey {
    pub k1: [u8; 32],
    pub k2: [u8; 32]
}

impl CustomKey {
    pub fn to_bytes(&self) -> AlignedVec {
        rkyv::to_bytes::<_, 64>(self).unwrap()
    }
}

