#[doc = "Register `THR` reader"]
pub type R = crate::R<ThrSpec>;
#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `THRES` reader - SPI Control Threshold Value. SPI Control Threshold Interrupt is asserted, whenever larger than SPI Control Threshold amount of space available in SPI Control Buffer."]
pub type ThresR = crate::FieldReader;
#[doc = "Field `THRES` writer - SPI Control Threshold Value. SPI Control Threshold Interrupt is asserted, whenever larger than SPI Control Threshold amount of space available in SPI Control Buffer."]
pub type ThresW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - SPI Control Threshold Value. SPI Control Threshold Interrupt is asserted, whenever larger than SPI Control Threshold amount of space available in SPI Control Buffer."]
    #[inline(always)]
    pub fn thres(&self) -> ThresR {
        ThresR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Control Threshold Value. SPI Control Threshold Interrupt is asserted, whenever larger than SPI Control Threshold amount of space available in SPI Control Buffer."]
    #[inline(always)]
    pub fn thres(&mut self) -> ThresW<'_, ThrSpec> {
        ThresW::new(self, 0)
    }
}
#[doc = "SPI Control threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`thr::R`](R) reader structure"]
impl crate::Readable for ThrSpec {}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {}
