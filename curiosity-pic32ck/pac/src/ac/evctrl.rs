#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `COMPEO0` reader - Comparator 0 Event Output Enable"]
pub type Compeo0R = crate::BitReader;
#[doc = "Field `COMPEO0` writer - Comparator 0 Event Output Enable"]
pub type Compeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEO1` reader - Comparator 1 Event Output Enable"]
pub type Compeo1R = crate::BitReader;
#[doc = "Field `COMPEO1` writer - Comparator 1 Event Output Enable"]
pub type Compeo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEO0` reader - Window 0 Event Output Enable"]
pub type Wineo0R = crate::BitReader;
#[doc = "Field `WINEO0` writer - Window 0 Event Output Enable"]
pub type Wineo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI0` reader - Comparator 0 Event Input Enable"]
pub type Compei0R = crate::BitReader;
#[doc = "Field `COMPEI0` writer - Comparator 0 Event Input Enable"]
pub type Compei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI1` reader - Comparator 1 Event Input Enable"]
pub type Compei1R = crate::BitReader;
#[doc = "Field `COMPEI1` writer - Comparator 1 Event Input Enable"]
pub type Compei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI0` reader - Inverted Event Input Enable 0"]
pub type Invei0R = crate::BitReader;
#[doc = "Field `INVEI0` writer - Inverted Event Input Enable 0"]
pub type Invei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI1` reader - Inverted Event Input Enable 1"]
pub type Invei1R = crate::BitReader;
#[doc = "Field `INVEI1` writer - Inverted Event Input Enable 1"]
pub type Invei1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> Compeo0R {
        Compeo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> Compeo1R {
        Compeo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> Wineo0R {
        Wineo0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> Compei0R {
        Compei0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> Compei1R {
        Compei1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - Inverted Event Input Enable 0"]
    #[inline(always)]
    pub fn invei0(&self) -> Invei0R {
        Invei0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Inverted Event Input Enable 1"]
    #[inline(always)]
    pub fn invei1(&self) -> Invei1R {
        Invei1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&mut self) -> Compeo0W<'_, EvctrlSpec> {
        Compeo0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&mut self) -> Compeo1W<'_, EvctrlSpec> {
        Compeo1W::new(self, 1)
    }
    #[doc = "Bit 8 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&mut self) -> Wineo0W<'_, EvctrlSpec> {
        Wineo0W::new(self, 8)
    }
    #[doc = "Bit 16 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&mut self) -> Compei0W<'_, EvctrlSpec> {
        Compei0W::new(self, 16)
    }
    #[doc = "Bit 17 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&mut self) -> Compei1W<'_, EvctrlSpec> {
        Compei1W::new(self, 17)
    }
    #[doc = "Bit 24 - Inverted Event Input Enable 0"]
    #[inline(always)]
    pub fn invei0(&mut self) -> Invei0W<'_, EvctrlSpec> {
        Invei0W::new(self, 24)
    }
    #[doc = "Bit 25 - Inverted Event Input Enable 1"]
    #[inline(always)]
    pub fn invei1(&mut self) -> Invei1W<'_, EvctrlSpec> {
        Invei1W::new(self, 25)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {}
