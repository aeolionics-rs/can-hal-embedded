#[doc = "Register `MBFIFO[%s]` reader"]
pub type R = crate::R<MbfifoSpec>;
#[doc = "Register `MBFIFO[%s]` writer"]
pub type W = crate::W<MbfifoSpec>;
#[doc = "Field `MBFIFO` reader - MAILBOX FIFO"]
pub type MbfifoR = crate::FieldReader<u32>;
#[doc = "Field `MBFIFO` writer - MAILBOX FIFO"]
pub type MbfifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAILBOX FIFO"]
    #[inline(always)]
    pub fn mbfifo(&self) -> MbfifoR {
        MbfifoR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAILBOX FIFO"]
    #[inline(always)]
    pub fn mbfifo(&mut self) -> MbfifoW<'_, MbfifoSpec> {
        MbfifoW::new(self, 0)
    }
}
#[doc = "MAILBOX FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`mbfifo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbfifo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">The register is <b>modified</b> in some way after a read operation.</div>"]
pub struct MbfifoSpec;
impl crate::RegisterSpec for MbfifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbfifo::R`](R) reader structure"]
impl crate::Readable for MbfifoSpec {}
#[doc = "`write(|w| ..)` method takes [`mbfifo::W`](W) writer structure"]
impl crate::Writable for MbfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBFIFO[%s] to value 0"]
impl crate::Resettable for MbfifoSpec {}
