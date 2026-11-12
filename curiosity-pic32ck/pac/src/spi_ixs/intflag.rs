#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SPIRXBF` reader - SPI Receive Buffer Full Status Bit"]
pub type SpirxbfR = crate::BitReader;
#[doc = "Field `SPIRXBF` writer - SPI Receive Buffer Full Status Bit"]
pub type SpirxbfW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPITXBE` reader - SPI Transmit Buffer Empty Status Bit"]
pub type SpitxbeR = crate::BitReader;
#[doc = "Field `SPITXBE` writer - SPI Transmit Buffer Empty Status Bit"]
pub type SpitxbeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FRMERR` reader - SPI Frame Error Status bit"]
pub type FrmerrR = crate::BitReader;
#[doc = "Field `FRMERR` writer - SPI Frame Error Status bit"]
pub type FrmerrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPITUR` reader - Transmit Underrun Status bit"]
pub type SpiturR = crate::BitReader;
#[doc = "Field `SPITUR` writer - Transmit Underrun Status bit"]
pub type SpiturW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SPIROV` reader - Receive Overflow Status bit"]
pub type SpirovR = crate::BitReader;
#[doc = "Field `SPIROV` writer - Receive Overflow Status bit"]
pub type SpirovW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - SPI Receive Buffer Full Status Bit"]
    #[inline(always)]
    pub fn spirxbf(&self) -> SpirxbfR {
        SpirxbfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Transmit Buffer Empty Status Bit"]
    #[inline(always)]
    pub fn spitxbe(&self) -> SpitxbeR {
        SpitxbeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI Frame Error Status bit"]
    #[inline(always)]
    pub fn frmerr(&self) -> FrmerrR {
        FrmerrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Underrun Status bit"]
    #[inline(always)]
    pub fn spitur(&self) -> SpiturR {
        SpiturR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Overflow Status bit"]
    #[inline(always)]
    pub fn spirov(&self) -> SpirovR {
        SpirovR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Receive Buffer Full Status Bit"]
    #[inline(always)]
    pub fn spirxbf(&mut self) -> SpirxbfW<'_, IntflagSpec> {
        SpirxbfW::new(self, 0)
    }
    #[doc = "Bit 4 - SPI Transmit Buffer Empty Status Bit"]
    #[inline(always)]
    pub fn spitxbe(&mut self) -> SpitxbeW<'_, IntflagSpec> {
        SpitxbeW::new(self, 4)
    }
    #[doc = "Bit 15 - SPI Frame Error Status bit"]
    #[inline(always)]
    pub fn frmerr(&mut self) -> FrmerrW<'_, IntflagSpec> {
        FrmerrW::new(self, 15)
    }
    #[doc = "Bit 27 - Transmit Underrun Status bit"]
    #[inline(always)]
    pub fn spitur(&mut self) -> SpiturW<'_, IntflagSpec> {
        SpiturW::new(self, 27)
    }
    #[doc = "Bit 30 - Receive Overflow Status bit"]
    #[inline(always)]
    pub fn spirov(&mut self) -> SpirovW<'_, IntflagSpec> {
        SpirovW::new(self, 30)
    }
}
#[doc = "SPI Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x4800_8011;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
