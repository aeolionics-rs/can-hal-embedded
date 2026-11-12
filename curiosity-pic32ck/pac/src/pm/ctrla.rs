#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "I/O Retention\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ioretselect {
    #[doc = "0: When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are released."]
    Noioret = 0,
    #[doc = "1: When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are stretched."]
    Ioret = 1,
}
impl From<Ioretselect> for bool {
    #[inline(always)]
    fn from(variant: Ioretselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IORET` reader - I/O Retention"]
pub type IoretR = crate::BitReader<Ioretselect>;
impl IoretR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ioretselect {
        match self.bits {
            false => Ioretselect::Noioret,
            true => Ioretselect::Ioret,
        }
    }
    #[doc = "When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are released."]
    #[inline(always)]
    pub fn is_noioret(&self) -> bool {
        *self == Ioretselect::Noioret
    }
    #[doc = "When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are stretched."]
    #[inline(always)]
    pub fn is_ioret(&self) -> bool {
        *self == Ioretselect::Ioret
    }
}
#[doc = "Field `IORET` writer - I/O Retention"]
pub type IoretW<'a, REG> = crate::BitWriter<'a, REG, Ioretselect>;
impl<'a, REG> IoretW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are released."]
    #[inline(always)]
    pub fn noioret(self) -> &'a mut crate::W<REG> {
        self.variant(Ioretselect::Noioret)
    }
    #[doc = "When the device exits the HIBERNATE or BACKUP mode, the I/O line configuration are stretched."]
    #[inline(always)]
    pub fn ioret(self) -> &'a mut crate::W<REG> {
        self.variant(Ioretselect::Ioret)
    }
}
impl R {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    pub fn ioret(&self) -> IoretR {
        IoretR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - I/O Retention"]
    #[inline(always)]
    pub fn ioret(&mut self) -> IoretW<'_, CtrlaSpec> {
        IoretW::new(self, 2)
    }
}
#[doc = "Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
