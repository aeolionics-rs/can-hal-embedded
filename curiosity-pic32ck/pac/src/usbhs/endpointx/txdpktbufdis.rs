#[doc = "Register `TXDPKTBUFDIS` reader"]
pub type R = crate::R<TxdpktbufdisSpec>;
#[doc = "Register `TXDPKTBUFDIS` writer"]
pub type W = crate::W<TxdpktbufdisSpec>;
#[doc = "Field `EPxTXDIS` reader - each bit indicates which of the Tx endpoint have disabled double buffer functionality"]
pub type EpxTxdisR = crate::FieldReader<u16>;
#[doc = "Field `EPxTXDIS` writer - each bit indicates which of the Tx endpoint have disabled double buffer functionality"]
pub type EpxTxdisW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 1:15 - each bit indicates which of the Tx endpoint have disabled double buffer functionality"]
    #[inline(always)]
    pub fn epx_txdis(&self) -> EpxTxdisR {
        EpxTxdisR::new((self.bits >> 1) & 0x7fff)
    }
}
impl W {
    #[doc = "Bits 1:15 - each bit indicates which of the Tx endpoint have disabled double buffer functionality"]
    #[inline(always)]
    pub fn epx_txdis(&mut self) -> EpxTxdisW<'_, TxdpktbufdisSpec> {
        EpxTxdisW::new(self, 1)
    }
}
#[doc = "Indicates Which Of The TX Endpoints Have Disabled Double Packet Buffer Functionality\n\nYou can [`read`](crate::Reg::read) this register and get [`txdpktbufdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdpktbufdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdpktbufdisSpec;
impl crate::RegisterSpec for TxdpktbufdisSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`txdpktbufdis::R`](R) reader structure"]
impl crate::Readable for TxdpktbufdisSpec {}
#[doc = "`write(|w| ..)` method takes [`txdpktbufdis::W`](W) writer structure"]
impl crate::Writable for TxdpktbufdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDPKTBUFDIS to value 0"]
impl crate::Resettable for TxdpktbufdisSpec {}
