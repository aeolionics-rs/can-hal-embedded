#[doc = "Register `COUNT0` reader"]
pub type R = crate::R<Count0Spec>;
#[doc = "Field `EP0RXCOUNT` reader - Number Of Received Data Bytes In Endpoint 0 FIFO"]
pub type Ep0rxcountR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Number Of Received Data Bytes In Endpoint 0 FIFO"]
    #[inline(always)]
    pub fn ep0rxcount(&self) -> Ep0rxcountR {
        Ep0rxcountR::new(self.bits & 0x7f)
    }
}
#[doc = "ENDPOINT0 Number Of Received Bytes In Endpoint 0 FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`count0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Count0Spec;
impl crate::RegisterSpec for Count0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`count0::R`](R) reader structure"]
impl crate::Readable for Count0Spec {}
#[doc = "`reset()` method sets COUNT0 to value 0"]
impl crate::Resettable for Count0Spec {}
