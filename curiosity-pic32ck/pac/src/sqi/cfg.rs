#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `MODE` reader - Mode Select The default value of this field is 0. After power on reset, the controller enters boot mode. 000 - Boot mode. The CSR registers are loaded with boot strap values. 001 - PIO mode. The controller is controlled by the CPU in PIO mode. 010 - DMA mode. 011 - XIP mode. When this mode is entered the controller behaves as if executing in Place (XIP) but uses the register data to control timing. 100 - 111 - Reserved"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - Mode Select The default value of this field is 0. After power on reset, the controller enters boot mode. 000 - Boot mode. The CSR registers are loaded with boot strap values. 001 - PIO mode. The controller is controlled by the CPU in PIO mode. 010 - DMA mode. 011 - XIP mode. When this mode is entered the controller behaves as if executing in Place (XIP) but uses the register data to control timing. 100 - 111 - Reserved"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CPHA` reader - Clock phase (selects the transfer format) 0 = SPICLK starts toggling at the middle of 1st data bit. 1 = SPICLK starts toggling at the start of 1st data bit."]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase (selects the transfer format) 0 = SPICLK starts toggling at the middle of 1st data bit. 1 = SPICLK starts toggling at the start of 1st data bit."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity 0 = active-high SPICLK (SPICLK low is the idle state) 1 = active-low SPICLK (SPICLK high is the idle state)"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity 0 = active-high SPICLK (SPICLK low is the idle state) 1 = active-low SPICLK (SPICLK high is the idle state)"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBF` reader - Data format on SPI interface 0 = MSBit sent/received first. 1 = LSBit sent/received first. This setting is supeceded by LSBF control bit in Buffer Descriptor during DMA operations."]
pub type LsbfR = crate::BitReader;
#[doc = "Field `LSBF` writer - Data format on SPI interface 0 = MSBit sent/received first. 1 = LSBit sent/received first. This setting is supeceded by LSBF control bit in Buffer Descriptor during DMA operations."]
pub type LsbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WP` reader - Write Protect: In single or Dual lane mode, this bit is used to drive the spiout2 signal. Whenever this bit is high, the controller drives 0 on spiout2 line and sets spimoe2 high (active only during transfer)."]
pub type WpR = crate::BitReader;
#[doc = "Field `WP` writer - Write Protect: In single or Dual lane mode, this bit is used to drive the spiout2 signal. Whenever this bit is high, the controller drives 0 on spiout2 line and sets spimoe2 high (active only during transfer)."]
pub type WpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOLD` reader - Hold: In Single or Dual lane mode, this bit is used to drive the spiout3 signal. Whenever this bit is high, the controller drives 0 on spiout3 line and sets spimoe3 high (active only during transfer)."]
pub type HoldR = crate::BitReader;
#[doc = "Field `HOLD` writer - Hold: In Single or Dual lane mode, this bit is used to drive the spiout3 signal. Whenever this bit is high, the controller drives 0 on spiout3 line and sets spimoe3 high (active only during transfer)."]
pub type HoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURSTEN` reader - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR is enabled 0 - AHB INCR is disabled"]
pub type BurstenR = crate::BitReader;
#[doc = "Field `BURSTEN` writer - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR is enabled 0 - AHB INCR is disabled"]
pub type BurstenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_BURST_INCR4_EN` reader - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR4 is enabled. 0 - AHB INCR4 is disabled"]
pub type AhbBurstIncr4EnR = crate::BitReader;
#[doc = "Field `AHB_BURST_INCR4_EN` writer - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR4 is enabled. 0 - AHB INCR4 is disabled"]
pub type AhbBurstIncr4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_BURST_INCR8_EN` reader - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR8 is enabled. 0 - AHB INCR8 is disabled"]
pub type AhbBurstIncr8EnR = crate::BitReader;
#[doc = "Field `AHB_BURST_INCR8_EN` writer - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR8 is enabled. 0 - AHB INCR8 is disabled"]
pub type AhbBurstIncr8EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHB_BURST_INCR16_EN` reader - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR16 is enabled. 0 - AHB INCR16 is disabled"]
pub type AhbBurstIncr16EnR = crate::BitReader;
#[doc = "Field `AHB_BURST_INCR16_EN` writer - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR16 is enabled. 0 - AHB INCR16 is disabled"]
pub type AhbBurstIncr16EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Software Reset Select Bit"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Software Reset Select Bit"]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBUFRST` reader - Transmit FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Transmit FIFO pointers will get reset by this reset pulse. Note: Transmit FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only.Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type TxbufrstR = crate::BitReader;
#[doc = "Field `TXBUFRST` writer - Transmit FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Transmit FIFO pointers will get reset by this reset pulse. Note: Transmit FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only.Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type TxbufrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBUFRST` reader - Receive FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Receive fifo pointers will get reset by this reset pulse. Note: Receive FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type RxbufrstR = crate::BitReader;
#[doc = "Field `RXBUFRST` writer - Receive FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Receive fifo pointers will get reset by this reset pulse. Note: Receive FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type RxbufrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONBUFRST` reader - Control FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Control fifo pointers will get reset by this reset pulse. Note: Control FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type ConbufrstR = crate::BitReader;
#[doc = "Field `CONBUFRST` writer - Control FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Control fifo pointers will get reset by this reset pulse. Note: Control FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
pub type ConbufrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAEN` reader - Max Data Lanes 11 - Reserved, 10 - 4, 01 - 2, 00 - 1 SPI_OUT_PIN_EN\\[3:0\\] output pins are controlled by writing to these bits."]
pub type DataenR = crate::FieldReader;
#[doc = "Field `DATAEN` writer - Max Data Lanes 11 - Reserved, 10 - 4, 01 - 2, 00 - 1 SPI_OUT_PIN_EN\\[3:0\\] output pins are controlled by writing to these bits."]
pub type DataenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SQIEN` reader - 1 - Enabled, 0 - Disabled SPI_ON output pin is controlled by writing to this bit."]
pub type SqienR = crate::BitReader;
#[doc = "Field `SQIEN` writer - 1 - Enabled, 0 - Disabled SPI_ON output pin is controlled by writing to this bit."]
pub type SqienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSEN` reader - 1 - Chip Select is used 0 - Chip Select is not used SPI_CS\\[7\\] for CS7, SPI_CS\\[6\\] for CS6, SPI_CS\\[5\\] for CS5, SPI_CS\\[4\\] for CS4, SPI_CS\\[3\\] for CS3, SPI_CS\\[2\\] for CS2, SPI_CS\\[1\\] for CS1, SPI_CS\\[0\\] for CS0. SPI_CS_PIN_EN\\[7:0/3:0\\] output pins are controlled by writing to these bits."]
pub type CsenR = crate::FieldReader;
#[doc = "Field `CSEN` writer - 1 - Chip Select is used 0 - Chip Select is not used SPI_CS\\[7\\] for CS7, SPI_CS\\[6\\] for CS6, SPI_CS\\[5\\] for CS5, SPI_CS\\[4\\] for CS4, SPI_CS\\[3\\] for CS3, SPI_CS\\[2\\] for CS2, SPI_CS\\[1\\] for CS1, SPI_CS\\[0\\] for CS0. SPI_CS_PIN_EN\\[7:0/3:0\\] output pins are controlled by writing to these bits."]
pub type CsenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - Mode Select The default value of this field is 0. After power on reset, the controller enters boot mode. 000 - Boot mode. The CSR registers are loaded with boot strap values. 001 - PIO mode. The controller is controlled by the CPU in PIO mode. 010 - DMA mode. 011 - XIP mode. When this mode is entered the controller behaves as if executing in Place (XIP) but uses the register data to control timing. 100 - 111 - Reserved"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Clock phase (selects the transfer format) 0 = SPICLK starts toggling at the middle of 1st data bit. 1 = SPICLK starts toggling at the start of 1st data bit."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock polarity 0 = active-high SPICLK (SPICLK low is the idle state) 1 = active-low SPICLK (SPICLK high is the idle state)"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data format on SPI interface 0 = MSBit sent/received first. 1 = LSBit sent/received first. This setting is supeceded by LSBF control bit in Buffer Descriptor during DMA operations."]
    #[inline(always)]
    pub fn lsbf(&self) -> LsbfR {
        LsbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protect: In single or Dual lane mode, this bit is used to drive the spiout2 signal. Whenever this bit is high, the controller drives 0 on spiout2 line and sets spimoe2 high (active only during transfer)."]
    #[inline(always)]
    pub fn wp(&self) -> WpR {
        WpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Hold: In Single or Dual lane mode, this bit is used to drive the spiout3 signal. Whenever this bit is high, the controller drives 0 on spiout3 line and sets spimoe3 high (active only during transfer)."]
    #[inline(always)]
    pub fn hold(&self) -> HoldR {
        HoldR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR is enabled 0 - AHB INCR is disabled"]
    #[inline(always)]
    pub fn bursten(&self) -> BurstenR {
        BurstenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR4 is enabled. 0 - AHB INCR4 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr4_en(&self) -> AhbBurstIncr4EnR {
        AhbBurstIncr4EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR8 is enabled. 0 - AHB INCR8 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr8_en(&self) -> AhbBurstIncr8EnR {
        AhbBurstIncr8EnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR16 is enabled. 0 - AHB INCR16 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr16_en(&self) -> AhbBurstIncr16EnR {
        AhbBurstIncr16EnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Software Reset Select Bit"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Transmit FIFO pointers will get reset by this reset pulse. Note: Transmit FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only.Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn txbufrst(&self) -> TxbufrstR {
        TxbufrstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Receive FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Receive fifo pointers will get reset by this reset pulse. Note: Receive FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn rxbufrst(&self) -> RxbufrstR {
        RxbufrstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Control FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Control fifo pointers will get reset by this reset pulse. Note: Control FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn conbufrst(&self) -> ConbufrstR {
        ConbufrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Max Data Lanes 11 - Reserved, 10 - 4, 01 - 2, 00 - 1 SPI_OUT_PIN_EN\\[3:0\\] output pins are controlled by writing to these bits."]
    #[inline(always)]
    pub fn dataen(&self) -> DataenR {
        DataenR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 23 - 1 - Enabled, 0 - Disabled SPI_ON output pin is controlled by writing to this bit."]
    #[inline(always)]
    pub fn sqien(&self) -> SqienR {
        SqienR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 1 - Chip Select is used 0 - Chip Select is not used SPI_CS\\[7\\] for CS7, SPI_CS\\[6\\] for CS6, SPI_CS\\[5\\] for CS5, SPI_CS\\[4\\] for CS4, SPI_CS\\[3\\] for CS3, SPI_CS\\[2\\] for CS2, SPI_CS\\[1\\] for CS1, SPI_CS\\[0\\] for CS0. SPI_CS_PIN_EN\\[7:0/3:0\\] output pins are controlled by writing to these bits."]
    #[inline(always)]
    pub fn csen(&self) -> CsenR {
        CsenR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Mode Select The default value of this field is 0. After power on reset, the controller enters boot mode. 000 - Boot mode. The CSR registers are loaded with boot strap values. 001 - PIO mode. The controller is controlled by the CPU in PIO mode. 010 - DMA mode. 011 - XIP mode. When this mode is entered the controller behaves as if executing in Place (XIP) but uses the register data to control timing. 100 - 111 - Reserved"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CfgSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bit 3 - Clock phase (selects the transfer format) 0 = SPICLK starts toggling at the middle of 1st data bit. 1 = SPICLK starts toggling at the start of 1st data bit."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, CfgSpec> {
        CphaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock polarity 0 = active-high SPICLK (SPICLK low is the idle state) 1 = active-low SPICLK (SPICLK high is the idle state)"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CfgSpec> {
        CpolW::new(self, 4)
    }
    #[doc = "Bit 5 - Data format on SPI interface 0 = MSBit sent/received first. 1 = LSBit sent/received first. This setting is supeceded by LSBF control bit in Buffer Descriptor during DMA operations."]
    #[inline(always)]
    pub fn lsbf(&mut self) -> LsbfW<'_, CfgSpec> {
        LsbfW::new(self, 5)
    }
    #[doc = "Bit 9 - Write Protect: In single or Dual lane mode, this bit is used to drive the spiout2 signal. Whenever this bit is high, the controller drives 0 on spiout2 line and sets spimoe2 high (active only during transfer)."]
    #[inline(always)]
    pub fn wp(&mut self) -> WpW<'_, CfgSpec> {
        WpW::new(self, 9)
    }
    #[doc = "Bit 10 - Hold: In Single or Dual lane mode, this bit is used to drive the spiout3 signal. Whenever this bit is high, the controller drives 0 on spiout3 line and sets spimoe3 high (active only during transfer)."]
    #[inline(always)]
    pub fn hold(&mut self) -> HoldW<'_, CfgSpec> {
        HoldW::new(self, 10)
    }
    #[doc = "Bit 11 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR is enabled 0 - AHB INCR is disabled"]
    #[inline(always)]
    pub fn bursten(&mut self) -> BurstenW<'_, CfgSpec> {
        BurstenW::new(self, 11)
    }
    #[doc = "Bit 12 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR4 is enabled. 0 - AHB INCR4 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr4_en(&mut self) -> AhbBurstIncr4EnW<'_, CfgSpec> {
        AhbBurstIncr4EnW::new(self, 12)
    }
    #[doc = "Bit 13 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR8 is enabled. 0 - AHB INCR8 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr8_en(&mut self) -> AhbBurstIncr8EnW<'_, CfgSpec> {
        AhbBurstIncr8EnW::new(self, 13)
    }
    #[doc = "Bit 14 - This field is used to configure the AHB Master Burst capability. 1 - AHB INCR16 is enabled. 0 - AHB INCR16 is disabled"]
    #[inline(always)]
    pub fn ahb_burst_incr16_en(&mut self) -> AhbBurstIncr16EnW<'_, CfgSpec> {
        AhbBurstIncr16EnW::new(self, 14)
    }
    #[doc = "Bit 16 - Software Reset Select Bit"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CfgSpec> {
        ResetW::new(self, 16)
    }
    #[doc = "Bit 17 - Transmit FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Transmit FIFO pointers will get reset by this reset pulse. Note: Transmit FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only.Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn txbufrst(&mut self) -> TxbufrstW<'_, CfgSpec> {
        TxbufrstW::new(self, 17)
    }
    #[doc = "Bit 18 - Receive FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Receive fifo pointers will get reset by this reset pulse. Note: Receive FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn rxbufrst(&mut self) -> RxbufrstW<'_, CfgSpec> {
        RxbufrstW::new(self, 18)
    }
    #[doc = "Bit 19 - Control FIFO Reset A reset pulse is generated when writing 1 to this bit. This bit is Auto clear and SPI Controller will clear this bit. Control fifo pointers will get reset by this reset pulse. Note: Control FIFO reset can be done only after a transaction is complete on spi bus; however if we want to reset FIFO during transaction, we need to go for soft reset only. Note: For Michigan Ax versions, treat as Reserved, keep 0."]
    #[inline(always)]
    pub fn conbufrst(&mut self) -> ConbufrstW<'_, CfgSpec> {
        ConbufrstW::new(self, 19)
    }
    #[doc = "Bits 20:21 - Max Data Lanes 11 - Reserved, 10 - 4, 01 - 2, 00 - 1 SPI_OUT_PIN_EN\\[3:0\\] output pins are controlled by writing to these bits."]
    #[inline(always)]
    pub fn dataen(&mut self) -> DataenW<'_, CfgSpec> {
        DataenW::new(self, 20)
    }
    #[doc = "Bit 23 - 1 - Enabled, 0 - Disabled SPI_ON output pin is controlled by writing to this bit."]
    #[inline(always)]
    pub fn sqien(&mut self) -> SqienW<'_, CfgSpec> {
        SqienW::new(self, 23)
    }
    #[doc = "Bits 24:31 - 1 - Chip Select is used 0 - Chip Select is not used SPI_CS\\[7\\] for CS7, SPI_CS\\[6\\] for CS6, SPI_CS\\[5\\] for CS5, SPI_CS\\[4\\] for CS4, SPI_CS\\[3\\] for CS3, SPI_CS\\[2\\] for CS2, SPI_CS\\[1\\] for CS1, SPI_CS\\[0\\] for CS0. SPI_CS_PIN_EN\\[7:0/3:0\\] output pins are controlled by writing to these bits."]
    #[inline(always)]
    pub fn csen(&mut self) -> CsenW<'_, CfgSpec> {
        CsenW::new(self, 24)
    }
}
#[doc = "SPI Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0x0100_7e01"]
impl crate::Resettable for CfgSpec {
    const RESET_VALUE: u32 = 0x0100_7e01;
}
