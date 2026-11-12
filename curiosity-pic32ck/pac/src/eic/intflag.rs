#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `EXTINT` reader - External Interrupt"]
pub type ExtintR = crate::FieldReader<u16>;
#[doc = "Field `EXTINT` writer - External Interrupt"]
pub type ExtintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NSCHK` reader - Non-secure Check Interrupt"]
pub type NschkR = crate::BitReader;
#[doc = "Field `NSCHK` writer - Non-secure Check Interrupt"]
pub type NschkW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt"]
    #[inline(always)]
    pub fn extint(&self) -> ExtintR {
        ExtintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt"]
    #[inline(always)]
    pub fn nschk(&self) -> NschkR {
        NschkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt"]
    #[inline(always)]
    pub fn extint(&mut self) -> ExtintW<'_, IntflagSpec> {
        ExtintW::new(self, 0)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NschkW<'_, IntflagSpec> {
        NschkW::new(self, 31)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_ffff;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
