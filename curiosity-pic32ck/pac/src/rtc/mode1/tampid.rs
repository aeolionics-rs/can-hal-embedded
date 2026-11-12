#[doc = "Register `TAMPID` reader"]
pub type R = crate::R<TampidSpec>;
#[doc = "Register `TAMPID` writer"]
pub type W = crate::W<TampidSpec>;
#[doc = "Field `TAMPID0` reader - Tamper Input 0 Detected"]
pub type Tampid0R = crate::BitReader;
#[doc = "Field `TAMPID0` writer - Tamper Input 0 Detected"]
pub type Tampid0W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID1` reader - Tamper Input 1 Detected"]
pub type Tampid1R = crate::BitReader;
#[doc = "Field `TAMPID1` writer - Tamper Input 1 Detected"]
pub type Tampid1W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID2` reader - Tamper Input 2 Detected"]
pub type Tampid2R = crate::BitReader;
#[doc = "Field `TAMPID2` writer - Tamper Input 2 Detected"]
pub type Tampid2W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID3` reader - Tamper Input 3 Detected"]
pub type Tampid3R = crate::BitReader;
#[doc = "Field `TAMPID3` writer - Tamper Input 3 Detected"]
pub type Tampid3W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID4` reader - Tamper Input 4 Detected"]
pub type Tampid4R = crate::BitReader;
#[doc = "Field `TAMPID4` writer - Tamper Input 4 Detected"]
pub type Tampid4W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID5` reader - Tamper Input 5 Detected"]
pub type Tampid5R = crate::BitReader;
#[doc = "Field `TAMPID5` writer - Tamper Input 5 Detected"]
pub type Tampid5W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID6` reader - Tamper Input 6 Detected"]
pub type Tampid6R = crate::BitReader;
#[doc = "Field `TAMPID6` writer - Tamper Input 6 Detected"]
pub type Tampid6W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPID7` reader - Tamper Input 7 Detected"]
pub type Tampid7R = crate::BitReader;
#[doc = "Field `TAMPID7` writer - Tamper Input 7 Detected"]
pub type Tampid7W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TAMPEVT` reader - Tamper Event Detected"]
pub type TampevtR = crate::BitReader;
#[doc = "Field `TAMPEVT` writer - Tamper Event Detected"]
pub type TampevtW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&self) -> Tampid0R {
        Tampid0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&self) -> Tampid1R {
        Tampid1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&self) -> Tampid2R {
        Tampid2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&self) -> Tampid3R {
        Tampid3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&self) -> Tampid4R {
        Tampid4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tamper Input 5 Detected"]
    #[inline(always)]
    pub fn tampid5(&self) -> Tampid5R {
        Tampid5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tamper Input 6 Detected"]
    #[inline(always)]
    pub fn tampid6(&self) -> Tampid6R {
        Tampid6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tamper Input 7 Detected"]
    #[inline(always)]
    pub fn tampid7(&self) -> Tampid7R {
        Tampid7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&self) -> TampevtR {
        TampevtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper Input 0 Detected"]
    #[inline(always)]
    pub fn tampid0(&mut self) -> Tampid0W<'_, TampidSpec> {
        Tampid0W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper Input 1 Detected"]
    #[inline(always)]
    pub fn tampid1(&mut self) -> Tampid1W<'_, TampidSpec> {
        Tampid1W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper Input 2 Detected"]
    #[inline(always)]
    pub fn tampid2(&mut self) -> Tampid2W<'_, TampidSpec> {
        Tampid2W::new(self, 2)
    }
    #[doc = "Bit 3 - Tamper Input 3 Detected"]
    #[inline(always)]
    pub fn tampid3(&mut self) -> Tampid3W<'_, TampidSpec> {
        Tampid3W::new(self, 3)
    }
    #[doc = "Bit 4 - Tamper Input 4 Detected"]
    #[inline(always)]
    pub fn tampid4(&mut self) -> Tampid4W<'_, TampidSpec> {
        Tampid4W::new(self, 4)
    }
    #[doc = "Bit 5 - Tamper Input 5 Detected"]
    #[inline(always)]
    pub fn tampid5(&mut self) -> Tampid5W<'_, TampidSpec> {
        Tampid5W::new(self, 5)
    }
    #[doc = "Bit 6 - Tamper Input 6 Detected"]
    #[inline(always)]
    pub fn tampid6(&mut self) -> Tampid6W<'_, TampidSpec> {
        Tampid6W::new(self, 6)
    }
    #[doc = "Bit 7 - Tamper Input 7 Detected"]
    #[inline(always)]
    pub fn tampid7(&mut self) -> Tampid7W<'_, TampidSpec> {
        Tampid7W::new(self, 7)
    }
    #[doc = "Bit 31 - Tamper Event Detected"]
    #[inline(always)]
    pub fn tampevt(&mut self) -> TampevtW<'_, TampidSpec> {
        TampevtW::new(self, 31)
    }
}
#[doc = "Tamper ID\n\nYou can [`read`](crate::Reg::read) this register and get [`tampid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampidSpec;
impl crate::RegisterSpec for TampidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampid::R`](R) reader structure"]
impl crate::Readable for TampidSpec {}
#[doc = "`write(|w| ..)` method takes [`tampid::W`](W) writer structure"]
impl crate::Writable for TampidSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000_00ff;
}
#[doc = "`reset()` method sets TAMPID to value 0"]
impl crate::Resettable for TampidSpec {}
