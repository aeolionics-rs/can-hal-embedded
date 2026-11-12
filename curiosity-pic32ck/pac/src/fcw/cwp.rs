#[doc = "Register `CWP` reader"]
pub type R = crate::R<CwpSpec>;
#[doc = "Register `CWP` writer"]
pub type W = crate::W<CwpSpec>;
#[doc = "Field `UC1WP` reader - User Configuration Page 1 Write Protect Bit"]
pub type Uc1wpR = crate::BitReader;
#[doc = "Field `UC1WP` writer - User Configuration Page 1 Write Protect Bit"]
pub type Uc1wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC1WP` reader - Boot Configuration Page 1 Write Protect Bit"]
pub type Bc1wpR = crate::BitReader;
#[doc = "Field `BC1WP` writer - Boot Configuration Page 1 Write Protect Bit"]
pub type Bc1wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC2WP` reader - User Configuration Page 2 Write Protect Bit"]
pub type Uc2wpR = crate::BitReader;
#[doc = "Field `UC2WP` writer - User Configuration Page 2 Write Protect Bit"]
pub type Uc2wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC2WP` reader - Boot Configuration Page 2 Write Protect Bit"]
pub type Bc2wpR = crate::BitReader;
#[doc = "Field `BC2WP` writer - Boot Configuration Page 2 Write Protect Bit"]
pub type Bc2wpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC1WPLOCK` reader - User Configuration Page 1 WP Lock Bit"]
pub type Uc1wplockR = crate::BitReader;
#[doc = "Field `UC1WPLOCK` writer - User Configuration Page 1 WP Lock Bit"]
pub type Uc1wplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC1WPLOCK` reader - Boot Configuration Page 1 WP Lock Bit"]
pub type Bc1wplockR = crate::BitReader;
#[doc = "Field `BC1WPLOCK` writer - Boot Configuration Page 1 WP Lock Bit"]
pub type Bc1wplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UC2WPLOCK` reader - User Configuration Page 2 WP Lock Bit"]
pub type Uc2wplockR = crate::BitReader;
#[doc = "Field `UC2WPLOCK` writer - User Configuration Page 2 WP Lock Bit"]
pub type Uc2wplockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BC2WPLOCK` reader - Boot Configuration Page 2 WP Lock Bit"]
pub type Bc2wplockR = crate::BitReader;
#[doc = "Field `BC2WPLOCK` writer - Boot Configuration Page 2 WP Lock Bit"]
pub type Bc2wplockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - User Configuration Page 1 Write Protect Bit"]
    #[inline(always)]
    pub fn uc1wp(&self) -> Uc1wpR {
        Uc1wpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Boot Configuration Page 1 Write Protect Bit"]
    #[inline(always)]
    pub fn bc1wp(&self) -> Bc1wpR {
        Bc1wpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - User Configuration Page 2 Write Protect Bit"]
    #[inline(always)]
    pub fn uc2wp(&self) -> Uc2wpR {
        Uc2wpR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Boot Configuration Page 2 Write Protect Bit"]
    #[inline(always)]
    pub fn bc2wp(&self) -> Bc2wpR {
        Bc2wpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - User Configuration Page 1 WP Lock Bit"]
    #[inline(always)]
    pub fn uc1wplock(&self) -> Uc1wplockR {
        Uc1wplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Boot Configuration Page 1 WP Lock Bit"]
    #[inline(always)]
    pub fn bc1wplock(&self) -> Bc1wplockR {
        Bc1wplockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - User Configuration Page 2 WP Lock Bit"]
    #[inline(always)]
    pub fn uc2wplock(&self) -> Uc2wplockR {
        Uc2wplockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Boot Configuration Page 2 WP Lock Bit"]
    #[inline(always)]
    pub fn bc2wplock(&self) -> Bc2wplockR {
        Bc2wplockR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - User Configuration Page 1 Write Protect Bit"]
    #[inline(always)]
    pub fn uc1wp(&mut self) -> Uc1wpW<'_, CwpSpec> {
        Uc1wpW::new(self, 0)
    }
    #[doc = "Bit 2 - Boot Configuration Page 1 Write Protect Bit"]
    #[inline(always)]
    pub fn bc1wp(&mut self) -> Bc1wpW<'_, CwpSpec> {
        Bc1wpW::new(self, 2)
    }
    #[doc = "Bit 8 - User Configuration Page 2 Write Protect Bit"]
    #[inline(always)]
    pub fn uc2wp(&mut self) -> Uc2wpW<'_, CwpSpec> {
        Uc2wpW::new(self, 8)
    }
    #[doc = "Bit 10 - Boot Configuration Page 2 Write Protect Bit"]
    #[inline(always)]
    pub fn bc2wp(&mut self) -> Bc2wpW<'_, CwpSpec> {
        Bc2wpW::new(self, 10)
    }
    #[doc = "Bit 16 - User Configuration Page 1 WP Lock Bit"]
    #[inline(always)]
    pub fn uc1wplock(&mut self) -> Uc1wplockW<'_, CwpSpec> {
        Uc1wplockW::new(self, 16)
    }
    #[doc = "Bit 18 - Boot Configuration Page 1 WP Lock Bit"]
    #[inline(always)]
    pub fn bc1wplock(&mut self) -> Bc1wplockW<'_, CwpSpec> {
        Bc1wplockW::new(self, 18)
    }
    #[doc = "Bit 24 - User Configuration Page 2 WP Lock Bit"]
    #[inline(always)]
    pub fn uc2wplock(&mut self) -> Uc2wplockW<'_, CwpSpec> {
        Uc2wplockW::new(self, 24)
    }
    #[doc = "Bit 26 - Boot Configuration Page 2 WP Lock Bit"]
    #[inline(always)]
    pub fn bc2wplock(&mut self) -> Bc2wplockW<'_, CwpSpec> {
        Bc2wplockW::new(self, 26)
    }
}
#[doc = "CFM Page Write Protect Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwpSpec;
impl crate::RegisterSpec for CwpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwp::R`](R) reader structure"]
impl crate::Readable for CwpSpec {}
#[doc = "`write(|w| ..)` method takes [`cwp::W`](W) writer structure"]
impl crate::Writable for CwpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWP to value 0x0505"]
impl crate::Resettable for CwpSpec {
    const RESET_VALUE: u32 = 0x0505;
}
