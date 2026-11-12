#[doc = "Register `LPMCNTRL_PERIPHERAL_MODE` reader"]
pub type R = crate::R<LpmcntrlPeripheralModeSpec>;
#[doc = "Register `LPMCNTRL_PERIPHERAL_MODE` writer"]
pub type W = crate::W<LpmcntrlPeripheralModeSpec>;
#[doc = "Field `LPMXMT` reader - this bit is set by software to instruct the core to transition to the l1 state upon the receipt of the next lpm transaction."]
pub type LpmxmtR = crate::BitReader;
#[doc = "Field `LPMXMT` writer - this bit is set by software to instruct the core to transition to the l1 state upon the receipt of the next lpm transaction."]
pub type LpmxmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRES` reader - this bit is used by software to initiate resume (remote wakeup)."]
pub type LpmresR = crate::BitReader;
#[doc = "Field `LPMRES` writer - this bit is used by software to initiate resume (remote wakeup)."]
pub type LpmresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "this register is used to enable lpm in the musbmhdrc.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpmenselect {
    #[doc = "0: Core supports LPM extended transactions"]
    Lpmtrans = 0,
    #[doc = "1: LPM is not supported but extended transactions are supported."]
    Extntrans = 1,
    #[doc = "2: LPM and extended transactions are not supported"]
    Notsupported1 = 2,
    #[doc = "3: LPM and extended transactions are not supported"]
    Notsupported2 = 3,
}
impl From<Lpmenselect> for u8 {
    #[inline(always)]
    fn from(variant: Lpmenselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpmenselect {
    type Ux = u8;
}
impl crate::IsEnum for Lpmenselect {}
#[doc = "Field `LPMEN` reader - this register is used to enable lpm in the musbmhdrc."]
pub type LpmenR = crate::FieldReader<Lpmenselect>;
impl LpmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmenselect {
        match self.bits {
            0 => Lpmenselect::Lpmtrans,
            1 => Lpmenselect::Extntrans,
            2 => Lpmenselect::Notsupported1,
            3 => Lpmenselect::Notsupported2,
            _ => unreachable!(),
        }
    }
    #[doc = "Core supports LPM extended transactions"]
    #[inline(always)]
    pub fn is_lpmtrans(&self) -> bool {
        *self == Lpmenselect::Lpmtrans
    }
    #[doc = "LPM is not supported but extended transactions are supported."]
    #[inline(always)]
    pub fn is_extntrans(&self) -> bool {
        *self == Lpmenselect::Extntrans
    }
    #[doc = "LPM and extended transactions are not supported"]
    #[inline(always)]
    pub fn is_notsupported1(&self) -> bool {
        *self == Lpmenselect::Notsupported1
    }
    #[doc = "LPM and extended transactions are not supported"]
    #[inline(always)]
    pub fn is_notsupported2(&self) -> bool {
        *self == Lpmenselect::Notsupported2
    }
}
#[doc = "Field `LPMEN` writer - this register is used to enable lpm in the musbmhdrc."]
pub type LpmenW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lpmenselect, crate::Safe>;
impl<'a, REG> LpmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Core supports LPM extended transactions"]
    #[inline(always)]
    pub fn lpmtrans(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmenselect::Lpmtrans)
    }
    #[doc = "LPM is not supported but extended transactions are supported."]
    #[inline(always)]
    pub fn extntrans(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmenselect::Extntrans)
    }
    #[doc = "LPM and extended transactions are not supported"]
    #[inline(always)]
    pub fn notsupported1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmenselect::Notsupported1)
    }
    #[doc = "LPM and extended transactions are not supported"]
    #[inline(always)]
    pub fn notsupported2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmenselect::Notsupported2)
    }
}
#[doc = "Field `LPMNAK` reader - this bit is used to place all end points in a state such that the response to all transactions other then an lpm transaction will be a nak."]
pub type LpmnakR = crate::BitReader;
#[doc = "Field `LPMNAK` writer - this bit is used to place all end points in a state such that the response to all transactions other then an lpm transaction will be a nak."]
pub type LpmnakW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit is set by software to instruct the core to transition to the l1 state upon the receipt of the next lpm transaction."]
    #[inline(always)]
    pub fn lpmxmt(&self) -> LpmxmtR {
        LpmxmtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit is used by software to initiate resume (remote wakeup)."]
    #[inline(always)]
    pub fn lpmres(&self) -> LpmresR {
        LpmresR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - this register is used to enable lpm in the musbmhdrc."]
    #[inline(always)]
    pub fn lpmen(&self) -> LpmenR {
        LpmenR::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - this bit is used to place all end points in a state such that the response to all transactions other then an lpm transaction will be a nak."]
    #[inline(always)]
    pub fn lpmnak(&self) -> LpmnakR {
        LpmnakR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit is set by software to instruct the core to transition to the l1 state upon the receipt of the next lpm transaction."]
    #[inline(always)]
    pub fn lpmxmt(&mut self) -> LpmxmtW<'_, LpmcntrlPeripheralModeSpec> {
        LpmxmtW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit is used by software to initiate resume (remote wakeup)."]
    #[inline(always)]
    pub fn lpmres(&mut self) -> LpmresW<'_, LpmcntrlPeripheralModeSpec> {
        LpmresW::new(self, 1)
    }
    #[doc = "Bits 2:3 - this register is used to enable lpm in the musbmhdrc."]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LpmenW<'_, LpmcntrlPeripheralModeSpec> {
        LpmenW::new(self, 2)
    }
    #[doc = "Bit 4 - this bit is used to place all end points in a state such that the response to all transactions other then an lpm transaction will be a nak."]
    #[inline(always)]
    pub fn lpmnak(&mut self) -> LpmnakW<'_, LpmcntrlPeripheralModeSpec> {
        LpmnakW::new(self, 4)
    }
}
#[doc = "LPM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmcntrl_peripheral_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmcntrl_peripheral_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmcntrlPeripheralModeSpec;
impl crate::RegisterSpec for LpmcntrlPeripheralModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmcntrl_peripheral_mode::R`](R) reader structure"]
impl crate::Readable for LpmcntrlPeripheralModeSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmcntrl_peripheral_mode::W`](W) writer structure"]
impl crate::Writable for LpmcntrlPeripheralModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMCNTRL_PERIPHERAL_MODE to value 0"]
impl crate::Resettable for LpmcntrlPeripheralModeSpec {}
