#[doc = "Register `UOWP` reader"]
pub type R = crate::R<UowpSpec>;
#[doc = "Register `UOWP` writer"]
pub type W = crate::W<UowpSpec>;
#[doc = "Field `UO1WP` reader - User OTP Page 1 Write Protect Row Bit"]
pub type Uo1wpR = crate::FieldReader;
#[doc = "Field `UO1WP` writer - User OTP Page 1 Write Protect Row Bit"]
pub type Uo1wpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UO2WP` reader - User OTP Page 2 Write Protect Row Bit"]
pub type Uo2wpR = crate::FieldReader;
#[doc = "Field `UO2WP` writer - User OTP Page 2 Write Protect Row Bit"]
pub type Uo2wpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UO1WPRLOCK` reader - User OTP Page 1 WP Row Lock Bit"]
pub type Uo1wprlockR = crate::FieldReader;
#[doc = "Field `UO1WPRLOCK` writer - User OTP Page 1 WP Row Lock Bit"]
pub type Uo1wprlockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UO2WPRLOCK` reader - User OTP Page 2 WP Row Lock Bit"]
pub type Uo2wprlockR = crate::FieldReader;
#[doc = "Field `UO2WPRLOCK` writer - User OTP Page 2 WP Row Lock Bit"]
pub type Uo2wprlockW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - User OTP Page 1 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo1wp(&self) -> Uo1wpR {
        Uo1wpR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - User OTP Page 2 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo2wp(&self) -> Uo2wpR {
        Uo2wpR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - User OTP Page 1 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo1wprlock(&self) -> Uo1wprlockR {
        Uo1wprlockR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - User OTP Page 2 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo2wprlock(&self) -> Uo2wprlockR {
        Uo2wprlockR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - User OTP Page 1 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo1wp(&mut self) -> Uo1wpW<'_, UowpSpec> {
        Uo1wpW::new(self, 0)
    }
    #[doc = "Bits 8:11 - User OTP Page 2 Write Protect Row Bit"]
    #[inline(always)]
    pub fn uo2wp(&mut self) -> Uo2wpW<'_, UowpSpec> {
        Uo2wpW::new(self, 8)
    }
    #[doc = "Bits 16:19 - User OTP Page 1 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo1wprlock(&mut self) -> Uo1wprlockW<'_, UowpSpec> {
        Uo1wprlockW::new(self, 16)
    }
    #[doc = "Bits 24:27 - User OTP Page 2 WP Row Lock Bit"]
    #[inline(always)]
    pub fn uo2wprlock(&mut self) -> Uo2wprlockW<'_, UowpSpec> {
        Uo2wprlockW::new(self, 24)
    }
}
#[doc = "User OTP Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`uowp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uowp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UowpSpec;
impl crate::RegisterSpec for UowpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uowp::R`](R) reader structure"]
impl crate::Readable for UowpSpec {}
#[doc = "`write(|w| ..)` method takes [`uowp::W`](W) writer structure"]
impl crate::Writable for UowpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UOWP to value 0x0f0f"]
impl crate::Resettable for UowpSpec {
    const RESET_VALUE: u32 = 0x0f0f;
}
