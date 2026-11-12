#[doc = "Register `CHINTENCLR` reader"]
pub type R = crate::R<ChintenclrSpec>;
#[doc = "Register `CHINTENCLR` writer"]
pub type W = crate::W<ChintenclrSpec>;
#[doc = "Field `SD` reader - Channel Event Output Enable"]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - Channel Event Output Enable"]
pub type SdW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TA` reader - Clear Transfer Abort Interrupt Enable"]
pub type TaR = crate::BitReader;
#[doc = "Field `TA` writer - Clear Transfer Abort Interrupt Enable"]
pub type TaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CC` reader - Clear Cell Transfer Complete Interrupt Enable"]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - Clear Cell Transfer Complete Interrupt Enable"]
pub type CcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BC` reader - Clear Cell Transfer Complete Interrupt Enable"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - Clear Cell Transfer Complete Interrupt Enable"]
pub type BcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `BH` reader - Clear Block Transfer Half Complete Interrupt Enable"]
pub type BhR = crate::BitReader;
#[doc = "Field `BH` writer - Clear Block Transfer Half Complete Interrupt Enable"]
pub type BhW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LL` reader - Clear Linked List Done Interrupt Enable"]
pub type LlR = crate::BitReader;
#[doc = "Field `LL` writer - Clear Linked List Done Interrupt Enable"]
pub type LlW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Block Transfer Half Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bh(&self) -> BhR {
        BhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear Linked List Done Interrupt Enable"]
    #[inline(always)]
    pub fn ll(&self) -> LlR {
        LlR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, ChintenclrSpec> {
        SdW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn ta(&mut self) -> TaW<'_, ChintenclrSpec> {
        TaW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, ChintenclrSpec> {
        CcW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, ChintenclrSpec> {
        BcW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear Block Transfer Half Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bh(&mut self) -> BhW<'_, ChintenclrSpec> {
        BhW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear Linked List Done Interrupt Enable"]
    #[inline(always)]
    pub fn ll(&mut self) -> LlW<'_, ChintenclrSpec> {
        LlW::new(self, 5)
    }
}
#[doc = "CHANNEL INTERRUPT ENABLE CLEAR REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintenclrSpec;
impl crate::RegisterSpec for ChintenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chintenclr::R`](R) reader structure"]
impl crate::Readable for ChintenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`chintenclr::W`](W) writer structure"]
impl crate::Writable for ChintenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets CHINTENCLR to value 0"]
impl crate::Resettable for ChintenclrSpec {}
