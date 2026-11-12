#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `NSCHK` reader - Non-Secure Check Interrupt Enable"]
pub type NschkR = crate::BitReader;
#[doc = "Field `NSCHK` writer - Non-Secure Check Interrupt Enable"]
pub type NschkW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Non-Secure Check Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&self) -> NschkR {
        NschkR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Non-Secure Check Interrupt Enable"]
    #[inline(always)]
    pub fn nschk(&mut self) -> NschkW<'_, IntenclrSpec> {
        NschkW::new(self, 0)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
