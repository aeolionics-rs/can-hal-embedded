#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Start Measurement Event Input Enable"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Measurement Event Input Enable"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTINV` reader - Start Measurement Event Invert Enable"]
pub type StartinvR = crate::BitReader;
#[doc = "Field `STARTINV` writer - Start Measurement Event Invert Enable"]
pub type StartinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DONEEO` reader - Measurement Done Event Out"]
pub type DoneeoR = crate::BitReader;
#[doc = "Field `DONEEO` writer - Measurement Done Event Out"]
pub type DoneeoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINMONEO` reader - Window Monitor Event Out"]
pub type WinmoneoR = crate::BitReader;
#[doc = "Field `WINMONEO` writer - Window Monitor Event Out"]
pub type WinmoneoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Measurement Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Measurement Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&self) -> StartinvR {
        StartinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Measurement Done Event Out"]
    #[inline(always)]
    pub fn doneeo(&self) -> DoneeoR {
        DoneeoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&self) -> WinmoneoR {
        WinmoneoR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Measurement Event Input Enable"]
    #[inline(always)]
    pub fn startei(&mut self) -> StarteiW<'_, EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Measurement Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&mut self) -> StartinvW<'_, EvctrlSpec> {
        StartinvW::new(self, 1)
    }
    #[doc = "Bit 4 - Measurement Done Event Out"]
    #[inline(always)]
    pub fn doneeo(&mut self) -> DoneeoW<'_, EvctrlSpec> {
        DoneeoW::new(self, 4)
    }
    #[doc = "Bit 5 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn winmoneo(&mut self) -> WinmoneoW<'_, EvctrlSpec> {
        WinmoneoW::new(self, 5)
    }
}
#[doc = "Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
