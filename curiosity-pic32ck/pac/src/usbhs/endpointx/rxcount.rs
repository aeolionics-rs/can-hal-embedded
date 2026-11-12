#[doc = "Register `RXCOUNT` reader"]
pub type R = crate::R<RxcountSpec>;
#[doc = "Field `ENDPOINTRXCOUNT` reader - Number Of Data Bytes In The Packet Currently In Line To Be Read From The RX FIFO."]
pub type EndpointrxcountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Number Of Data Bytes In The Packet Currently In Line To Be Read From The RX FIFO."]
    #[inline(always)]
    pub fn endpointrxcount(&self) -> EndpointrxcountR {
        EndpointrxcountR::new(self.bits & 0x3fff)
    }
}
#[doc = "ENDPOINTX Number Of Data Bytes In The Packet Currently In Line To Be Read From The RX FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcount::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcountSpec;
impl crate::RegisterSpec for RxcountSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxcount::R`](R) reader structure"]
impl crate::Readable for RxcountSpec {}
#[doc = "`reset()` method sets RXCOUNT to value 0"]
impl crate::Resettable for RxcountSpec {}
