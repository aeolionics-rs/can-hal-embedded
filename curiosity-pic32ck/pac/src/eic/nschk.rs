#[doc = "Register `NSCHK` reader"]
pub type R = crate::R<NschkSpec>;
#[doc = "Register `NSCHK` writer"]
pub type W = crate::W<NschkSpec>;
#[doc = "Field `EXTINT` reader - External Interrupt Nonsecure Check Enable"]
pub type ExtintR = crate::FieldReader<u16>;
#[doc = "Field `EXTINT` writer - External Interrupt Nonsecure Check Enable"]
pub type ExtintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NMI` reader - Non-Maskable External Interrupt Nonsecure Check Enable"]
pub type NmiR = crate::BitReader;
#[doc = "Field `NMI` writer - Non-Maskable External Interrupt Nonsecure Check Enable"]
pub type NmiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn extint(&self) -> ExtintR {
        ExtintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn nmi(&self) -> NmiR {
        NmiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> ExtintW<'_, NschkSpec> {
        ExtintW::new(self, 0)
    }
    #[doc = "Bit 31 - Non-Maskable External Interrupt Nonsecure Check Enable"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NmiW<'_, NschkSpec> {
        NmiW::new(self, 31)
    }
}
#[doc = "Non-secure Interrupt Check Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`nschk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nschk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NschkSpec;
impl crate::RegisterSpec for NschkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nschk::R`](R) reader structure"]
impl crate::Readable for NschkSpec {}
#[doc = "`write(|w| ..)` method takes [`nschk::W`](W) writer structure"]
impl crate::Writable for NschkSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NSCHK to value 0"]
impl crate::Resettable for NschkSpec {}
