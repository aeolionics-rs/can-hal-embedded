#[doc = "Register `PRAS` reader"]
pub type R = crate::R<PrasSpec>;
#[doc = "Register `PRAS` writer"]
pub type W = crate::W<PrasSpec>;
#[doc = "Field `M0PR` reader - Master 0 Priority"]
pub type M0prR = crate::FieldReader;
#[doc = "Field `M0PR` writer - Master 0 Priority"]
pub type M0prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN0` reader - Latency Quality Of Service Enable for Master 0"]
pub type Lqosen0R = crate::BitReader;
#[doc = "Field `LQOSEN0` writer - Latency Quality Of Service Enable for Master 0"]
pub type Lqosen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1PR` reader - Master 1 Priority"]
pub type M1prR = crate::FieldReader;
#[doc = "Field `M1PR` writer - Master 1 Priority"]
pub type M1prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN1` reader - Latency Quality Of Service Enable for Master 1"]
pub type Lqosen1R = crate::BitReader;
#[doc = "Field `LQOSEN1` writer - Latency Quality Of Service Enable for Master 1"]
pub type Lqosen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M2PR` reader - Master 2 Priority"]
pub type M2prR = crate::FieldReader;
#[doc = "Field `M2PR` writer - Master 2 Priority"]
pub type M2prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN2` reader - Latency Quality Of Service Enable for Master 2"]
pub type Lqosen2R = crate::BitReader;
#[doc = "Field `LQOSEN2` writer - Latency Quality Of Service Enable for Master 2"]
pub type Lqosen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M3PR` reader - Master 3 Priority"]
pub type M3prR = crate::FieldReader;
#[doc = "Field `M3PR` writer - Master 3 Priority"]
pub type M3prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN3` reader - Latency Quality Of Service Enable for Master 3"]
pub type Lqosen3R = crate::BitReader;
#[doc = "Field `LQOSEN3` writer - Latency Quality Of Service Enable for Master 3"]
pub type Lqosen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M4PR` reader - Master 4 Priority"]
pub type M4prR = crate::FieldReader;
#[doc = "Field `M4PR` writer - Master 4 Priority"]
pub type M4prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN4` reader - Latency Quality Of Service Enable for Master 4"]
pub type Lqosen4R = crate::BitReader;
#[doc = "Field `LQOSEN4` writer - Latency Quality Of Service Enable for Master 4"]
pub type Lqosen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M5PR` reader - Master 5 Priority"]
pub type M5prR = crate::FieldReader;
#[doc = "Field `M5PR` writer - Master 5 Priority"]
pub type M5prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN5` reader - Latency Quality Of Service Enable for Master 5"]
pub type Lqosen5R = crate::BitReader;
#[doc = "Field `LQOSEN5` writer - Latency Quality Of Service Enable for Master 5"]
pub type Lqosen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M6PR` reader - Master 6 Priority"]
pub type M6prR = crate::FieldReader;
#[doc = "Field `M6PR` writer - Master 6 Priority"]
pub type M6prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN6` reader - Latency Quality Of Service Enable for Master 6"]
pub type Lqosen6R = crate::BitReader;
#[doc = "Field `LQOSEN6` writer - Latency Quality Of Service Enable for Master 6"]
pub type Lqosen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M7PR` reader - Master 7 Priority"]
pub type M7prR = crate::FieldReader;
#[doc = "Field `M7PR` writer - Master 7 Priority"]
pub type M7prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN7` reader - Latency Quality Of Service Enable for Master 7"]
pub type Lqosen7R = crate::BitReader;
#[doc = "Field `LQOSEN7` writer - Latency Quality Of Service Enable for Master 7"]
pub type Lqosen7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&self) -> M0prR {
        M0prR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Latency Quality Of Service Enable for Master 0"]
    #[inline(always)]
    pub fn lqosen0(&self) -> Lqosen0R {
        Lqosen0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&self) -> M1prR {
        M1prR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Latency Quality Of Service Enable for Master 1"]
    #[inline(always)]
    pub fn lqosen1(&self) -> Lqosen1R {
        Lqosen1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&self) -> M2prR {
        M2prR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Latency Quality Of Service Enable for Master 2"]
    #[inline(always)]
    pub fn lqosen2(&self) -> Lqosen2R {
        Lqosen2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&self) -> M3prR {
        M3prR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Latency Quality Of Service Enable for Master 3"]
    #[inline(always)]
    pub fn lqosen3(&self) -> Lqosen3R {
        Lqosen3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&self) -> M4prR {
        M4prR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Latency Quality Of Service Enable for Master 4"]
    #[inline(always)]
    pub fn lqosen4(&self) -> Lqosen4R {
        Lqosen4R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&self) -> M5prR {
        M5prR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Latency Quality Of Service Enable for Master 5"]
    #[inline(always)]
    pub fn lqosen5(&self) -> Lqosen5R {
        Lqosen5R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&self) -> M6prR {
        M6prR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Latency Quality Of Service Enable for Master 6"]
    #[inline(always)]
    pub fn lqosen6(&self) -> Lqosen6R {
        Lqosen6R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&self) -> M7prR {
        M7prR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Latency Quality Of Service Enable for Master 7"]
    #[inline(always)]
    pub fn lqosen7(&self) -> Lqosen7R {
        Lqosen7R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Priority"]
    #[inline(always)]
    pub fn m0pr(&mut self) -> M0prW<'_, PrasSpec> {
        M0prW::new(self, 0)
    }
    #[doc = "Bit 2 - Latency Quality Of Service Enable for Master 0"]
    #[inline(always)]
    pub fn lqosen0(&mut self) -> Lqosen0W<'_, PrasSpec> {
        Lqosen0W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Master 1 Priority"]
    #[inline(always)]
    pub fn m1pr(&mut self) -> M1prW<'_, PrasSpec> {
        M1prW::new(self, 4)
    }
    #[doc = "Bit 6 - Latency Quality Of Service Enable for Master 1"]
    #[inline(always)]
    pub fn lqosen1(&mut self) -> Lqosen1W<'_, PrasSpec> {
        Lqosen1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Master 2 Priority"]
    #[inline(always)]
    pub fn m2pr(&mut self) -> M2prW<'_, PrasSpec> {
        M2prW::new(self, 8)
    }
    #[doc = "Bit 10 - Latency Quality Of Service Enable for Master 2"]
    #[inline(always)]
    pub fn lqosen2(&mut self) -> Lqosen2W<'_, PrasSpec> {
        Lqosen2W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Master 3 Priority"]
    #[inline(always)]
    pub fn m3pr(&mut self) -> M3prW<'_, PrasSpec> {
        M3prW::new(self, 12)
    }
    #[doc = "Bit 14 - Latency Quality Of Service Enable for Master 3"]
    #[inline(always)]
    pub fn lqosen3(&mut self) -> Lqosen3W<'_, PrasSpec> {
        Lqosen3W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Master 4 Priority"]
    #[inline(always)]
    pub fn m4pr(&mut self) -> M4prW<'_, PrasSpec> {
        M4prW::new(self, 16)
    }
    #[doc = "Bit 18 - Latency Quality Of Service Enable for Master 4"]
    #[inline(always)]
    pub fn lqosen4(&mut self) -> Lqosen4W<'_, PrasSpec> {
        Lqosen4W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Master 5 Priority"]
    #[inline(always)]
    pub fn m5pr(&mut self) -> M5prW<'_, PrasSpec> {
        M5prW::new(self, 20)
    }
    #[doc = "Bit 22 - Latency Quality Of Service Enable for Master 5"]
    #[inline(always)]
    pub fn lqosen5(&mut self) -> Lqosen5W<'_, PrasSpec> {
        Lqosen5W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Master 6 Priority"]
    #[inline(always)]
    pub fn m6pr(&mut self) -> M6prW<'_, PrasSpec> {
        M6prW::new(self, 24)
    }
    #[doc = "Bit 26 - Latency Quality Of Service Enable for Master 6"]
    #[inline(always)]
    pub fn lqosen6(&mut self) -> Lqosen6W<'_, PrasSpec> {
        Lqosen6W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Master 7 Priority"]
    #[inline(always)]
    pub fn m7pr(&mut self) -> M7prW<'_, PrasSpec> {
        M7prW::new(self, 28)
    }
    #[doc = "Bit 30 - Latency Quality Of Service Enable for Master 7"]
    #[inline(always)]
    pub fn lqosen7(&mut self) -> Lqosen7W<'_, PrasSpec> {
        Lqosen7W::new(self, 30)
    }
}
#[doc = "Priority A for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`pras::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pras::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrasSpec;
impl crate::RegisterSpec for PrasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pras::R`](R) reader structure"]
impl crate::Readable for PrasSpec {}
#[doc = "`write(|w| ..)` method takes [`pras::W`](W) writer structure"]
impl crate::Writable for PrasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRAS to value 0"]
impl crate::Resettable for PrasSpec {}
