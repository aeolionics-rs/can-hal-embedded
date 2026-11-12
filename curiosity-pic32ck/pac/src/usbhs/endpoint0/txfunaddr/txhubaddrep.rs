#[doc = "Register `TXHUBADDREP` reader"]
pub type R = crate::R<TxhubaddrepSpec>;
#[doc = "Register `TXHUBADDREP` writer"]
pub type W = crate::W<TxhubaddrepSpec>;
#[doc = "Field `TXHUBADDR` reader - hub address"]
pub type TxhubaddrR = crate::FieldReader;
#[doc = "Field `TXHUBADDR` writer - hub address"]
pub type TxhubaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MULTRANS` reader - hub has multiple or single translator"]
pub type MultransR = crate::BitReader;
#[doc = "Field `MULTRANS` writer - hub has multiple or single translator"]
pub type MultransW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - hub address"]
    #[inline(always)]
    pub fn txhubaddr(&self) -> TxhubaddrR {
        TxhubaddrR::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - hub has multiple or single translator"]
    #[inline(always)]
    pub fn multrans(&self) -> MultransR {
        MultransR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - hub address"]
    #[inline(always)]
    pub fn txhubaddr(&mut self) -> TxhubaddrW<'_, TxhubaddrepSpec> {
        TxhubaddrW::new(self, 0)
    }
    #[doc = "Bit 7 - hub has multiple or single translator"]
    #[inline(always)]
    pub fn multrans(&mut self) -> MultransW<'_, TxhubaddrepSpec> {
        MultransW::new(self, 7)
    }
}
#[doc = "Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`txhubaddrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txhubaddrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxhubaddrepSpec;
impl crate::RegisterSpec for TxhubaddrepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txhubaddrep::R`](R) reader structure"]
impl crate::Readable for TxhubaddrepSpec {}
#[doc = "`write(|w| ..)` method takes [`txhubaddrep::W`](W) writer structure"]
impl crate::Writable for TxhubaddrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXHUBADDREP to value 0"]
impl crate::Resettable for TxhubaddrepSpec {}
