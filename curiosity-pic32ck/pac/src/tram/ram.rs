#[doc = "Register `RAM[%s]` reader"]
pub type R = crate::R<RamSpec>;
#[doc = "Register `RAM[%s]` writer"]
pub type W = crate::W<RamSpec>;
#[doc = "Field `DATA` reader - Trust RAM Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Trust RAM Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Trust RAM Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, RamSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "TrustRAM\n\nYou can [`read`](crate::Reg::read) this register and get [`ram::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ram::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RamSpec;
impl crate::RegisterSpec for RamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ram::R`](R) reader structure"]
impl crate::Readable for RamSpec {}
#[doc = "`write(|w| ..)` method takes [`ram::W`](W) writer structure"]
impl crate::Writable for RamSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RAM[%s] to value 0"]
impl crate::Resettable for RamSpec {}
