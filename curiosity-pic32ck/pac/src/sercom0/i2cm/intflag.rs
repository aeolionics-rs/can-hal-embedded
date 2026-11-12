#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `MB` reader - Master On Bus Interrupt"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - Master On Bus Interrupt"]
pub type MbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SB` reader - Slave On Bus Interrupt"]
pub type SbR = crate::BitReader;
#[doc = "Field `SB` writer - Slave On Bus Interrupt"]
pub type SbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFE` reader - Tx FIFO Empty Interrupt"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFE` writer - Tx FIFO Empty Interrupt"]
pub type TxfeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFF` reader - Rx FIFO Full Interrupt"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFF` writer - Rx FIFO Full Interrupt"]
pub type RxffW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master On Bus Interrupt"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt"]
    #[inline(always)]
    pub fn sb(&self) -> SbR {
        SbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master On Bus Interrupt"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<'_, IntflagSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - Slave On Bus Interrupt"]
    #[inline(always)]
    pub fn sb(&mut self) -> SbW<'_, IntflagSpec> {
        SbW::new(self, 1)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TxfeW<'_, IntflagSpec> {
        TxfeW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RxffW<'_, IntflagSpec> {
        RxffW::new(self, 4)
    }
    #[doc = "Bit 7 - Combined Error Interrupt"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntflagSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CM Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x9b;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
