#[doc = "Register `LPMINTREN` reader"]
pub type R = crate::R<LpmintrenSpec>;
#[doc = "Register `LPMINTREN` writer"]
pub type W = crate::W<LpmintrenSpec>;
#[doc = "Field `LPMSTEN` reader - LPMMST interrupt"]
pub type LpmstenR = crate::BitReader;
#[doc = "Field `LPMSTEN` writer - LPMMST interrupt"]
pub type LpmstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNYEN` reader - LPMMNY interrupt"]
pub type LpmnyenR = crate::BitReader;
#[doc = "Field `LPMNYEN` writer - LPMMNY interrupt"]
pub type LpmnyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACKEN` reader - LPMACK interrupt"]
pub type LpmackenR = crate::BitReader;
#[doc = "Field `LPMACKEN` writer - LPMACK interrupt"]
pub type LpmackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNCEN` reader - LPMNC interrupt"]
pub type LpmncenR = crate::BitReader;
#[doc = "Field `LPMNCEN` writer - LPMNC interrupt"]
pub type LpmncenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRESEN` reader - LPMRES interrupt"]
pub type LpmresenR = crate::BitReader;
#[doc = "Field `LPMRESEN` writer - LPMRES interrupt"]
pub type LpmresenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMERREN` reader - LPMERR interrupt"]
pub type LpmerrenR = crate::BitReader;
#[doc = "Field `LPMERREN` writer - LPMERR interrupt"]
pub type LpmerrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPMMST interrupt"]
    #[inline(always)]
    pub fn lpmsten(&self) -> LpmstenR {
        LpmstenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPMMNY interrupt"]
    #[inline(always)]
    pub fn lpmnyen(&self) -> LpmnyenR {
        LpmnyenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LPMACK interrupt"]
    #[inline(always)]
    pub fn lpmacken(&self) -> LpmackenR {
        LpmackenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPMNC interrupt"]
    #[inline(always)]
    pub fn lpmncen(&self) -> LpmncenR {
        LpmncenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPMRES interrupt"]
    #[inline(always)]
    pub fn lpmresen(&self) -> LpmresenR {
        LpmresenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPMERR interrupt"]
    #[inline(always)]
    pub fn lpmerren(&self) -> LpmerrenR {
        LpmerrenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMMST interrupt"]
    #[inline(always)]
    pub fn lpmsten(&mut self) -> LpmstenW<'_, LpmintrenSpec> {
        LpmstenW::new(self, 0)
    }
    #[doc = "Bit 1 - LPMMNY interrupt"]
    #[inline(always)]
    pub fn lpmnyen(&mut self) -> LpmnyenW<'_, LpmintrenSpec> {
        LpmnyenW::new(self, 1)
    }
    #[doc = "Bit 2 - LPMACK interrupt"]
    #[inline(always)]
    pub fn lpmacken(&mut self) -> LpmackenW<'_, LpmintrenSpec> {
        LpmackenW::new(self, 2)
    }
    #[doc = "Bit 3 - LPMNC interrupt"]
    #[inline(always)]
    pub fn lpmncen(&mut self) -> LpmncenW<'_, LpmintrenSpec> {
        LpmncenW::new(self, 3)
    }
    #[doc = "Bit 4 - LPMRES interrupt"]
    #[inline(always)]
    pub fn lpmresen(&mut self) -> LpmresenW<'_, LpmintrenSpec> {
        LpmresenW::new(self, 4)
    }
    #[doc = "Bit 5 - LPMERR interrupt"]
    #[inline(always)]
    pub fn lpmerren(&mut self) -> LpmerrenW<'_, LpmintrenSpec> {
        LpmerrenW::new(self, 5)
    }
}
#[doc = "LPM Interrupts Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintren::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintren::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmintrenSpec;
impl crate::RegisterSpec for LpmintrenSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmintren::R`](R) reader structure"]
impl crate::Readable for LpmintrenSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmintren::W`](W) writer structure"]
impl crate::Writable for LpmintrenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMINTREN to value 0"]
impl crate::Resettable for LpmintrenSpec {}
