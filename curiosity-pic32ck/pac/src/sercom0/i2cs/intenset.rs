#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `PREC` reader - Stop Received Interrupt Enable"]
pub type PrecR = crate::BitReader;
#[doc = "Field `PREC` writer - Stop Received Interrupt Enable"]
pub type PrecW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt Enable"]
pub type AmatchR = crate::BitReader;
#[doc = "Field `AMATCH` writer - Address Match Interrupt Enable"]
pub type AmatchW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DRDY` reader - Data Interrupt Enable"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `DRDY` writer - Data Interrupt Enable"]
pub type DrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `TXFE` reader - Tx FIFO Empty Interrupt Enable"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFE` writer - Tx FIFO Empty Interrupt Enable"]
pub type TxfeW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RXFF` reader - Rx FIFO Full Interrupt Enable"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFF` writer - Rx FIFO Full Interrupt Enable"]
pub type RxffW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Enable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Enable"]
pub type ErrorW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    pub fn prec(&self) -> PrecR {
        PrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn amatch(&self) -> AmatchR {
        AmatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt Enable"]
    #[inline(always)]
    pub fn prec(&mut self) -> PrecW<'_, IntensetSpec> {
        PrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Enable"]
    #[inline(always)]
    pub fn amatch(&mut self) -> AmatchW<'_, IntensetSpec> {
        AmatchW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Interrupt Enable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DrdyW<'_, IntensetSpec> {
        DrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt Enable"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TxfeW<'_, IntensetSpec> {
        TxfeW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt Enable"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RxffW<'_, IntensetSpec> {
        RxffW::new(self, 4)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Enable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntensetSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CS Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x9f;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
