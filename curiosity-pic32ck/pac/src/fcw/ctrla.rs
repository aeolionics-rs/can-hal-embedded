#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `NVMOP` reader - NVM Operation"]
pub type NvmopR = crate::FieldReader;
#[doc = "Field `NVMOP` writer - NVM Operation"]
pub type NvmopW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PREPG` reader - NVM Pre-Program Configuration Bit"]
pub type PrepgR = crate::BitReader;
#[doc = "Field `PREPG` writer - NVM Pre-Program Configuration Bit"]
pub type PrepgW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - NVM Operation"]
    #[inline(always)]
    pub fn nvmop(&self) -> NvmopR {
        NvmopR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - NVM Pre-Program Configuration Bit"]
    #[inline(always)]
    pub fn prepg(&self) -> PrepgR {
        PrepgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - NVM Operation"]
    #[inline(always)]
    pub fn nvmop(&mut self) -> NvmopW<'_, CtrlaSpec> {
        NvmopW::new(self, 0)
    }
    #[doc = "Bit 7 - NVM Pre-Program Configuration Bit"]
    #[inline(always)]
    pub fn prepg(&mut self) -> PrepgW<'_, CtrlaSpec> {
        PrepgW::new(self, 7)
    }
}
#[doc = "NVM Write Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
