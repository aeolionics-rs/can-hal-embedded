#[doc = "Register `MEIER` writer"]
pub type W = crate::W<MeierSpec>;
#[doc = "Field `MERR0` writer - Master 0 Access Error"]
pub type Merr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR1` writer - Master 1 Access Error"]
pub type Merr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR2` writer - Master 2 Access Error"]
pub type Merr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR3` writer - Master 3 Access Error"]
pub type Merr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR4` writer - Master 4 Access Error"]
pub type Merr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR5` writer - Master 5 Access Error"]
pub type Merr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR6` writer - Master 6 Access Error"]
pub type Merr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR7` writer - Master 7 Access Error"]
pub type Merr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR8` writer - Master 8 Access Error"]
pub type Merr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR9` writer - Master 9 Access Error"]
pub type Merr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR10` writer - Master 10 Access Error"]
pub type Merr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR11` writer - Master 11 Access Error"]
pub type Merr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR12` writer - Master 12 Access Error"]
pub type Merr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR13` writer - Master 13 Access Error"]
pub type Merr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR14` writer - Master 14 Access Error"]
pub type Merr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MERR15` writer - Master 15 Access Error"]
pub type Merr15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Master 0 Access Error"]
    #[inline(always)]
    pub fn merr0(&mut self) -> Merr0W<'_, MeierSpec> {
        Merr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Master 1 Access Error"]
    #[inline(always)]
    pub fn merr1(&mut self) -> Merr1W<'_, MeierSpec> {
        Merr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Master 2 Access Error"]
    #[inline(always)]
    pub fn merr2(&mut self) -> Merr2W<'_, MeierSpec> {
        Merr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Master 3 Access Error"]
    #[inline(always)]
    pub fn merr3(&mut self) -> Merr3W<'_, MeierSpec> {
        Merr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Master 4 Access Error"]
    #[inline(always)]
    pub fn merr4(&mut self) -> Merr4W<'_, MeierSpec> {
        Merr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Master 5 Access Error"]
    #[inline(always)]
    pub fn merr5(&mut self) -> Merr5W<'_, MeierSpec> {
        Merr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Master 6 Access Error"]
    #[inline(always)]
    pub fn merr6(&mut self) -> Merr6W<'_, MeierSpec> {
        Merr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Master 7 Access Error"]
    #[inline(always)]
    pub fn merr7(&mut self) -> Merr7W<'_, MeierSpec> {
        Merr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Master 8 Access Error"]
    #[inline(always)]
    pub fn merr8(&mut self) -> Merr8W<'_, MeierSpec> {
        Merr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Master 9 Access Error"]
    #[inline(always)]
    pub fn merr9(&mut self) -> Merr9W<'_, MeierSpec> {
        Merr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Master 10 Access Error"]
    #[inline(always)]
    pub fn merr10(&mut self) -> Merr10W<'_, MeierSpec> {
        Merr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Master 11 Access Error"]
    #[inline(always)]
    pub fn merr11(&mut self) -> Merr11W<'_, MeierSpec> {
        Merr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Master 12 Access Error"]
    #[inline(always)]
    pub fn merr12(&mut self) -> Merr12W<'_, MeierSpec> {
        Merr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Master 13 Access Error"]
    #[inline(always)]
    pub fn merr13(&mut self) -> Merr13W<'_, MeierSpec> {
        Merr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Master 14 Access Error"]
    #[inline(always)]
    pub fn merr14(&mut self) -> Merr14W<'_, MeierSpec> {
        Merr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Master 15 Access Error"]
    #[inline(always)]
    pub fn merr15(&mut self) -> Merr15W<'_, MeierSpec> {
        Merr15W::new(self, 15)
    }
}
#[doc = "Master Error Interrupt Enable\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`meier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeierSpec;
impl crate::RegisterSpec for MeierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`meier::W`](W) writer structure"]
impl crate::Writable for MeierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEIER to value 0"]
impl crate::Resettable for MeierSpec {}
