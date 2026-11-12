#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: USART with external clock"]
    UsartExtClk = 0,
    #[doc = "1: USART with internal clock"]
    UsartIntClk = 1,
    #[doc = "2: SPI in slave operation"]
    SpiSlave = 2,
    #[doc = "3: SPI in master operation"]
    SpiMaster = 3,
    #[doc = "4: I2C slave operation"]
    I2cSlave = 4,
    #[doc = "5: I2C master operation"]
    I2cMaster = 5,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::UsartExtClk),
            1 => Some(Modeselect::UsartIntClk),
            2 => Some(Modeselect::SpiSlave),
            3 => Some(Modeselect::SpiMaster),
            4 => Some(Modeselect::I2cSlave),
            5 => Some(Modeselect::I2cMaster),
            _ => None,
        }
    }
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn is_usart_ext_clk(&self) -> bool {
        *self == Modeselect::UsartExtClk
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn is_usart_int_clk(&self) -> bool {
        *self == Modeselect::UsartIntClk
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn is_spi_slave(&self) -> bool {
        *self == Modeselect::SpiSlave
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        *self == Modeselect::SpiMaster
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn is_i2c_slave(&self) -> bool {
        *self == Modeselect::I2cSlave
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        *self == Modeselect::I2cMaster
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART with external clock"]
    #[inline(always)]
    pub fn usart_ext_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartExtClk)
    }
    #[doc = "USART with internal clock"]
    #[inline(always)]
    pub fn usart_int_clk(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::UsartIntClk)
    }
    #[doc = "SPI in slave operation"]
    #[inline(always)]
    pub fn spi_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiSlave)
    }
    #[doc = "SPI in master operation"]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::SpiMaster)
    }
    #[doc = "I2C slave operation"]
    #[inline(always)]
    pub fn i2c_slave(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cSlave)
    }
    #[doc = "I2C master operation"]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::I2cMaster)
    }
}
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBON` reader - Immediate Buffer Overflow Notification"]
pub type IbonR = crate::BitReader;
#[doc = "Field `IBON` writer - Immediate Buffer Overflow Notification"]
pub type IbonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Transmit Data Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txinvselect {
    #[doc = "0: TxD is not inverted"]
    Disable = 0,
    #[doc = "1: TxD is inverted"]
    Inv = 1,
}
impl From<Txinvselect> for bool {
    #[inline(always)]
    fn from(variant: Txinvselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Invert"]
pub type TxinvR = crate::BitReader<Txinvselect>;
impl TxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txinvselect {
        match self.bits {
            false => Txinvselect::Disable,
            true => Txinvselect::Inv,
        }
    }
    #[doc = "TxD is not inverted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txinvselect::Disable
    }
    #[doc = "TxD is inverted"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Txinvselect::Inv
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Invert"]
pub type TxinvW<'a, REG> = crate::BitWriter<'a, REG, Txinvselect>;
impl<'a, REG> TxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TxD is not inverted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txinvselect::Disable)
    }
    #[doc = "TxD is inverted"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Txinvselect::Inv)
    }
}
#[doc = "Receive Data Invert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxinvselect {
    #[doc = "0: RxD is not inverted"]
    Disable = 0,
    #[doc = "1: RxD is inverted"]
    Inv = 1,
}
impl From<Rxinvselect> for bool {
    #[inline(always)]
    fn from(variant: Rxinvselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXINV` reader - Receive Data Invert"]
