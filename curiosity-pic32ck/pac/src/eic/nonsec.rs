#[doc = "Register `NONSEC` reader"]
pub type R = crate::R<NonsecSpec>;
#[doc = "Register `NONSEC` writer"]
pub type W = crate::W<NonsecSpec>;
#[doc = "Field `EXTINT` reader - External Interrupt Nonsecure Enable"]
pub type ExtintR = crate::FieldReader<u16>;
#[doc = "Field `EXTINT` writer - External Interrupt Nonsecure Enable"]
pub type ExtintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NMI` reader - Non-Maskable Interrupt Nonsecure Enable"]
pub type NmiR = crate::BitReader;
#[doc = "Field `NMI` writer - Non-Maskable Interrupt Nonsecure Enable"]
pub type NmiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn extint(&self) -> ExtintR {
        ExtintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Non-Maskable Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn nmi(&self) -> NmiR {
        NmiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> ExtintW<'_, NonsecSpec> {
        ExtintW::new(self, 0)
    }
    #[doc = "Bit 31 - Non-Maskable Interrupt Nonsecure Enable"]
    #[inline(always)]
    pub fn nmi(&mut self) -> NmiW<'_, NonsecSpec> {
        NmiW::new(self, 31)
    }
}
#[doc = "Non-secure Interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nonsec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nonsec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NonsecSpec;
impl crate::RegisterSpec for NonsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nonsec::R`](R) reader structure"]
impl crate::Readable for NonsecSpec {}
#[doc = "`write(|w| ..)` method takes [`nonsec::W`](W) writer structure"]
impl crate::Writable for NonsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NONSEC to value 0"]
impl crate::Resettable for NonsecSpec {}
