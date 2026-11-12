#[doc = "Register `RXDPKTBUFDIS` reader"]
pub type R = crate::R<RxdpktbufdisSpec>;
#[doc = "Register `RXDPKTBUFDIS` writer"]
pub type W = crate::W<RxdpktbufdisSpec>;
#[doc = "Field `EPxRXDIS` reader - each bit indicates which of the Rx endpoint have disabled double buffer functionality"]
pub type EpxRxdisR = crate::FieldReader<u16>;
#[doc = "Field `EPxRXDIS` writer - each bit indicates which of the Rx endpoint have disabled double buffer functionality"]
pub type EpxRxdisW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 1:15 - each bit indicates which of the Rx endpoint have disabled double buffer functionality"]
    #[inline(always)]
    pub fn epx_rxdis(&self) -> EpxRxdisR {
        EpxRxdisR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 1:15 - each bit indicates which of the Rx endpoint have disabled double buffer functionality"]
    #[inline(always)]
    pub fn epx_rxdis(&mut self) -> EpxRxdisW<'_, RxdpktbufdisSpec> {
        EpxRxdisW::new(self, 1)
    }
}
#[doc = "Indicates Which Of The RX Endpoints Have Disabled Double Packet Buffer Functionality\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdpktbufdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdpktbufdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdpktbufdisSpec;
impl crate::RegisterSpec for RxdpktbufdisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxdpktbufdis::R`](R) reader structure"]
impl crate::Readable for RxdpktbufdisSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdpktbufdis::W`](W) writer structure"]
impl crate::Writable for RxdpktbufdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDPKTBUFDIS to value 0"]
impl crate::Resettable for RxdpktbufdisSpec {}
