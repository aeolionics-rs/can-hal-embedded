#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Start Event conversion input enable"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Event conversion input enable"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTINV` reader - Start Conversion Invert"]
pub type StartinvR = crate::BitReader;
#[doc = "Field `STARTINV` writer - Start Conversion Invert"]
pub type StartinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESRDYEO` reader - Result Ready Event Out"]
pub type ResrdyeoR = crate::BitReader;
#[doc = "Field `RESRDYEO` writer - Result Ready Event Out"]
pub type ResrdyeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPEO` reader - Comparator Window Event Out"]
pub type CmpeoR = crate::BitReader;
#[doc = "Field `CMPEO` writer - Comparator Window Event Out"]
pub type CmpeoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Event conversion input enable"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Start Conversion Invert"]
    #[inline(always)]
    pub fn startinv(&self) -> StartinvR {
        StartinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&self) -> ResrdyeoR {
        ResrdyeoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Comparator Window Event Out"]
    #[inline(always)]
    pub fn cmpeo(&self) -> CmpeoR {
        CmpeoR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Event conversion input enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> StarteiW<'_, EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 3 - Start Conversion Invert"]
    #[inline(always)]
    pub fn startinv(&mut self) -> StartinvW<'_, EvctrlSpec> {
        StartinvW::new(self, 3)
    }
    #[doc = "Bit 4 - Result Ready Event Out"]
    #[inline(always)]
    pub fn resrdyeo(&mut self) -> ResrdyeoW<'_, EvctrlSpec> {
        ResrdyeoW::new(self, 4)
    }
    #[doc = "Bit 5 - Comparator Window Event Out"]
    #[inline(always)]
    pub fn cmpeo(&mut self) -> CmpeoW<'_, EvctrlSpec> {
        CmpeoW::new(self, 5)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