pub type RxinvR = crate::BitReader<Rxinvselect>;
impl RxinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxinvselect {
        match self.bits {
            false => Rxinvselect::Disable,
            true => Rxinvselect::Inv,
        }
    }
    #[doc = "RxD is not inverted"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rxinvselect::Disable
    }
    #[doc = "RxD is inverted"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        *self == Rxinvselect::Inv
    }
}
#[doc = "Field `RXINV` writer - Receive Data Invert"]
pub type RxinvW<'a, REG> = crate::BitWriter<'a, REG, Rxinvselect>;
impl<'a, REG> RxinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RxD is not inverted"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinvselect::Disable)
    }
    #[doc = "RxD is inverted"]
    #[inline(always)]
    pub fn inv(self) -> &'a mut crate::W<REG> {
        self.variant(Rxinvselect::Inv)
    }
}
#[doc = "Sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Samprselect {
    #[doc = "0: 16x oversampling using arithmetic baud rate generation"]
    Arithm16x = 0,
    #[doc = "1: 16x oversampling using fractional baud rate generation"]
    Frac16x = 1,
    #[doc = "2: 8x oversampling using arithmetic baud rate generation"]
    Arithm8x = 2,
    #[doc = "3: 8x oversampling using fractional baud rate generation"]
    Frac8x = 3,
    #[doc = "4: 3x oversampling using arithmetic baud rate generation"]
    Arithm3x = 4,
}
impl From<Samprselect> for u8 {
    #[inline(always)]
    fn from(variant: Samprselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Samprselect {
    type Ux = u8;
}
impl crate::IsEnum for Samprselect {}
#[doc = "Field `SAMPR` reader - Sample"]
pub type SamprR = crate::FieldReader<Samprselect>;
impl SamprR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Samprselect> {
        match self.bits {
            0 => Some(Samprselect::Arithm16x),
            1 => Some(Samprselect::Frac16x),
            2 => Some(Samprselect::Arithm8x),
            3 => Some(Samprselect::Frac8x),
            4 => Some(Samprselect::Arithm3x),
            _ => None,
        }
    }
    #[doc = "16x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_arithm16x(&self) -> bool {
        *self == Samprselect::Arithm16x
    }
    #[doc = "16x oversampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn is_frac16x(&self) -> bool {
        *self == Samprselect::Frac16x
    }
    #[doc = "8x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_arithm8x(&self) -> bool {
        *self == Samprselect::Arithm8x
    }
    #[doc = "8x oversampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn is_frac8x(&self) -> bool {
        *self == Samprselect::Frac8x
    }
    #[doc = "3x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn is_arithm3x(&self) -> bool {
        *self == Samprselect::Arithm3x
    }
}
#[doc = "Field `SAMPR` writer - Sample"]
pub type SamprW<'a, REG> = crate::FieldWriter<'a, REG, 3, Samprselect>;
impl<'a, REG> SamprW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn arithm16x(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::Arithm16x)
    }
    #[doc = "16x oversampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn frac16x(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::Frac16x)
    }
    #[doc = "8x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn arithm8x(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::Arithm8x)
    }
    #[doc = "8x oversampling using fractional baud rate generation"]
    #[inline(always)]
    pub fn frac8x(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::Frac8x)
    }
    #[doc = "3x oversampling using arithmetic baud rate generation"]
    #[inline(always)]
    pub fn arithm3x(self) -> &'a mut crate::W<REG> {
        self.variant(Samprselect::Arithm3x)
    }
}
#[doc = "Transmit Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txposelect {
    #[doc = "0: PAD\\[0\\] = TxD; PAD\\[1\\] = XCK"]
    Pad0 = 0,
    #[doc = "2: PAD\\[0\\] = TxD; PAD\\[2\\] = RTS; PAD\\[3\\] = CTS"]
    Pad2 = 2,
    #[doc = "3: PAD\\[0\\] = TxD; PAD\\[1\\] = XCK; PAD\\[2\\] = TE"]
    Pad3 = 3,
}
impl From<Txposelect> for u8 {
    #[inline(always)]
    fn from(variant: Txposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txposelect {
    type Ux = u8;
}
impl crate::IsEnum for Txposelect {}
#[doc = "Field `TXPO` reader - Transmit Data Pinout"]
pub type TxpoR = crate::FieldReader<Txposelect>;
impl TxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Txposelect> {
        match self.bits {
            0 => Some(Txposelect::Pad0),
            2 => Some(Txposelect::Pad2),
            3 => Some(Txposelect::Pad3),
            _ => None,
        }
    }
    #[doc = "PAD\\[0\\] = TxD; PAD\\[1\\] = XCK"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Txposelect::Pad0
    }
    #[doc = "PAD\\[0\\] = TxD; PAD\\[2\\] = RTS; PAD\\[3\\] = CTS"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Txposelect::Pad2
    }
    #[doc = "PAD\\[0\\] = TxD; PAD\\[1\\] = XCK; PAD\\[2\\] = TE"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Txposelect::Pad3
    }
}
#[doc = "Field `TXPO` writer - Transmit Data Pinout"]
pub type TxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txposelect>;
impl<'a, REG> TxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PAD\\[0\\] = TxD; PAD\\[1\\] = XCK"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Pad0)
    }
    #[doc = "PAD\\[0\\] = TxD; PAD\\[2\\] = RTS; PAD\\[3\\] = CTS"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Pad2)
    }
    #[doc = "PAD\\[0\\] = TxD; PAD\\[1\\] = XCK; PAD\\[2\\] = TE"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Txposelect::Pad3)
    }
}
#[doc = "Receive Data Pinout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxposelect {
    #[doc = "0: SERCOM PAD0 is used for data reception"]
    Pad0 = 0,
    #[doc = "1: SERCOM PAD1 is used for data reception"]
    Pad1 = 1,
    #[doc = "2: SERCOM PAD2 is used for data reception"]
    Pad2 = 2,
    #[doc = "3: SERCOM PAD3 is used for data reception"]
    Pad3 = 3,
}
impl From<Rxposelect> for u8 {
    #[inline(always)]
    fn from(variant: Rxposelect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxposelect {
    type Ux = u8;
}
impl crate::IsEnum for Rxposelect {}
#[doc = "Field `RXPO` reader - Receive Data Pinout"]
pub type RxpoR = crate::FieldReader<Rxposelect>;
impl RxpoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxposelect {
        match self.bits {
            0 => Rxposelect::Pad0,
            1 => Rxposelect::Pad1,
            2 => Rxposelect::Pad2,
            3 => Rxposelect::Pad3,
            _ => unreachable!(),
        }
    }
    #[doc = "SERCOM PAD0 is used for data reception"]
    #[inline(always)]
    pub fn is_pad0(&self) -> bool {
        *self == Rxposelect::Pad0
    }
    #[doc = "SERCOM PAD1 is used for data reception"]
    #[inline(always)]
    pub fn is_pad1(&self) -> bool {
        *self == Rxposelect::Pad1
    }
    #[doc = "SERCOM PAD2 is used for data reception"]
    #[inline(always)]
    pub fn is_pad2(&self) -> bool {
        *self == Rxposelect::Pad2
    }
    #[doc = "SERCOM PAD3 is used for data reception"]
    #[inline(always)]
    pub fn is_pad3(&self) -> bool {
        *self == Rxposelect::Pad3
    }
}
#[doc = "Field `RXPO` writer - Receive Data Pinout"]
pub type RxpoW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxposelect, crate::Safe>;
impl<'a, REG> RxpoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SERCOM PAD0 is used for data reception"]
    #[inline(always)]
    pub fn pad0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad0)
    }
    #[doc = "SERCOM PAD1 is used for data reception"]
    #[inline(always)]
    pub fn pad1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad1)
    }
    #[doc = "SERCOM PAD2 is used for data reception"]
    #[inline(always)]
    pub fn pad2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad2)
    }
    #[doc = "SERCOM PAD3 is used for data reception"]
    #[inline(always)]
    pub fn pad3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxposelect::Pad3)
    }
}
#[doc = "Field `SAMPA` reader - Sample Adjustment"]
pub type SampaR = crate::FieldReader;
#[doc = "Field `SAMPA` writer - Sample Adjustment"]
pub type SampaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Frame Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Formselect {
    #[doc = "0: USART frame"]
    Usart = 0,
    #[doc = "1: USART frame with parity"]
    Usartp = 1,
    #[doc = "2: LIN Master Break and Synck generation"]
    Linbrkgen = 2,
    #[doc = "4: LIN Slave break detection and auto-baud"]
    Linbrkdet = 4,
    #[doc = "5: Break detection and auto-baud with parity"]
    Brkdet = 5,
    #[doc = "7: ISO 7816 Communication"]
    Iso7816 = 7,
}
impl From<Formselect> for u8 {
    #[inline(always)]
    fn from(variant: Formselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Formselect {
    type Ux = u8;
}
impl crate::IsEnum for Formselect {}
#[doc = "Field `FORM` reader - Frame Format"]
pub type FormR = crate::FieldReader<Formselect>;
impl FormR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Formselect> {
        match self.bits {
            0 => Some(Formselect::Usart),
            1 => Some(Formselect::Usartp),
            2 => Some(Formselect::Linbrkgen),
            4 => Some(Formselect::Linbrkdet),
            5 => Some(Formselect::Brkdet),
            7 => Some(Formselect::Iso7816),
            _ => None,
        }
    }
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn is_usart(&self) -> bool {
        *self == Formselect::Usart
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn is_usartp(&self) -> bool {
        *self == Formselect::Usartp
    }
    #[doc = "LIN Master Break and Synck generation"]
    #[inline(always)]
    pub fn is_linbrkgen(&self) -> bool {
        *self == Formselect::Linbrkgen
    }
    #[doc = "LIN Slave break detection and auto-baud"]
    #[inline(always)]
    pub fn is_linbrkdet(&self) -> bool {
        *self == Formselect::Linbrkdet
    }
    #[doc = "Break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn is_brkdet(&self) -> bool {
        *self == Formselect::Brkdet
    }
    #[doc = "ISO 7816 Communication"]
    #[inline(always)]
    pub fn is_iso7816(&self) -> bool {
        *self == Formselect::Iso7816
    }
}
#[doc = "Field `FORM` writer - Frame Format"]
pub type FormW<'a, REG> = crate::FieldWriter<'a, REG, 4, Formselect>;
impl<'a, REG> FormW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USART frame"]
    #[inline(always)]
    pub fn usart(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Usart)
    }
    #[doc = "USART frame with parity"]
    #[inline(always)]
    pub fn usartp(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Usartp)
    }
    #[doc = "LIN Master Break and Synck generation"]
    #[inline(always)]
    pub fn linbrkgen(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Linbrkgen)
    }
    #[doc = "LIN Slave break detection and auto-baud"]
    #[inline(always)]
    pub fn linbrkdet(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Linbrkdet)
    }
    #[doc = "Break detection and auto-baud with parity"]
    #[inline(always)]
    pub fn brkdet(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Brkdet)
    }
    #[doc = "ISO 7816 Communication"]
    #[inline(always)]
    pub fn iso7816(self) -> &'a mut crate::W<REG> {
        self.variant(Formselect::Iso7816)
    }
}
#[doc = "Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmodeselect {
    #[doc = "0: Asynchronous communication."]
    Async = 0,
    #[doc = "1: Synchronous communication."]
    Sync = 1,
}
impl From<Cmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Cmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMODE` reader - Communication Mode"]
pub type CmodeR = crate::BitReader<Cmodeselect>;
impl CmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmodeselect {
        match self.bits {
            false => Cmodeselect::Async,
            true => Cmodeselect::Sync,
        }
    }
    #[doc = "Asynchronous communication."]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == Cmodeselect::Async
    }
    #[doc = "Synchronous communication."]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == Cmodeselect::Sync
    }
}
#[doc = "Field `CMODE` writer - Communication Mode"]
pub type CmodeW<'a, REG> = crate::BitWriter<'a, REG, Cmodeselect>;
impl<'a, REG> CmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Asynchronous communication."]
    #[inline(always)]
    pub fn async_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Async)
    }
    #[doc = "Synchronous communication."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut crate::W<REG> {
        self.variant(Cmodeselect::Sync)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Order\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dordselect {
    #[doc = "0: MSB is transmitted first."]
    Msb = 0,
    #[doc = "1: LSB is transmitted first."]
    Lsb = 1,
}
impl From<Dordselect> for bool {
    #[inline(always)]
    fn from(variant: Dordselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DORD` reader - Data Order"]
