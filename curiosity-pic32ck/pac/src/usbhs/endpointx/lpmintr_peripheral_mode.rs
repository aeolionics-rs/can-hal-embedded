#[doc = "Register `LPMINTR_PERIPHERAL_MODE` reader"]
pub type R = crate::R<LpmintrPeripheralModeSpec>;
#[doc = "Register `LPMINTR_PERIPHERAL_MODE` writer"]
pub type W = crate::W<LpmintrPeripheralModeSpec>;
#[doc = "Field `LPMST` reader - this bit is set when an lpm transaction is received and the core responds with a stall."]
pub type LpmstR = crate::BitReader;
#[doc = "Field `LPMST` writer - this bit is set when an lpm transaction is received and the core responds with a stall."]
pub type LpmstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNY` reader - this bit is set when an lpm transaction is received and the core responds with a nyet"]
pub type LpmnyR = crate::BitReader;
#[doc = "Field `LPMNY` writer - this bit is set when an lpm transaction is received and the core responds with a nyet"]
pub type LpmnyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - this bit is set when an lpm transaction is received and the core responds with an ack."]
pub type LpmackR = crate::BitReader;
#[doc = "Field `LPMACK` writer - this bit is set when an lpm transaction is received and the core responds with an ack."]
pub type LpmackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMNC` reader - this bit is set when an lpm transaction is received and the core responds with a nyet due to data pending in the rx fifos."]
pub type LpmncR = crate::BitReader;
#[doc = "Field `LPMNC` writer - this bit is set when an lpm transaction is received and the core responds with a nyet due to data pending in the rx fifos."]
pub type LpmncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRES` reader - this bit is set if the core has been resumed for any reason."]
pub type LpmresR = crate::BitReader;
#[doc = "Field `LPMRES` writer - this bit is set if the core has been resumed for any reason."]
pub type LpmresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMERR` reader - this bit is set if an lpm transaction is received that has a linkstate field that is not supported"]
pub type LpmerrR = crate::BitReader;
#[doc = "Field `LPMERR` writer - this bit is set if an lpm transaction is received that has a linkstate field that is not supported"]
pub type LpmerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit is set when an lpm transaction is received and the core responds with a stall."]
    #[inline(always)]
    pub fn lpmst(&self) -> LpmstR {
        LpmstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit is set when an lpm transaction is received and the core responds with a nyet"]
    #[inline(always)]
    pub fn lpmny(&self) -> LpmnyR {
        LpmnyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit is set when an lpm transaction is received and the core responds with an ack."]
    #[inline(always)]
    pub fn lpmack(&self) -> LpmackR {
        LpmackR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit is set when an lpm transaction is received and the core responds with a nyet due to data pending in the rx fifos."]
    #[inline(always)]
    pub fn lpmnc(&self) -> LpmncR {
        LpmncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - this bit is set if the core has been resumed for any reason."]
    #[inline(always)]
    pub fn lpmres(&self) -> LpmresR {
        LpmresR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - this bit is set if an lpm transaction is received that has a linkstate field that is not supported"]
    #[inline(always)]
    pub fn lpmerr(&self) -> LpmerrR {
        LpmerrR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - this bit is set when an lpm transaction is received and the core responds with a stall."]
    #[inline(always)]
    pub fn lpmst(&mut self) -> LpmstW<'_, LpmintrPeripheralModeSpec> {
        LpmstW::new(self, 0)
    }
    #[doc = "Bit 1 - this bit is set when an lpm transaction is received and the core responds with a nyet"]
    #[inline(always)]
    pub fn lpmny(&mut self) -> LpmnyW<'_, LpmintrPeripheralModeSpec> {
        LpmnyW::new(self, 1)
    }
    #[doc = "Bit 2 - this bit is set when an lpm transaction is received and the core responds with an ack."]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LpmackW<'_, LpmintrPeripheralModeSpec> {
        LpmackW::new(self, 2)
    }
    #[doc = "Bit 3 - this bit is set when an lpm transaction is received and the core responds with a nyet due to data pending in the rx fifos."]
    #[inline(always)]
    pub fn lpmnc(&mut self) -> LpmncW<'_, LpmintrPeripheralModeSpec> {
        LpmncW::new(self, 3)
    }
    #[doc = "Bit 4 - this bit is set if the core has been resumed for any reason."]
    #[inline(always)]
    pub fn lpmres(&mut self) -> LpmresW<'_, LpmintrPeripheralModeSpec> {
        LpmresW::new(self, 4)
    }
    #[doc = "Bit 5 - this bit is set if an lpm transaction is received that has a linkstate field that is not supported"]
    #[inline(always)]
    pub fn lpmerr(&mut self) -> LpmerrW<'_, LpmintrPeripheralModeSpec> {
        LpmerrW::new(self, 5)
    }
}
#[doc = "Status Of The LPM Power State\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmintr_peripheral_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmintr_peripheral_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmintrPeripheralModeSpec;
impl crate::RegisterSpec for LpmintrPeripheralModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lpmintr_peripheral_mode::R`](R) reader structure"]
impl crate::Readable for LpmintrPeripheralModeSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmintr_peripheral_mode::W`](W) writer structure"]
impl crate::Writable for LpmintrPeripheralModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMINTR_PERIPHERAL_MODE to value 0"]
impl crate::Resettable for LpmintrPeripheralModeSpec {}
