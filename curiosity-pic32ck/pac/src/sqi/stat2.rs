#[doc = "Register `STAT2` reader"]
pub type R = crate::R<Stat2Spec>;
#[doc = "Field `TXOV` reader - This bit would get set when TX FIFO OVERFLOW happens. S/W needs to monitor TX_FIFO_FREE when filling TX_FIFO to make sure TX FIFO doesn?t over flow or in case OVERFLOW bit gets set, then TX_FIFO_RST (bit 17 of SPI Configuration register) should be set which would then clear TX FIFO pointers and hence this bit would get cleared. Generally software should avoid getting into this condition."]
pub type TxovR = crate::BitReader;
#[doc = "Field `RXUN` reader - This bit would get set when RX FIFO UNDERFLOW happens. S/W needs to monitor RX_FIFO_CNT when emptying RX_FIFO to make sure RX FIFO doesn?t under flow or in case UNDERFLOW bit gets set, then RX_FIFO_RST (bit 18 of SPI Configuration register) must be set which would then clear RX FIFO pointers and hence this bit would get cleared. Generally software should avoid getting into this condition."]
pub type RxunR = crate::BitReader;
#[doc = "Field `SQID0` reader - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
pub type Sqid0R = crate::BitReader;
#[doc = "Field `SQID1` reader - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
pub type Sqid1R = crate::BitReader;
#[doc = "Field `SQID2` reader - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
pub type Sqid2R = crate::BitReader;
#[doc = "Field `SQID3` reader - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
pub type Sqid3R = crate::BitReader;
#[doc = "Field `CONAVAIL` reader - Indicates the number of entries in Control Buffer remaining. NOTE: This field not present in Michigan Ax versions. Treat as Reserved."]
pub type ConavailR = crate::FieldReader;
#[doc = "Field `CMDSTAT` reader - Reflect the (internal state machine) CMD_INIT value. Indicates whether the controller is in Transmit/Receive/IDLE state. 2?b00 - IDLE ,2?b01 - Transmit ,2?b10 - Receive ,2?b11 - Reserved"]
pub type CmdstatR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit would get set when TX FIFO OVERFLOW happens. S/W needs to monitor TX_FIFO_FREE when filling TX_FIFO to make sure TX FIFO doesn?t over flow or in case OVERFLOW bit gets set, then TX_FIFO_RST (bit 17 of SPI Configuration register) should be set which would then clear TX FIFO pointers and hence this bit would get cleared. Generally software should avoid getting into this condition."]
    #[inline(always)]
    pub fn txov(&self) -> TxovR {
        TxovR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit would get set when RX FIFO UNDERFLOW happens. S/W needs to monitor RX_FIFO_CNT when emptying RX_FIFO to make sure RX FIFO doesn?t under flow or in case UNDERFLOW bit gets set, then RX_FIFO_RST (bit 18 of SPI Configuration register) must be set which would then clear RX FIFO pointers and hence this bit would get cleared. Generally software should avoid getting into this condition."]
    #[inline(always)]
    pub fn rxun(&self) -> RxunR {
        RxunR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
    #[inline(always)]
    pub fn sqid0(&self) -> Sqid0R {
        Sqid0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
    #[inline(always)]
    pub fn sqid1(&self) -> Sqid1R {
        Sqid1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
    #[inline(always)]
    pub fn sqid2(&self) -> Sqid2R {
        Sqid2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI Data bus: This status is used to check the SPI DATA line level for Debugging. Bit3 - SPI_IO0 ,Bit4 - SPI_IO1 ,Bit5 - SPI_IO2 ,Bit6 - SPI_IO3"]
    #[inline(always)]
    pub fn sqid3(&self) -> Sqid3R {
        Sqid3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - Indicates the number of entries in Control Buffer remaining. NOTE: This field not present in Michigan Ax versions. Treat as Reserved."]
    #[inline(always)]
    pub fn conavail(&self) -> ConavailR {
        ConavailR::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Reflect the (internal state machine) CMD_INIT value. Indicates whether the controller is in Transmit/Receive/IDLE state. 2?b00 - IDLE ,2?b01 - Transmit ,2?b10 - Receive ,2?b11 - Reserved"]
    #[inline(always)]
    pub fn cmdstat(&self) -> CmdstatR {
        CmdstatR::new(((self.bits >> 16) & 3) as u8)
    }
}
#[doc = "SPI status 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`stat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Stat2Spec;
impl crate::RegisterSpec for Stat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat2::R`](R) reader structure"]
impl crate::Readable for Stat2Spec {}
#[doc = "`reset()` method sets STAT2 to value 0"]
impl crate::Resettable for Stat2Spec {}
