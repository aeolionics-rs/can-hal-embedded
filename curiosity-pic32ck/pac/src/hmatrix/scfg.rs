#[doc = "Register `SCFG[%s]` reader"]
pub type R = crate::R<ScfgSpec>;
#[doc = "Register `SCFG[%s]` writer"]
pub type W = crate::W<ScfgSpec>;
#[doc = "Field `SLOT_CYCLE` reader - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleR = crate::FieldReader<u16>;
#[doc = "Field `SLOT_CYCLE` writer - Maximum Number of Allowed Cycles for a Burst"]
pub type SlotCycleW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Default Master Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DefmstrTypeselect {
    #[doc = "0: No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    NoDefault = 0,
    #[doc = "1: Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    LastDefault = 1,
    #[doc = "2: Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    FixedDefault = 2,
}
impl From<DefmstrTypeselect> for u8 {
    #[inline(always)]
    fn from(variant: DefmstrTypeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DefmstrTypeselect {
    type Ux = u8;
}
impl crate::IsEnum for DefmstrTypeselect {}
#[doc = "Field `DEFMSTR_TYPE` reader - Default Master Type"]
pub type DefmstrTypeR = crate::FieldReader<DefmstrTypeselect>;
impl DefmstrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DefmstrTypeselect> {
        match self.bits {
            0 => Some(DefmstrTypeselect::NoDefault),
            1 => Some(DefmstrTypeselect::LastDefault),
            2 => Some(DefmstrTypeselect::FixedDefault),
            _ => None,
        }
    }
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    #[inline(always)]
    pub fn is_no_default(&self) -> bool {
        *self == DefmstrTypeselect::NoDefault
    }
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    #[inline(always)]
    pub fn is_last_default(&self) -> bool {
        *self == DefmstrTypeselect::LastDefault
    }
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    #[inline(always)]
    pub fn is_fixed_default(&self) -> bool {
        *self == DefmstrTypeselect::FixedDefault
    }
}
#[doc = "Field `DEFMSTR_TYPE` writer - Default Master Type"]
pub type DefmstrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, DefmstrTypeselect>;
impl<'a, REG> DefmstrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Default Master. At the end of current slave access, if no other master request is pending, the slave is deconnected from all masters. This resusts in having a one cycle latency for the first transfer of a burst."]
    #[inline(always)]
    pub fn no_default(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::NoDefault)
    }
    #[doc = "Last Default Master At the end of current slave access, if no other master request is pending, the slave stay connected with the last master havingaccessed it.This resusts in not having the one cycle latency when the last master re-trying access on the slave."]
    #[inline(always)]
    pub fn last_default(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::LastDefault)
    }
    #[doc = "Fixed Default Master At the end of current slave access, if no other master request is pending, the slave connects with fixed master which numberis in FIXED_DEFMSTR register.This resusts in not having the one cycle latency when the fixed master re-trying access on the slave."]
    #[inline(always)]
    pub fn fixed_default(self) -> &'a mut crate::W<REG> {
        self.variant(DefmstrTypeselect::FixedDefault)
    }
}
#[doc = "Field `FIXED_DEFMSTR` reader - Fixed Index of Default Master"]
pub type FixedDefmstrR = crate::FieldReader;
#[doc = "Field `FIXED_DEFMSTR` writer - Fixed Index of Default Master"]
pub type FixedDefmstrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:8 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&self) -> SlotCycleR {
        SlotCycleR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&self) -> DefmstrTypeR {
        DefmstrTypeR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&self) -> FixedDefmstrR {
        FixedDefmstrR::new(((self.bits >> 18) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Number of Allowed Cycles for a Burst"]
    #[inline(always)]
    pub fn slot_cycle(&mut self) -> SlotCycleW<'_, ScfgSpec> {
        SlotCycleW::new(self, 0)
    }
    #[doc = "Bits 16:17 - Default Master Type"]
    #[inline(always)]
    pub fn defmstr_type(&mut self) -> DefmstrTypeW<'_, ScfgSpec> {
        DefmstrTypeW::new(self, 16)
    }
    #[doc = "Bits 18:21 - Fixed Index of Default Master"]
    #[inline(always)]
    pub fn fixed_defmstr(&mut self) -> FixedDefmstrW<'_, ScfgSpec> {
        FixedDefmstrW::new(self, 18)
    }
}
#[doc = "Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScfgSpec;
impl crate::RegisterSpec for ScfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scfg::R`](R) reader structure"]
impl crate::Readable for ScfgSpec {}
#[doc = "`write(|w| ..)` method takes [`scfg::W`](W) writer structure"]
impl crate::Writable for ScfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCFG[%s] to value 0x10"]
impl crate::Resettable for ScfgSpec {
    const RESET_VALUE: u32 = 0x10;
}
