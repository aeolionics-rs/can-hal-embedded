#[doc = "Register `CHINTENSET` reader"]
pub type R = crate::R<ChintensetSpec>;
#[doc = "Register `CHINTENSET` writer"]
pub type W = crate::W<ChintensetSpec>;
#[doc = "Field `SD` reader - Channel Event Output Enable"]
pub type SdR = crate::BitReader;
#[doc = "Field `SD` writer - Channel Event Output Enable"]
pub type SdW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TA` reader - Set Transfer Abort Interrupt Enable"]
pub type TaR = crate::BitReader;
#[doc = "Field `TA` writer - Set Transfer Abort Interrupt Enable"]
pub type TaW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `CC` reader - set Cell Transfer Complete Interrupt Enable"]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - set Cell Transfer Complete Interrupt Enable"]
pub type CcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BC` reader - set Cell Transfer Complete Interrupt Enable"]
pub type BcR = crate::BitReader;
#[doc = "Field `BC` writer - set Cell Transfer Complete Interrupt Enable"]
pub type BcW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `BH` reader - set Block Transfer Half Complete Interrupt Enable"]
pub type BhR = crate::BitReader;
#[doc = "Field `BH` writer - set Block Transfer Half Complete Interrupt Enable"]
pub type BhW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `LL` reader - set Linked List Done Interrupt Enable"]
pub type LlR = crate::BitReader;
#[doc = "Field `LL` writer - set Linked List Done Interrupt Enable"]
pub type LlW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn sd(&self) -> SdR {
        SdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn ta(&self) -> TaR {
        TaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - set Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - set Block Transfer Half Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bh(&self) -> BhR {
        BhR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - set Linked List Done Interrupt Enable"]
    #[inline(always)]
    pub fn ll(&self) -> LlR {
        LlR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Event Output Enable"]
    #[inline(always)]
    pub fn sd(&mut self) -> SdW<'_, ChintensetSpec> {
        SdW::new(self, 0)
    }
    #[doc = "Bit 1 - Set Transfer Abort Interrupt Enable"]
    #[inline(always)]
    pub fn ta(&mut self) -> TaW<'_, ChintensetSpec> {
        TaW::new(self, 1)
    }
    #[doc = "Bit 2 - set Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, ChintensetSpec> {
        CcW::new(self, 2)
    }
    #[doc = "Bit 3 - set Cell Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, ChintensetSpec> {
        BcW::new(self, 3)
    }
    #[doc = "Bit 4 - set Block Transfer Half Complete Interrupt Enable"]
    #[inline(always)]
    pub fn bh(&mut self) -> BhW<'_, ChintensetSpec> {
        BhW::new(self, 4)
    }
    #[doc = "Bit 5 - set Linked List Done Interrupt Enable"]
    #[inline(always)]
    pub fn ll(&mut self) -> LlW<'_, ChintensetSpec> {
        LlW::new(self, 5)
    }
}
#[doc = "CHANNEL INTERRUPT ENABLE SET REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chintenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chintenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChintensetSpec;
impl crate::RegisterSpec for ChintensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chintenset::R`](R) reader structure"]
impl crate::Readable for ChintensetSpec {}
#[doc = "`write(|w| ..)` method takes [`chintenset::W`](W) writer structure"]
impl crate::Writable for ChintensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets CHINTENSET to value 0"]
impl crate::Resettable for ChintensetSpec {}
