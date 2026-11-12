#[doc = "Register `MBTXHEAD` reader"]
pub type R = crate::R<MbtxheadSpec>;
#[doc = "Register `MBTXHEAD` writer"]
pub type W = crate::W<MbtxheadSpec>;
#[doc = "Field `TXHEAD` reader - MAILBOX TRANSMIT HEADER REGISTER"]
pub type TxheadR = crate::FieldReader<u32>;
#[doc = "Field `TXHEAD` writer - MAILBOX TRANSMIT HEADER REGISTER"]
pub type TxheadW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAILBOX TRANSMIT HEADER REGISTER"]
    #[inline(always)]
    pub fn txhead(&self) -> TxheadR {
        TxheadR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAILBOX TRANSMIT HEADER REGISTER"]
    #[inline(always)]
    pub fn txhead(&mut self) -> TxheadW<'_, MbtxheadSpec> {
        TxheadW::new(self, 0)
    }
}
#[doc = "MAILBOX TRANSMIT HEADER\n\nYou can [`read`](crate::Reg::read) this register and get [`mbtxhead::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbtxhead::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct MbtxheadSpec;
impl crate::RegisterSpec for MbtxheadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbtxhead::R`](R) reader structure"]
impl crate::Readable for MbtxheadSpec {}
#[doc = "`write(|w| ..)` method takes [`mbtxhead::W`](W) writer structure"]
impl crate::Writable for MbtxheadSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBTXHEAD to value 0"]
impl crate::Resettable for MbtxheadSpec {}