pub type DordR = crate::BitReader<Dordselect>;
impl DordR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dordselect {
        match self.bits {
            false => Dordselect::Msb,
            true => Dordselect::Lsb,
        }
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == Dordselect::Msb
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == Dordselect::Lsb
    }
}
#[doc = "Field `DORD` writer - Data Order"]
pub type DordW<'a, REG> = crate::BitWriter<'a, REG, Dordselect>;
impl<'a, REG> DordW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Msb)
    }
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut crate::W<REG> {
        self.variant(Dordselect::Lsb)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&self) -> IbonR {
        IbonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&self) -> TxinvR {
        TxinvR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&self) -> RxinvR {
        RxinvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&self) -> SamprR {
        SamprR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&self) -> TxpoR {
        TxpoR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&self) -> RxpoR {
        RxpoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&self) -> SampaR {
        SampaR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&self) -> FormR {
        FormR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&self) -> CmodeR {
        CmodeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&self) -> DordR {
        DordR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&mut self) -> SwrstW<'_, CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:4 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 7 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RunstdbyW<'_, CtrlaSpec> {
        RunstdbyW::new(self, 7)
    }
    #[doc = "Bit 8 - Immediate Buffer Overflow Notification"]
    #[inline(always)]
    pub fn ibon(&mut self) -> IbonW<'_, CtrlaSpec> {
        IbonW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Data Invert"]
    #[inline(always)]
    pub fn txinv(&mut self) -> TxinvW<'_, CtrlaSpec> {
        TxinvW::new(self, 9)
    }
    #[doc = "Bit 10 - Receive Data Invert"]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RxinvW<'_, CtrlaSpec> {
        RxinvW::new(self, 10)
    }
    #[doc = "Bits 13:15 - Sample"]
    #[inline(always)]
    pub fn sampr(&mut self) -> SamprW<'_, CtrlaSpec> {
        SamprW::new(self, 13)
    }
    #[doc = "Bits 16:17 - Transmit Data Pinout"]
    #[inline(always)]
    pub fn txpo(&mut self) -> TxpoW<'_, CtrlaSpec> {
        TxpoW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Receive Data Pinout"]
    #[inline(always)]
    pub fn rxpo(&mut self) -> RxpoW<'_, CtrlaSpec> {
        RxpoW::new(self, 20)
    }
    #[doc = "Bits 22:23 - Sample Adjustment"]
    #[inline(always)]
    pub fn sampa(&mut self) -> SampaW<'_, CtrlaSpec> {
        SampaW::new(self, 22)
    }
    #[doc = "Bits 24:27 - Frame Format"]
    #[inline(always)]
    pub fn form(&mut self) -> FormW<'_, CtrlaSpec> {
        FormW::new(self, 24)
    }
    #[doc = "Bit 28 - Communication Mode"]
    #[inline(always)]
    pub fn cmode(&mut self) -> CmodeW<'_, CtrlaSpec> {
        CmodeW::new(self, 28)
    }
    #[doc = "Bit 29 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CtrlaSpec> {
        CpolW::new(self, 29)
    }
    #[doc = "Bit 30 - Data Order"]
    #[inline(always)]
    pub fn dord(&mut self) -> DordW<'_, CtrlaSpec> {
        DordW::new(self, 30)
    }
}
#[doc = "USART Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {}
