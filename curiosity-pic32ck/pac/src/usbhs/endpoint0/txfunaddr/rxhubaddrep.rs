#[doc = "Register `RXHUBADDREP` reader"]
pub type R = crate::R<RxhubaddrepSpec>;
#[doc = "Register `RXHUBADDREP` writer"]
pub type W = crate::W<RxhubaddrepSpec>;
#[doc = "Field `RXHUBADDR` reader - hub address"]
pub type RxhubaddrR = crate::FieldReader;
#[doc = "Field `RXHUBADDR` writer - hub address"]
pub type RxhubaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `MULTRANS` reader - hub has multiple or single translator"]
pub type MultransR = crate::BitReader;
#[doc = "Field `MULTRANS` writer - hub has multiple or single translator"]
pub type MultransW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - hub address"]
    #[inline(always)]
    pub fn rxhubaddr(&self) -> RxhubaddrR {
        RxhubaddrR::new(self.bits & 0x7f)
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
    pub fn rxhubaddr(&mut self) -> RxhubaddrW<'_, RxhubaddrepSpec> {
        RxhubaddrW::new(self, 0)
    }
    #[doc = "Bit 7 - hub has multiple or single translator"]
    #[inline(always)]
    pub fn multrans(&mut self) -> MultransW<'_, RxhubaddrepSpec> {
        MultransW::new(self, 7)
    }
}
#[doc = "Relevant In Host Mode Only. HUB Address\n\nYou can [`read`](crate::Reg::read) this register and get [`rxhubaddrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxhubaddrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxhubaddrepSpec;
impl crate::RegisterSpec for RxhubaddrepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rxhubaddrep::R`](R) reader structure"]
impl crate::Readable for RxhubaddrepSpec {}
#[doc = "`write(|w| ..)` method takes [`rxhubaddrep::W`](W) writer structure"]
impl crate::Writable for RxhubaddrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXHUBADDREP to value 0"]
impl crate::Resettable for RxhubaddrepSpec {}
