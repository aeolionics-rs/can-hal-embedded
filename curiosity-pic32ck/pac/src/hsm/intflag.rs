#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `ERROR` reader - Error Interrupt Flag"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Error Interrupt Flag"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPER` reader - Tamper Interrupt Flag"]
pub type TamperR = crate::BitReader;
#[doc = "Field `TAMPER` writer - Tamper Interrupt Flag"]
pub type TamperW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Error Interrupt Flag"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tamper(&self) -> TamperR {
        TamperR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error Interrupt Flag"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntflagSpec> {
        ErrorW::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper Interrupt Flag"]
    #[inline(always)]
    pub fn tamper(&mut self) -> TamperW<'_, IntflagSpec> {
        TamperW::new(self, 1)
    }
}
#[doc = "INTERRUPT FLAG\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
