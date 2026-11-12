#[doc = "Register `MEIMR` reader"]
pub type R = crate::R<MeimrSpec>;
#[doc = "Field `MERR0` reader - Master 0 Access Error"]
pub type Merr0R = crate::BitReader;
#[doc = "Field `MERR1` reader - Master 1 Access Error"]
pub type Merr1R = crate::BitReader;
#[doc = "Field `MERR2` reader - Master 2 Access Error"]
pub type Merr2R = crate::BitReader;
#[doc = "Field `MERR3` reader - Master 3 Access Error"]
pub type Merr3R = crate::BitReader;
#[doc = "Field `MERR4` reader - Master 4 Access Error"]
pub type Merr4R = crate::BitReader;
#[doc = "Field `MERR5` reader - Master 5 Access Error"]
pub type Merr5R = crate::BitReader;
#[doc = "Field `MERR6` reader - Master 6 Access Error"]
pub type Merr6R = crate::BitReader;
#[doc = "Field `MERR7` reader - Master 7 Access Error"]
pub type Merr7R = crate::BitReader;
#[doc = "Field `MERR8` reader - Master 8 Access Error"]
pub type Merr8R = crate::BitReader;
#[doc = "Field `MERR9` reader - Master 9 Access Error"]
pub type Merr9R = crate::BitReader;
#[doc = "Field `MERR10` reader - Master 10 Access Error"]
pub type Merr10R = crate::BitReader;
#[doc = "Field `MERR11` reader - Master 11 Access Error"]
pub type Merr11R = crate::BitReader;
#[doc = "Field `MERR12` reader - Master 12 Access Error"]
pub type Merr12R = crate::BitReader;
#[doc = "Field `MERR13` reader - Master 13 Access Error"]
pub type Merr13R = crate::BitReader;
#[doc = "Field `MERR14` reader - Master 14 Access Error"]
pub type Merr14R = crate::BitReader;
#[doc = "Field `MERR15` reader - Master 15 Access Error"]
pub type Merr15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Master 0 Access Error"]
    #[inline(always)]
    pub fn merr0(&self) -> Merr0R {
        Merr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Master 1 Access Error"]
    #[inline(always)]
    pub fn merr1(&self) -> Merr1R {
        Merr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master 2 Access Error"]
    #[inline(always)]
    pub fn merr2(&self) -> Merr2R {
        Merr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master 3 Access Error"]
    #[inline(always)]
    pub fn merr3(&self) -> Merr3R {
        Merr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master 4 Access Error"]
    #[inline(always)]
    pub fn merr4(&self) -> Merr4R {
        Merr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Master 5 Access Error"]
    #[inline(always)]
    pub fn merr5(&self) -> Merr5R {
        Merr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Master 6 Access Error"]
    #[inline(always)]
    pub fn merr6(&self) -> Merr6R {
        Merr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master 7 Access Error"]
    #[inline(always)]
    pub fn merr7(&self) -> Merr7R {
        Merr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Master 8 Access Error"]
    #[inline(always)]
    pub fn merr8(&self) -> Merr8R {
        Merr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master 9 Access Error"]
    #[inline(always)]
    pub fn merr9(&self) -> Merr9R {
        Merr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Master 10 Access Error"]
    #[inline(always)]
    pub fn merr10(&self) -> Merr10R {
        Merr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Master 11 Access Error"]
    #[inline(always)]
    pub fn merr11(&self) -> Merr11R {
        Merr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Master 12 Access Error"]
    #[inline(always)]
    pub fn merr12(&self) -> Merr12R {
        Merr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Master 13 Access Error"]
    #[inline(always)]
    pub fn merr13(&self) -> Merr13R {
        Merr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Master 14 Access Error"]
    #[inline(always)]
    pub fn merr14(&self) -> Merr14R {
        Merr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Master 15 Access Error"]
    #[inline(always)]
    pub fn merr15(&self) -> Merr15R {
        Merr15R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Master Error Interrupt Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`meimr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeimrSpec;
impl crate::RegisterSpec for MeimrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meimr::R`](R) reader structure"]
impl crate::Readable for MeimrSpec {}
#[doc = "`reset()` method sets MEIMR to value 0"]
impl crate::Resettable for MeimrSpec {}
