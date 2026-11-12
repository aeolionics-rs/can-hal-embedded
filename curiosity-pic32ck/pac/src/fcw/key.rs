#[doc = "Register `KEY` reader"]
pub type R = crate::R<KeySpec>;
#[doc = "Register `KEY` writer"]
pub type W = crate::W<KeySpec>;
#[doc = "Field `KEYVALUE` reader - NVM SFR Key Value"]
pub type KeyvalueR = crate::FieldReader;
#[doc = "Field `KEYVALUE` writer - NVM SFR Key Value"]
pub type KeyvalueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `KEYCODE` reader - NVM SFR Key Code"]
pub type KeycodeR = crate::FieldReader<u32>;
#[doc = "Field `KEYCODE` writer - NVM SFR Key Code"]
pub type KeycodeW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - NVM SFR Key Value"]
    #[inline(always)]
    pub fn keyvalue(&self) -> KeyvalueR {
        KeyvalueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - NVM SFR Key Code"]
    #[inline(always)]
    pub fn keycode(&self) -> KeycodeR {
        KeycodeR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - NVM SFR Key Value"]
    #[inline(always)]
    pub fn keyvalue(&mut self) -> KeyvalueW<'_, KeySpec> {
        KeyvalueW::new(self, 0)
    }
    #[doc = "Bits 8:31 - NVM SFR Key Code"]
    #[inline(always)]
    pub fn keycode(&mut self) -> KeycodeW<'_, KeySpec> {
        KeycodeW::new(self, 8)
    }
}
#[doc = "SFR Unlock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`key::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeySpec;
impl crate::RegisterSpec for KeySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key::R`](R) reader structure"]
impl crate::Readable for KeySpec {}
#[doc = "`write(|w| ..)` method takes [`key::W`](W) writer structure"]
impl crate::Writable for KeySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KeySpec {}
