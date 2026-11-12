#[doc = "Register `CHXSIZ` reader"]
pub type R = crate::R<ChxsizSpec>;
#[doc = "Register `CHXSIZ` writer"]
pub type W = crate::W<ChxsizSpec>;
#[doc = "Field `CSZ` reader - Cell transfer size in bytes."]
pub type CszR = crate::FieldReader<u16>;
#[doc = "Field `CSZ` writer - Cell transfer size in bytes."]
pub type CszW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BLKSZ` reader - Block transfer size in bytes."]
pub type BlkszR = crate::FieldReader<u16>;
#[doc = "Field `BLKSZ` writer - Block transfer size in bytes."]
pub type BlkszW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - Cell transfer size in bytes."]
    #[inline(always)]
    pub fn csz(&self) -> CszR {
        CszR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Block transfer size in bytes."]
    #[inline(always)]
    pub fn blksz(&self) -> BlkszR {
        BlkszR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Cell transfer size in bytes."]
    #[inline(always)]
    pub fn csz(&mut self) -> CszW<'_, ChxsizSpec> {
        CszW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Block transfer size in bytes."]
    #[inline(always)]
    pub fn blksz(&mut self) -> BlkszW<'_, ChxsizSpec> {
        BlkszW::new(self, 16)
    }
}
#[doc = "CHANNEL TRANSFER SIZE REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`chxsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chxsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChxsizSpec;
impl crate::RegisterSpec for ChxsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chxsiz::R`](R) reader structure"]
impl crate::Readable for ChxsizSpec {}
#[doc = "`write(|w| ..)` method takes [`chxsiz::W`](W) writer structure"]
impl crate::Writable for ChxsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHXSIZ to value 0"]
impl crate::Resettable for ChxsizSpec {}
