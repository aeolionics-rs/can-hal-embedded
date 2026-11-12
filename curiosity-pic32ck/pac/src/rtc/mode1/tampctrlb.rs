#[doc = "Register `TAMPCTRLB` reader"]
pub type R = crate::R<TampctrlbSpec>;
#[doc = "Register `TAMPCTRLB` writer"]
pub type W = crate::W<TampctrlbSpec>;
#[doc = "Field `ALSI0` reader - Active Layer Select Internal 0"]
pub type Alsi0R = crate::BitReader;
#[doc = "Field `ALSI0` writer - Active Layer Select Internal 0"]
pub type Alsi0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI1` reader - Active Layer Select Internal 1"]
pub type Alsi1R = crate::BitReader;
#[doc = "Field `ALSI1` writer - Active Layer Select Internal 1"]
pub type Alsi1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI2` reader - Active Layer Select Internal 2"]
pub type Alsi2R = crate::BitReader;
#[doc = "Field `ALSI2` writer - Active Layer Select Internal 2"]
pub type Alsi2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI3` reader - Active Layer Select Internal 3"]
pub type Alsi3R = crate::BitReader;
#[doc = "Field `ALSI3` writer - Active Layer Select Internal 3"]
pub type Alsi3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI4` reader - Active Layer Select Internal 4"]
pub type Alsi4R = crate::BitReader;
#[doc = "Field `ALSI4` writer - Active Layer Select Internal 4"]
pub type Alsi4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI5` reader - Active Layer Select Internal 5"]
pub type Alsi5R = crate::BitReader;
#[doc = "Field `ALSI5` writer - Active Layer Select Internal 5"]
pub type Alsi5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI6` reader - Active Layer Select Internal 6"]
pub type Alsi6R = crate::BitReader;
#[doc = "Field `ALSI6` writer - Active Layer Select Internal 6"]
pub type Alsi6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALSI7` reader - Active Layer Select Internal 7"]
pub type Alsi7R = crate::BitReader;
#[doc = "Field `ALSI7` writer - Active Layer Select Internal 7"]
pub type Alsi7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&self) -> Alsi0R {
        Alsi0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&self) -> Alsi1R {
        Alsi1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&self) -> Alsi2R {
        Alsi2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&self) -> Alsi3R {
        Alsi3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active Layer Select Internal 4"]
    #[inline(always)]
    pub fn alsi4(&self) -> Alsi4R {
        Alsi4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Active Layer Select Internal 5"]
    #[inline(always)]
    pub fn alsi5(&self) -> Alsi5R {
        Alsi5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active Layer Select Internal 6"]
    #[inline(always)]
    pub fn alsi6(&self) -> Alsi6R {
        Alsi6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Active Layer Select Internal 7"]
    #[inline(always)]
    pub fn alsi7(&self) -> Alsi7R {
        Alsi7R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Active Layer Select Internal 0"]
    #[inline(always)]
    pub fn alsi0(&mut self) -> Alsi0W<'_, TampctrlbSpec> {
        Alsi0W::new(self, 0)
    }
    #[doc = "Bit 1 - Active Layer Select Internal 1"]
    #[inline(always)]
    pub fn alsi1(&mut self) -> Alsi1W<'_, TampctrlbSpec> {
        Alsi1W::new(self, 1)
    }
    #[doc = "Bit 2 - Active Layer Select Internal 2"]
    #[inline(always)]
    pub fn alsi2(&mut self) -> Alsi2W<'_, TampctrlbSpec> {
        Alsi2W::new(self, 2)
    }
    #[doc = "Bit 3 - Active Layer Select Internal 3"]
    #[inline(always)]
    pub fn alsi3(&mut self) -> Alsi3W<'_, TampctrlbSpec> {
        Alsi3W::new(self, 3)
    }
    #[doc = "Bit 4 - Active Layer Select Internal 4"]
    #[inline(always)]
    pub fn alsi4(&mut self) -> Alsi4W<'_, TampctrlbSpec> {
        Alsi4W::new(self, 4)
    }
    #[doc = "Bit 5 - Active Layer Select Internal 5"]
    #[inline(always)]
    pub fn alsi5(&mut self) -> Alsi5W<'_, TampctrlbSpec> {
        Alsi5W::new(self, 5)
    }
    #[doc = "Bit 6 - Active Layer Select Internal 6"]
    #[inline(always)]
    pub fn alsi6(&mut self) -> Alsi6W<'_, TampctrlbSpec> {
        Alsi6W::new(self, 6)
    }
    #[doc = "Bit 7 - Active Layer Select Internal 7"]
    #[inline(always)]
    pub fn alsi7(&mut self) -> Alsi7W<'_, TampctrlbSpec> {
        Alsi7W::new(self, 7)
    }
}
#[doc = "Tamper Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`tampctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tampctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TampctrlbSpec;
impl crate::RegisterSpec for TampctrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tampctrlb::R`](R) reader structure"]
impl crate::Readable for TampctrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`tampctrlb::W`](W) writer structure"]
impl crate::Writable for TampctrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TAMPCTRLB to value 0"]
impl crate::Resettable for TampctrlbSpec {}
