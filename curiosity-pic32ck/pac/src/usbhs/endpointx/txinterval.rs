#[doc = "Register `TXINTERVAL` reader"]
pub type R = crate::R<TxintervalSpec>;
#[doc = "Register `TXINTERVAL` writer"]
pub type W = crate::W<TxintervalSpec>;
#[doc = "Field `TXPOLLINGINTERVAL` reader - polling interval"]
pub type TxpollingintervalR = crate::FieldReader;
#[doc = "Field `TXPOLLINGINTERVAL` writer - polling interval"]
pub type TxpollingintervalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - polling interval"]
    #[inline(always)]
    pub fn txpollinginterval(&self) -> TxpollingintervalR {
        TxpollingintervalR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - polling interval"]
    #[inline(always)]
    pub fn txpollinginterval(&mut self) -> TxpollingintervalW<'_, TxintervalSpec> {
        TxpollingintervalW::new(self, 0)
    }
}
#[doc = "ENDPOINTX Host Mode Only: Defines The Polling Interval For The Currently Selected TX Endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`txinterval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txinterval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxintervalSpec;
impl crate::RegisterSpec for TxintervalSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txinterval::R`](R) reader structure"]
impl crate::Readable for TxintervalSpec {}
#[doc = "`write(|w| ..)` method takes [`txinterval::W`](W) writer structure"]
impl crate::Writable for TxintervalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXINTERVAL to value 0"]
impl crate::Resettable for TxintervalSpec {}
