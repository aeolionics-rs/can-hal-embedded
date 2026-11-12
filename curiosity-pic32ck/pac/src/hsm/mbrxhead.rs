#[doc = "Register `MBRXHEAD` reader"]
pub type R = crate::R<MbrxheadSpec>;
#[doc = "Register `MBRXHEAD` writer"]
pub type W = crate::W<MbrxheadSpec>;
#[doc = "Field `RXHEAD` reader - MAILBOX RECEIVE HEADER REGISTER"]
pub type RxheadR = crate::FieldReader<u32>;
#[doc = "Field `RXHEAD` writer - MAILBOX RECEIVE HEADER REGISTER"]
pub type RxheadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAILBOX RECEIVE HEADER REGISTER"]
    #[inline(always)]
    pub fn rxhead(&self) -> RxheadR {
        RxheadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAILBOX RECEIVE HEADER REGISTER"]
    #[inline(always)]
    pub fn rxhead(&mut self) -> RxheadW<'_, MbrxheadSpec> {
        RxheadW::new(self, 0)
    }
}
#[doc = "MAILBOX RECEIVE HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct MbrxheadSpec;
impl crate::RegisterSpec for MbrxheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbrxhead::R`](R) reader structure"]
impl crate::Readable for MbrxheadSpec {}
#[doc = "`write(|w| ..)` method takes [`mbrxhead::W`](W) writer structure"]
impl crate::Writable for MbrxheadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBRXHEAD to value 0"]
impl crate::Resettable for MbrxheadSpec {}
