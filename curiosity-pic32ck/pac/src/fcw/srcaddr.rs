#[doc = "Register `SRCADDR` reader"]
pub type R = crate::R<SrcaddrSpec>;
#[doc = "Register `SRCADDR` writer"]
pub type W = crate::W<SrcaddrSpec>;
#[doc = "Field `SRCADDR` reader - Source Data (Word) Address"]
pub type SrcaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data (Word) Address"]
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - Source Data (Word) Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - Source Data (Word) Address"]
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<'_, SrcaddrSpec> {
        SrcaddrW::new(self, 2)
    }
}
#[doc = "Source Data Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrcaddrSpec;
impl crate::RegisterSpec for SrcaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srcaddr::R`](R) reader structure"]
impl crate::Readable for SrcaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`srcaddr::W`](W) writer structure"]
impl crate::Writable for SrcaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRCADDR to value 0"]
impl crate::Resettable for SrcaddrSpec {}
