#[doc = "Register `TXFUNCADDREP` reader"]
pub type R = crate::R<TxfuncaddrepSpec>;
#[doc = "Register `TXFUNCADDREP` writer"]
pub type W = crate::W<TxfuncaddrepSpec>;
#[doc = "Field `TXFUNCADDR` reader - address of the target function"]
pub type TxfuncaddrR = crate::FieldReader;
#[doc = "Field `TXFUNCADDR` writer - address of the target function"]
pub type TxfuncaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - address of the target function"]
    #[inline(always)]
    pub fn txfuncaddr(&self) -> TxfuncaddrR {
        TxfuncaddrR::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - address of the target function"]
    #[inline(always)]
    pub fn txfuncaddr(&mut self) -> TxfuncaddrW<'_, TxfuncaddrepSpec> {
        TxfuncaddrW::new(self, 0)
    }
}
#[doc = "Relevant In Host Mode Only. Address Of The Target Function.\n\nYou can [`read`](crate::Reg::read) this register and get [`txfuncaddrep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfuncaddrep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfuncaddrepSpec;
impl crate::RegisterSpec for TxfuncaddrepSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`txfuncaddrep::R`](R) reader structure"]
impl crate::Readable for TxfuncaddrepSpec {}
#[doc = "`write(|w| ..)` method takes [`txfuncaddrep::W`](W) writer structure"]
impl crate::Writable for TxfuncaddrepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFUNCADDREP to value 0"]
impl crate::Resettable for TxfuncaddrepSpec {}
