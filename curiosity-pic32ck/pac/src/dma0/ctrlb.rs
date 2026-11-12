#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Priority Group 1 Quality Of Service Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos1select {
    #[doc = "0: QoS level is 0 (lowest)"]
    Level0 = 0,
    #[doc = "1: QoS level is 1 (lower)"]
    Level1 = 1,
    #[doc = "2: QoS level is 2 (medium)"]
    Level2 = 2,
    #[doc = "3: QoS level is 3 (high)"]
    Level3 = 3,
}
impl From<Qos1select> for u8 {
    #[inline(always)]
    fn from(variant: Qos1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos1select {
    type Ux = u8;
}
impl crate::IsEnum for Qos1select {}
#[doc = "Field `QOS1` reader - Priority Group 1 Quality Of Service Control"]
pub type Qos1R = crate::FieldReader<Qos1select>;
impl Qos1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos1select {
        match self.bits {
            0 => Qos1select::Level0,
            1 => Qos1select::Level1,
            2 => Qos1select::Level2,
            3 => Qos1select::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == Qos1select::Level0
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Qos1select::Level1
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == Qos1select::Level2
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == Qos1select::Level3
    }
}
#[doc = "Field `QOS1` writer - Priority Group 1 Quality Of Service Control"]
pub type Qos1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos1select, crate::Safe>;
impl<'a, REG> Qos1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Level0)
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Level1)
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Level2)
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut crate::W<REG> {
        self.variant(Qos1select::Level3)
    }
}
#[doc = "Priority Group 2 Quality Of Service Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos2select {
    #[doc = "0: QoS level is 0 (lowest)"]
    Level0 = 0,
    #[doc = "1: QoS level is 1 (lower)"]
    Level1 = 1,
    #[doc = "2: QoS level is 2 (medium)"]
    Level2 = 2,
    #[doc = "3: QoS level is 3 (high)"]
    Level3 = 3,
}
impl From<Qos2select> for u8 {
    #[inline(always)]
    fn from(variant: Qos2select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos2select {
    type Ux = u8;
}
impl crate::IsEnum for Qos2select {}
#[doc = "Field `QOS2` reader - Priority Group 2 Quality Of Service Control"]
pub type Qos2R = crate::FieldReader<Qos2select>;
impl Qos2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos2select {
        match self.bits {
            0 => Qos2select::Level0,
            1 => Qos2select::Level1,
            2 => Qos2select::Level2,
            3 => Qos2select::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == Qos2select::Level0
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Qos2select::Level1
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == Qos2select::Level2
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == Qos2select::Level3
    }
}
#[doc = "Field `QOS2` writer - Priority Group 2 Quality Of Service Control"]
pub type Qos2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos2select, crate::Safe>;
impl<'a, REG> Qos2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Level0)
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Level1)
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Level2)
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut crate::W<REG> {
        self.variant(Qos2select::Level3)
    }
}
#[doc = "Priority Group 3 Quality Of Service Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Qos3select {
    #[doc = "0: QoS level is 0 (lowest)"]
    Level0 = 0,
    #[doc = "1: QoS level is 1 (lower)"]
    Level1 = 1,
    #[doc = "2: QoS level is 2 (medium)"]
    Level2 = 2,
    #[doc = "3: QoS level is 3 (high)"]
    Level3 = 3,
}
impl From<Qos3select> for u8 {
    #[inline(always)]
    fn from(variant: Qos3select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Qos3select {
    type Ux = u8;
}
impl crate::IsEnum for Qos3select {}
#[doc = "Field `QOS3` reader - Priority Group 3 Quality Of Service Control"]
pub type Qos3R = crate::FieldReader<Qos3select>;
impl Qos3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Qos3select {
        match self.bits {
            0 => Qos3select::Level0,
            1 => Qos3select::Level1,
            2 => Qos3select::Level2,
            3 => Qos3select::Level3,
            _ => unreachable!(),
        }
    }
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn is_level_0(&self) -> bool {
        *self == Qos3select::Level0
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn is_level_1(&self) -> bool {
        *self == Qos3select::Level1
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn is_level_2(&self) -> bool {
        *self == Qos3select::Level2
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn is_level_3(&self) -> bool {
        *self == Qos3select::Level3
    }
}
#[doc = "Field `QOS3` writer - Priority Group 3 Quality Of Service Control"]
pub type Qos3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Qos3select, crate::Safe>;
impl<'a, REG> Qos3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "QoS level is 0 (lowest)"]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Level0)
    }
    #[doc = "QoS level is 1 (lower)"]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Level1)
    }
    #[doc = "QoS level is 2 (medium)"]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Level2)
    }
    #[doc = "QoS level is 3 (high)"]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut crate::W<REG> {
        self.variant(Qos3select::Level3)
    }
}
impl R {
    #[doc = "Bits 0:1 - Priority Group 1 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos1(&self) -> Qos1R {
        Qos1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Priority Group 2 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos2(&self) -> Qos2R {
        Qos2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Priority Group 3 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos3(&self) -> Qos3R {
        Qos3R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Priority Group 1 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos1(&mut self) -> Qos1W<'_, CtrlbSpec> {
        Qos1W::new(self, 0)
    }
    #[doc = "Bits 8:9 - Priority Group 2 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos2(&mut self) -> Qos2W<'_, CtrlbSpec> {
        Qos2W::new(self, 8)
    }
    #[doc = "Bits 16:17 - Priority Group 3 Quality Of Service Control"]
    #[inline(always)]
    pub fn qos3(&mut self) -> Qos3W<'_, CtrlbSpec> {
        Qos3W::new(self, 16)
    }
}
#[doc = "DMA CONTROL B REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CtrlbSpec {}
