#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `EXTINT` reader - External Interrupt Enable"]
pub type ExtintR = crate::FieldReader<u16>;
#[doc = "Field `EXTINT` writer - External Interrupt Enable"]
pub type ExtintW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `NSCHK` reader - Non-secure Check Interrupt Enable"]
pub type NschkR = crate::BitReader;
#[doc = "Field `NSCHK` writer - Non-secure Check Interrupt Enable"]
pub type NschkW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - External Interrupt Enable"]
    #[inline(always)]
    pub fn extint(&self) -> ExtintR {
        ExtintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&self) -> NschkR {
        NschkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - External Interrupt Enable"]
    #[inline(always)]
    pub fn extint(&mut self) -> ExtintW<'_, IntenclrSpec> {
        ExtintW::new(self, 0)
    }
    #[doc = "Bit 31 - Non-secure Check Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NschkW<'_, IntenclrSpec> {
        NschkW::new(self, 31)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_ffff;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
