#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUSY` reader - HSM CPU BUSY"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - HSM CPU BUSY"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKUP` reader - HSM CPU LOCKUP"]
pub type LkupR = crate::BitReader;
#[doc = "Field `LKUP` writer - HSM CPU LOCKUP"]
pub type LkupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - PROCESSING STATE"]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - PROCESSING STATE"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LCS` reader - LIFECYCLE STATE"]
pub type LcsR = crate::FieldReader;
#[doc = "Field `LCS` writer - LIFECYCLE STATE"]
pub type LcsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SBS` reader - SECURE BOOT STATE"]
pub type SbsR = crate::FieldReader;
#[doc = "Field `SBS` writer - SECURE BOOT STATE"]
pub type SbsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ECODE` reader - ERROR CODE"]
pub type EcodeR = crate::FieldReader;
#[doc = "Field `ECODE` writer - ERROR CODE"]
pub type EcodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - HSM CPU BUSY"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSM CPU LOCKUP"]
    #[inline(always)]
    pub fn lkup(&self) -> LkupR {
        LkupR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - PROCESSING STATE"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - LIFECYCLE STATE"]
    #[inline(always)]
    pub fn lcs(&self) -> LcsR {
        LcsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SECURE BOOT STATE"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - ERROR CODE"]
    #[inline(always)]
    pub fn ecode(&self) -> EcodeR {
        EcodeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - HSM CPU BUSY"]
    #[inline(always)]
    pub fn busy(&mut self) -> BusyW<'_, StatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - HSM CPU LOCKUP"]
    #[inline(always)]
    pub fn lkup(&mut self) -> LkupW<'_, StatusSpec> {
        LkupW::new(self, 1)
    }
    #[doc = "Bits 4:6 - PROCESSING STATE"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, StatusSpec> {
        PsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - LIFECYCLE STATE"]
    #[inline(always)]
    pub fn lcs(&mut self) -> LcsW<'_, StatusSpec> {
        LcsW::new(self, 8)
    }
    #[doc = "Bits 12:14 - SECURE BOOT STATE"]
    #[inline(always)]
    pub fn sbs(&mut self) -> SbsW<'_, StatusSpec> {
        SbsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - ERROR CODE"]
    #[inline(always)]
    pub fn ecode(&mut self) -> EcodeW<'_, StatusSpec> {
        EcodeW::new(self, 16)
    }
}
#[doc = "STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
