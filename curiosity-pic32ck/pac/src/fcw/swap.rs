#[doc = "Register `SWAP` reader"]
pub type R = crate::R<SwapSpec>;
#[doc = "Register `SWAP` writer"]
pub type W = crate::W<SwapSpec>;
#[doc = "Field `BFSWAP` reader - BFM Swap Status/Control Bit"]
pub type BfswapR = crate::BitReader;
#[doc = "Field `BFSWAP` writer - BFM Swap Status/Control Bit"]
pub type BfswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFSLOCK` reader - BFM Swap Lock Bit"]
pub type BfslockR = crate::BitReader;
#[doc = "Field `BFSLOCK` writer - BFM Swap Lock Bit"]
pub type BfslockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFSWAP` reader - PFM Swap Status/Control Bit"]
pub type PfswapR = crate::BitReader;
#[doc = "Field `PFSWAP` writer - PFM Swap Status/Control Bit"]
pub type PfswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFSLOCK` reader - PFM Swap Lock Bit"]
pub type PfslockR = crate::BitReader;
#[doc = "Field `PFSLOCK` writer - PFM Swap Lock Bit"]
pub type PfslockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn bfswap(&self) -> BfswapR {
        BfswapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BFM Swap Lock Bit"]
    #[inline(always)]
    pub fn bfslock(&self) -> BfslockR {
        BfslockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - PFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn pfswap(&self) -> PfswapR {
        PfswapR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PFM Swap Lock Bit"]
    #[inline(always)]
    pub fn pfslock(&self) -> PfslockR {
        PfslockR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn bfswap(&mut self) -> BfswapW<'_, SwapSpec> {
        BfswapW::new(self, 0)
    }
    #[doc = "Bit 1 - BFM Swap Lock Bit"]
    #[inline(always)]
    pub fn bfslock(&mut self) -> BfslockW<'_, SwapSpec> {
        BfslockW::new(self, 1)
    }
    #[doc = "Bit 8 - PFM Swap Status/Control Bit"]
    #[inline(always)]
    pub fn pfswap(&mut self) -> PfswapW<'_, SwapSpec> {
        PfswapW::new(self, 8)
    }
    #[doc = "Bit 9 - PFM Swap Lock Bit"]
    #[inline(always)]
    pub fn pfslock(&mut self) -> PfslockW<'_, SwapSpec> {
        PfslockW::new(self, 9)
    }
}
#[doc = "NVM Panel Swap Register\n\nYou can [`read`](crate::Reg::read) this register and get [`swap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwapSpec;
impl crate::RegisterSpec for SwapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swap::R`](R) reader structure"]
impl crate::Readable for SwapSpec {}
#[doc = "`write(|w| ..)` method takes [`swap::W`](W) writer structure"]
impl crate::Writable for SwapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWAP to value 0"]
impl crate::Resettable for SwapSpec {}
