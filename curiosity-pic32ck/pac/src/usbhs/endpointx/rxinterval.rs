#[doc = "Register `RXINTERVAL` reader"]
pub type R = crate::R<RxintervalSpec>;
#[doc = "Register `RXINTERVAL` writer"]
pub type W = crate::W<RxintervalSpec>;
#[doc = "Field `RXPOLLINGINTERVAL` reader - polling interval"]
pub type RxpollingintervalR = crate::FieldReader;
#[doc = "Field `RXPOLLINGINTERVAL` writer - polling interval"]
pub type RxpollingintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - polling interval"]
    #[inline(always)]
    pub fn rxpollinginterval(&self) -> RxpollingintervalR {
        RxpollingintervalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - polling interval"]
    #[inline(always)]
    pub fn rxpollinginterval(&mut self) -> RxpollingintervalW<'_, RxintervalSpec> {
        RxpollingintervalW::new(self, 0)
    }
}
#[doc = "ENDPOINTX Host Mode Only: Defines The Polling Interval For The Currently Selected RX Endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxinterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxinterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxintervalSpec;
impl crate::RegisterSpec for RxintervalSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxinterval::R`](R) reader structure"]
impl crate::Readable for RxintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`rxinterval::W`](W) writer structure"]
impl crate::Writable for RxintervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXINTERVAL to value 0"]
impl crate::Resettable for RxintervalSpec {}
