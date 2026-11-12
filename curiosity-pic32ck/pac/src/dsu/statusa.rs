#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Register `STATUSA` writer"]
pub type W = crate::W<StatusaSpec>;
#[doc = "Field `DONE` reader - Done"]
pub type DoneR = crate::BitReader;
#[doc = "Field `DONE` writer - Done"]
pub type DoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAIL` reader - Failure"]
pub type FailR = crate::BitReader;
#[doc = "Field `FAIL` writer - Failure"]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERR` reader - Bus Error"]
pub type BerrR = crate::BitReader;
#[doc = "Field `BERR` writer - Bus Error"]
pub type BerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERR` reader - Protection Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `PERR` writer - Protection Error"]
pub type PerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSTEXT0` reader - CPU 0 Reset Phase Extension"]
pub type Crstext0R = crate::BitReader;
#[doc = "Field `CRSTEXT0` writer - CPU 0 Reset Phase Extension"]
pub type Crstext0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSTEXT1` reader - CPU 1 Reset Phase Extension"]
pub type Crstext1R = crate::BitReader;
#[doc = "Field `CRSTEXT1` writer - CPU 1 Reset Phase Extension"]
pub type Crstext1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREXT0` reader - BootRom 0 Phase Extension"]
pub type Brext0R = crate::BitReader;
#[doc = "Field `BREXT0` writer - BootRom 0 Phase Extension"]
pub type Brext0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREXT1` reader - BootRom 1 Phase Extension"]
pub type Brext1R = crate::BitReader;
#[doc = "Field `BREXT1` writer - BootRom 1 Phase Extension"]
pub type Brext1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Failure"]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&self) -> BerrR {
        BerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Protection Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU 0 Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext0(&self) -> Crstext0R {
        Crstext0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU 1 Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext1(&self) -> Crstext1R {
        Crstext1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - BootRom 0 Phase Extension"]
    #[inline(always)]
    pub fn brext0(&self) -> Brext0R {
        Brext0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BootRom 1 Phase Extension"]
    #[inline(always)]
    pub fn brext1(&self) -> Brext1R {
        Brext1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Done"]
    #[inline(always)]
    pub fn done(&mut self) -> DoneW<'_, StatusaSpec> {
        DoneW::new(self, 0)
    }
    #[doc = "Bit 1 - Failure"]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<'_, StatusaSpec> {
        FailW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus Error"]
    #[inline(always)]
    pub fn berr(&mut self) -> BerrW<'_, StatusaSpec> {
        BerrW::new(self, 2)
    }
    #[doc = "Bit 3 - Protection Error"]
    #[inline(always)]
    pub fn perr(&mut self) -> PerrW<'_, StatusaSpec> {
        PerrW::new(self, 3)
    }
    #[doc = "Bit 8 - CPU 0 Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext0(&mut self) -> Crstext0W<'_, StatusaSpec> {
        Crstext0W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU 1 Reset Phase Extension"]
    #[inline(always)]
    pub fn crstext1(&mut self) -> Crstext1W<'_, StatusaSpec> {
        Crstext1W::new(self, 9)
    }
    #[doc = "Bit 16 - BootRom 0 Phase Extension"]
    #[inline(always)]
    pub fn brext0(&mut self) -> Brext0W<'_, StatusaSpec> {
        Brext0W::new(self, 16)
    }
    #[doc = "Bit 17 - BootRom 1 Phase Extension"]
    #[inline(always)]
    pub fn brext1(&mut self) -> Brext1W<'_, StatusaSpec> {
        Brext1W::new(self, 17)
    }
}
#[doc = "Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statusa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`write(|w| ..)` method takes [`statusa::W`](W) writer structure"]
impl crate::Writable for StatusaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {}
