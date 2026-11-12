#[doc = "Register `PRBS` reader"]
pub type R = crate::R<PrbsSpec>;
#[doc = "Register `PRBS` writer"]
pub type W = crate::W<PrbsSpec>;
#[doc = "Field `M8PR` reader - Master 8 Priority"]
pub type M8prR = crate::FieldReader;
#[doc = "Field `M8PR` writer - Master 8 Priority"]
pub type M8prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN8` reader - Latency Quality Of Service Enable for Master 8"]
pub type Lqosen8R = crate::BitReader;
#[doc = "Field `LQOSEN8` writer - Latency Quality Of Service Enable for Master 8"]
pub type Lqosen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M9PR` reader - Master 9 Priority"]
pub type M9prR = crate::FieldReader;
#[doc = "Field `M9PR` writer - Master 9 Priority"]
pub type M9prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN9` reader - Latency Quality Of Service Enable for Master 9"]
pub type Lqosen9R = crate::BitReader;
#[doc = "Field `LQOSEN9` writer - Latency Quality Of Service Enable for Master 9"]
pub type Lqosen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M10PR` reader - Master 10 Priority"]
pub type M10prR = crate::FieldReader;
#[doc = "Field `M10PR` writer - Master 10 Priority"]
pub type M10prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN10` reader - Latency Quality Of Service Enable for Master 10"]
pub type Lqosen10R = crate::BitReader;
#[doc = "Field `LQOSEN10` writer - Latency Quality Of Service Enable for Master 10"]
pub type Lqosen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M11PR` reader - Master 11 Priority"]
pub type M11prR = crate::FieldReader;
#[doc = "Field `M11PR` writer - Master 11 Priority"]
pub type M11prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN11` reader - Latency Quality Of Service Enable for Master 11"]
pub type Lqosen11R = crate::BitReader;
#[doc = "Field `LQOSEN11` writer - Latency Quality Of Service Enable for Master 11"]
pub type Lqosen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M12PR` reader - Master 12 Priority"]
pub type M12prR = crate::FieldReader;
#[doc = "Field `M12PR` writer - Master 12 Priority"]
pub type M12prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN12` reader - Latency Quality Of Service Enable for Master 12"]
pub type Lqosen12R = crate::BitReader;
#[doc = "Field `LQOSEN12` writer - Latency Quality Of Service Enable for Master 12"]
pub type Lqosen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M13PR` reader - Master 13 Priority"]
pub type M13prR = crate::FieldReader;
#[doc = "Field `M13PR` writer - Master 13 Priority"]
pub type M13prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN13` reader - Latency Quality Of Service Enable for Master 13"]
pub type Lqosen13R = crate::BitReader;
#[doc = "Field `LQOSEN13` writer - Latency Quality Of Service Enable for Master 13"]
pub type Lqosen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M14PR` reader - Master 14 Priority"]
pub type M14prR = crate::FieldReader;
#[doc = "Field `M14PR` writer - Master 14 Priority"]
pub type M14prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN14` reader - Latency Quality Of Service Enable for Master 14"]
pub type Lqosen14R = crate::BitReader;
#[doc = "Field `LQOSEN14` writer - Latency Quality Of Service Enable for Master 14"]
pub type Lqosen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M15PR` reader - Master 15 Priority"]
pub type M15prR = crate::FieldReader;
#[doc = "Field `M15PR` writer - Master 15 Priority"]
pub type M15prW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LQOSEN15` reader - Latency Quality Of Service Enable for Master 15"]
pub type Lqosen15R = crate::BitReader;
#[doc = "Field `LQOSEN15` writer - Latency Quality Of Service Enable for Master 15"]
pub type Lqosen15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8prR {
        M8prR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Latency Quality Of Service Enable for Master 8"]
    #[inline(always)]
    pub fn lqosen8(&self) -> Lqosen8R {
        Lqosen8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9prR {
        M9prR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Latency Quality Of Service Enable for Master 9"]
    #[inline(always)]
    pub fn lqosen9(&self) -> Lqosen9R {
        Lqosen9R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10prR {
        M10prR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Latency Quality Of Service Enable for Master 10"]
    #[inline(always)]
    pub fn lqosen10(&self) -> Lqosen10R {
        Lqosen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&self) -> M11prR {
        M11prR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Latency Quality Of Service Enable for Master 11"]
    #[inline(always)]
    pub fn lqosen11(&self) -> Lqosen11R {
        Lqosen11R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&self) -> M12prR {
        M12prR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Latency Quality Of Service Enable for Master 12"]
    #[inline(always)]
    pub fn lqosen12(&self) -> Lqosen12R {
        Lqosen12R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Master 13 Priority"]
    #[inline(always)]
    pub fn m13pr(&self) -> M13prR {
        M13prR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Latency Quality Of Service Enable for Master 13"]
    #[inline(always)]
    pub fn lqosen13(&self) -> Lqosen13R {
        Lqosen13R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Master 14 Priority"]
    #[inline(always)]
    pub fn m14pr(&self) -> M14prR {
        M14prR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Latency Quality Of Service Enable for Master 14"]
    #[inline(always)]
    pub fn lqosen14(&self) -> Lqosen14R {
        Lqosen14R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Master 15 Priority"]
    #[inline(always)]
    pub fn m15pr(&self) -> M15prR {
        M15prR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Latency Quality Of Service Enable for Master 15"]
    #[inline(always)]
    pub fn lqosen15(&self) -> Lqosen15R {
        Lqosen15R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8prW<'_, PrbsSpec> {
        M8prW::new(self, 0)
    }
    #[doc = "Bit 2 - Latency Quality Of Service Enable for Master 8"]
    #[inline(always)]
    pub fn lqosen8(&mut self) -> Lqosen8W<'_, PrbsSpec> {
        Lqosen8W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9prW<'_, PrbsSpec> {
        M9prW::new(self, 4)
    }
    #[doc = "Bit 6 - Latency Quality Of Service Enable for Master 9"]
    #[inline(always)]
    pub fn lqosen9(&mut self) -> Lqosen9W<'_, PrbsSpec> {
        Lqosen9W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10prW<'_, PrbsSpec> {
        M10prW::new(self, 8)
    }
    #[doc = "Bit 10 - Latency Quality Of Service Enable for Master 10"]
    #[inline(always)]
    pub fn lqosen10(&mut self) -> Lqosen10W<'_, PrbsSpec> {
        Lqosen10W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Master 11 Priority"]
    #[inline(always)]
    pub fn m11pr(&mut self) -> M11prW<'_, PrbsSpec> {
        M11prW::new(self, 12)
    }
    #[doc = "Bit 14 - Latency Quality Of Service Enable for Master 11"]
    #[inline(always)]
    pub fn lqosen11(&mut self) -> Lqosen11W<'_, PrbsSpec> {
        Lqosen11W::new(self, 14)
    }
    #[doc = "Bits 16:17 - Master 12 Priority"]
    #[inline(always)]
    pub fn m12pr(&mut self) -> M12prW<'_, PrbsSpec> {
        M12prW::new(self, 16)
    }
    #[doc = "Bit 18 - Latency Quality Of Service Enable for Master 12"]
    #[inline(always)]
    pub fn lqosen12(&mut self) -> Lqosen12W<'_, PrbsSpec> {
        Lqosen12W::new(self, 18)
    }
    #[doc = "Bits 20:21 - Master 13 Priority"]
    #[inline(always)]
    pub fn m13pr(&mut self) -> M13prW<'_, PrbsSpec> {
        M13prW::new(self, 20)
    }
    #[doc = "Bit 22 - Latency Quality Of Service Enable for Master 13"]
    #[inline(always)]
    pub fn lqosen13(&mut self) -> Lqosen13W<'_, PrbsSpec> {
        Lqosen13W::new(self, 22)
    }
    #[doc = "Bits 24:25 - Master 14 Priority"]
    #[inline(always)]
    pub fn m14pr(&mut self) -> M14prW<'_, PrbsSpec> {
        M14prW::new(self, 24)
    }
    #[doc = "Bit 26 - Latency Quality Of Service Enable for Master 14"]
    #[inline(always)]
    pub fn lqosen14(&mut self) -> Lqosen14W<'_, PrbsSpec> {
        Lqosen14W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Master 15 Priority"]
    #[inline(always)]
    pub fn m15pr(&mut self) -> M15prW<'_, PrbsSpec> {
        M15prW::new(self, 28)
    }
    #[doc = "Bit 30 - Latency Quality Of Service Enable for Master 15"]
    #[inline(always)]
    pub fn lqosen15(&mut self) -> Lqosen15W<'_, PrbsSpec> {
        Lqosen15W::new(self, 30)
    }
}
#[doc = "Priority B for Slave\n\nYou can [`read`](crate::Reg::read) this register and get [`prbs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prbs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrbsSpec;
impl crate::RegisterSpec for PrbsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prbs::R`](R) reader structure"]
impl crate::Readable for PrbsSpec {}
#[doc = "`write(|w| ..)` method takes [`prbs::W`](W) writer structure"]
impl crate::Writable for PrbsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRBS to value 0"]
impl crate::Resettable for PrbsSpec {}
