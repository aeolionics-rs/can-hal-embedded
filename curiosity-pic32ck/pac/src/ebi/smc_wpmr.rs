#[doc = "Register `SMC_WPMR` reader"]
pub type R = crate::R<SmcWpmrSpec>;
#[doc = "Register `SMC_WPMR` writer"]
pub type W = crate::W<SmcWpmrSpec>;
#[doc = "Field `WPEN` reader - Write Protect Enable"]
pub type WpenR = crate::BitReader;
#[doc = "Field `WPEN` writer - Write Protect Enable"]
pub type WpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Wpkeyselect {
    #[doc = "5459267: Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    Passwd = 5459267,
}
impl From<Wpkeyselect> for u32 {
    #[inline(always)]
    fn from(variant: Wpkeyselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wpkeyselect {
    type Ux = u32;
}
impl crate::IsEnum for Wpkeyselect {}
#[doc = "Field `WPKEY` reader - Write Protection Key"]
pub type WpkeyR = crate::FieldReader<Wpkeyselect>;
impl WpkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wpkeyselect> {
        match self.bits {
            5459267 => Some(Wpkeyselect::Passwd),
            _ => None,
        }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == Wpkeyselect::Passwd
    }
}
#[doc = "Field `WPKEY` writer - Write Protection Key"]
pub type WpkeyW<'a, REG> = crate::FieldWriter<'a, REG, 24, Wpkeyselect>;
impl<'a, REG> WpkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Writing any other value in this field aborts the write operation of the WPEN bit. Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut crate::W<REG> {
        self.variant(Wpkeyselect::Passwd)
    }
}
impl R {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WpenR {
        WpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&self) -> WpkeyR {
        WpkeyR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protect Enable"]
    #[inline(always)]
    pub fn wpen(&mut self) -> WpenW<'_, SmcWpmrSpec> {
        WpenW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WpkeyW<'_, SmcWpmrSpec> {
        WpkeyW::new(self, 8)
    }
}
#[doc = "SMC Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`smc_wpmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smc_wpmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcWpmrSpec;
impl crate::RegisterSpec for SmcWpmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smc_wpmr::R`](R) reader structure"]
impl crate::Readable for SmcWpmrSpec {}
#[doc = "`write(|w| ..)` method takes [`smc_wpmr::W`](W) writer structure"]
impl crate::Writable for SmcWpmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMC_WPMR to value 0"]
impl crate::Resettable for SmcWpmrSpec {}
