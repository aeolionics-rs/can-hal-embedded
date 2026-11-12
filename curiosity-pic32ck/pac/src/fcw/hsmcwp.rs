#[doc = "Register `HSMCWP` reader"]
pub type R = crate::R<HsmcwpSpec>;
#[doc = "Register `HSMCWP` writer"]
pub type W = crate::W<HsmcwpSpec>;
#[doc = "Field `HCWP` reader - HSM CFG Page Write Protect Bits"]
pub type HcwpR = crate::FieldReader;
#[doc = "Field `HCWP` writer - HSM CFG Page Write Protect Bits"]
pub type HcwpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HCWPLOCK` reader - HSMCWP Register Lock Bit"]
pub type HcwplockR = crate::FieldReader;
#[doc = "Field `HCWPLOCK` writer - HSMCWP Register Lock Bit"]
pub type HcwplockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 12:15 - HSM CFG Page Write Protect Bits"]
    #[inline(always)]
    pub fn hcwp(&self) -> HcwpR {
        HcwpR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - HSMCWP Register Lock Bit"]
    #[inline(always)]
    pub fn hcwplock(&self) -> HcwplockR {
        HcwplockR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - HSM CFG Page Write Protect Bits"]
    #[inline(always)]
    pub fn hcwp(&mut self) -> HcwpW<'_, HsmcwpSpec> {
        HcwpW::new(self, 12)
    }
    #[doc = "Bits 28:31 - HSMCWP Register Lock Bit"]
    #[inline(always)]
    pub fn hcwplock(&mut self) -> HcwplockW<'_, HsmcwpSpec> {
        HcwplockW::new(self, 28)
    }
}
#[doc = "HSM CFG Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsmcwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsmcwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsmcwpSpec;
impl crate::RegisterSpec for HsmcwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsmcwp::R`](R) reader structure"]
impl crate::Readable for HsmcwpSpec {}
#[doc = "`write(|w| ..)` method takes [`hsmcwp::W`](W) writer structure"]
impl crate::Writable for HsmcwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSMCWP to value 0xf000"]
impl crate::Resettable for HsmcwpSpec {
    const RESET_VALUE: u32 = 0xf000;
}
