#[doc = "Register `PWP[%s]` reader"]
pub type R = crate::R<PwpSpec>;
#[doc = "Register `PWP[%s]` writer"]
pub type W = crate::W<PwpSpec>;
#[doc = "Field `PWPSIZE` reader - PWP Region 0 Size in 4KB pages"]
pub type PwpsizeR = crate::FieldReader<u16>;
#[doc = "Field `PWPSIZE` writer - PWP Region 0 Size in 4KB pages"]
pub type PwpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PWPMIR` reader - Mirror PWP 0 bit"]
pub type PwpmirR = crate::BitReader;
#[doc = "Field `PWPMIR` writer - Mirror PWP 0 bit"]
pub type PwpmirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWPLOCK` reader - PWP Region 0 Lock Bit"]
pub type PwplockR = crate::BitReader;
#[doc = "Field `PWPLOCK` writer - PWP Region 0 Lock Bit"]
pub type PwplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWPEN` reader - PWP Region 0 Enable Bit"]
pub type PwpenR = crate::BitReader;
#[doc = "Field `PWPEN` writer - PWP Region 0 Enable Bit"]
pub type PwpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWPBASE` reader - PWP Region 0 Base Address - 4KB Page Aligned"]
pub type PwpbaseR = crate::FieldReader<u16>;
#[doc = "Field `PWPBASE` writer - PWP Region 0 Base Address - 4KB Page Aligned"]
pub type PwpbaseW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - PWP Region 0 Size in 4KB pages"]
    #[inline(always)]
    pub fn pwpsize(&self) -> PwpsizeR {
        PwpsizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 13 - Mirror PWP 0 bit"]
    #[inline(always)]
    pub fn pwpmir(&self) -> PwpmirR {
        PwpmirR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWP Region 0 Lock Bit"]
    #[inline(always)]
    pub fn pwplock(&self) -> PwplockR {
        PwplockR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PWP Region 0 Enable Bit"]
    #[inline(always)]
    pub fn pwpen(&self) -> PwpenR {
        PwpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - PWP Region 0 Base Address - 4KB Page Aligned"]
    #[inline(always)]
    pub fn pwpbase(&self) -> PwpbaseR {
        PwpbaseR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - PWP Region 0 Size in 4KB pages"]
    #[inline(always)]
    pub fn pwpsize(&mut self) -> PwpsizeW<'_, PwpSpec> {
        PwpsizeW::new(self, 0)
    }
    #[doc = "Bit 13 - Mirror PWP 0 bit"]
    #[inline(always)]
    pub fn pwpmir(&mut self) -> PwpmirW<'_, PwpSpec> {
        PwpmirW::new(self, 13)
    }
    #[doc = "Bit 14 - PWP Region 0 Lock Bit"]
    #[inline(always)]
    pub fn pwplock(&mut self) -> PwplockW<'_, PwpSpec> {
        PwplockW::new(self, 14)
    }
    #[doc = "Bit 15 - PWP Region 0 Enable Bit"]
    #[inline(always)]
    pub fn pwpen(&mut self) -> PwpenW<'_, PwpSpec> {
        PwpenW::new(self, 15)
    }
    #[doc = "Bits 16:27 - PWP Region 0 Base Address - 4KB Page Aligned"]
    #[inline(always)]
    pub fn pwpbase(&mut self) -> PwpbaseW<'_, PwpSpec> {
        PwpbaseW::new(self, 16)
    }
}
#[doc = "PFM Write Protect Region 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwpSpec;
impl crate::RegisterSpec for PwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwp::R`](R) reader structure"]
impl crate::Readable for PwpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwp::W`](W) writer structure"]
impl crate::Writable for PwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWP[%s] to value 0"]
impl crate::Resettable for PwpSpec {}
