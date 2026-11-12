#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_endpoint0: [u8; 0x1554],
}
impl RegisterBlock {
    #[doc = "0x00..0x1554 - Endpont 0 is selected"]
    #[inline(always)]
    pub const fn endpoint0(&self) -> &Endpoint0 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x1554 - Endpont 1-15 are selected. INDEX.SELEP value needs to be greater than 0"]
    #[inline(always)]
    pub const fn endpointx(&self) -> &Endpointx {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "Endpont 1-15 are selected. INDEX.SELEP value needs to be greater than 0"]
pub use self::endpointx::Endpointx;
#[doc = r"Cluster"]
#[doc = "Endpont 1-15 are selected. INDEX.SELEP value needs to be greater than 0"]
pub mod endpointx;
#[doc = "Endpont 0 is selected"]
pub use self::endpoint0::Endpoint0;
#[doc = r"Cluster"]
#[doc = "Endpont 0 is selected"]
pub mod endpoint0;
