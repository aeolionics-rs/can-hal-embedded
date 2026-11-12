#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Character Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Chsizeselect {
    #[doc = "0: 8-bits character"]
    _8Bit = 0,
    #[doc = "1: 9-bits character"]
    _9Bit = 1,
    #[doc = "5: 5-bits character"]
    _5Bit = 5,
    #[doc = "6: 6-bits character"]
    _6Bit = 6,
    #[doc = "7: 7-bits character"]
    _7Bit = 7,
}
impl From<Chsizeselect> for u8 {
    #[inline(always)]
    fn from(variant: Chsizeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Chsizeselect {
    type Ux = u8;
}
impl crate::IsEnum for Chsizeselect {}
#[doc = "Field `CHSIZE` reader - Character Size"]
pub type ChsizeR = crate::FieldReader<Chsizeselect>;
impl ChsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Chsizeselect> {
        match self.bits {
            0 => Some(Chsizeselect::_8Bit),
            1 => Some(Chsizeselect::_9Bit),
            5 => Some(Chsizeselect::_5Bit),
            6 => Some(Chsizeselect::_6Bit),
            7 => Some(Chsizeselect::_7Bit),
            _ => None,
        }
    }
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Chsizeselect::_8Bit
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Chsizeselect::_9Bit
    }
    #[doc = "5-bits character"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Chsizeselect::_5Bit
    }
    #[doc = "6-bits character"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Chsizeselect::_6Bit
    }
    #[doc = "7-bits character"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Chsizeselect::_7Bit
    }
}
#[doc = "Field `CHSIZE` writer - Character Size"]
pub type ChsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Chsizeselect>;
impl<'a, REG> ChsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bits character"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_8Bit)
    }
    #[doc = "9-bits character"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_9Bit)
    }
    #[doc = "5-bits character"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_5Bit)
    }
    #[doc = "6-bits character"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_6Bit)
    }
    #[doc = "7-bits character"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Chsizeselect::_7Bit)
    }
}
#[doc = "Stop Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbmodeselect {
    #[doc = "0: One stop bit"]
    _1Bit = 0,
    #[doc = "1: Two stop bits"]
    _2Bit = 1,
}
impl From<Sbmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Sbmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBMODE` reader - Stop Bit Mode"]
pub type SbmodeR = crate::BitReader<Sbmodeselect>;
impl SbmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbmodeselect {
        match self.bits {
            false => Sbmodeselect::_1Bit,
            true => Sbmodeselect::_2Bit,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == Sbmodeselect::_1Bit
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn is_2_bit(&self) -> bool {
        *self == Sbmodeselect::_2Bit
    }
}
#[doc = "Field `SBMODE` writer - Stop Bit Mode"]
pub type SbmodeW<'a, REG> = crate::BitWriter<'a, REG, Sbmodeselect>;
impl<'a, REG> SbmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::_1Bit)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn _2_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Sbmodeselect::_2Bit)
    }
}
#[doc = "Field `COLDEN` reader - Collision Detection Enable"]
pub type ColdenR = crate::BitReader;
#[doc = "Field `COLDEN` writer - Collision Detection Enable"]
pub type ColdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFDE` reader - Start of Frame Detection Enable"]
pub type SfdeR = crate::BitReader;
#[doc = "Field `SFDE` writer - Start of Frame Detection Enable"]
pub type SfdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Encoding Format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Encselect {
    #[doc = "0: Data is not encoded"]
    Disable = 0,
    #[doc = "1: Data is IrDA encoded"]
    Irda = 1,
}
impl From<Encselect> for bool {
    #[inline(always)]
    fn from(variant: Encselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENC` reader - Encoding Format"]
pub type EncR = crate::BitReader<Encselect>;
impl EncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Encselect {
        match self.bits {
            false => Encselect::Disable,
            true => Encselect::Irda,
        }
    }
    #[doc = "Data is not encoded"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Encselect::Disable
    }
    #[doc = "Data is IrDA encoded"]
    #[inline(always)]
    pub fn is_irda(&self) -> bool {
        *self == Encselect::Irda
    }
}
#[doc = "Field `ENC` writer - Encoding Format"]
pub type EncW<'a, REG> = crate::BitWriter<'a, REG, Encselect>;
impl<'a, REG> EncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is not encoded"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Encselect::Disable)
    }
    #[doc = "Data is IrDA encoded"]
    #[inline(always)]
    pub fn irda(self) -> &'a mut crate::W<REG> {
        self.variant(Encselect::Irda)
    }
}
#[doc = "Parity Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pmodeselect {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Pmodeselect> for bool {
    #[inline(always)]
    fn from(variant: Pmodeselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMODE` reader - Parity Mode"]
pub type PmodeR = crate::BitReader<Pmodeselect>;
impl PmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pmodeselect {
        match self.bits {
            false => Pmodeselect::Even,
            true => Pmodeselect::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Pmodeselect::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Pmodeselect::Odd
    }
}
#[doc = "Field `PMODE` writer - Parity Mode"]
pub type PmodeW<'a, REG> = crate::BitWriter<'a, REG, Pmodeselect>;
impl<'a, REG> PmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Pmodeselect::Odd)
    }
}
#[doc = "Field `TXEN` reader - Transmitter Enable"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXEN` writer - Transmitter Enable"]
pub type TxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEN` reader - Receiver Enable"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXEN` writer - Receiver Enable"]
pub type RxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "FIFO Clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fifoclrselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Clear TX FIFO"]
    Txfifo = 1,
    #[doc = "2: Clear RX FIFO"]
    Rxfifo = 2,
    #[doc = "3: Clear both TX and RF FIFOs"]
    Both = 3,
}
impl From<Fifoclrselect> for u8 {
    #[inline(always)]
    fn from(variant: Fifoclrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fifoclrselect {
    type Ux = u8;
}
impl crate::IsEnum for Fifoclrselect {}
#[doc = "Field `FIFOCLR` reader - FIFO Clear"]
pub type FifoclrR = crate::FieldReader<Fifoclrselect>;
impl FifoclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fifoclrselect {
        match self.bits {
            0 => Fifoclrselect::None,
            1 => Fifoclrselect::Txfifo,
            2 => Fifoclrselect::Rxfifo,
            3 => Fifoclrselect::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Fifoclrselect::None
    }
    #[doc = "Clear TX FIFO"]
    #[inline(always)]
    pub fn is_txfifo(&self) -> bool {
        *self == Fifoclrselect::Txfifo
    }
    #[doc = "Clear RX FIFO"]
    #[inline(always)]
    pub fn is_rxfifo(&self) -> bool {
        *self == Fifoclrselect::Rxfifo
    }
    #[doc = "Clear both TX and RF FIFOs"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Fifoclrselect::Both
    }
}
#[doc = "Field `FIFOCLR` writer - FIFO Clear"]
pub type FifoclrW<'a, REG> = crate::FieldWriter<'a, REG, 2, Fifoclrselect, crate::Safe>;
impl<'a, REG> FifoclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::None)
    }
    #[doc = "Clear TX FIFO"]
    #[inline(always)]
    pub fn txfifo(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Txfifo)
    }
    #[doc = "Clear RX FIFO"]
    #[inline(always)]
    pub fn rxfifo(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Rxfifo)
    }
    #[doc = "Clear both TX and RF FIFOs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoclrselect::Both)
    }
}
#[doc = "LIN Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lincmdselect {
    #[doc = "0: Normal USART transmission"]
    None = 0,
    #[doc = "1: Break field is transmitted when DATA is written"]
    SoftwareControlTransmitCmd = 1,
    #[doc = "2: Break, synch and identifier are automaticcaly transmitted when DATA is written with the identifier"]
    AutoTransmitCmd = 2,
}
impl From<Lincmdselect> for u8 {
    #[inline(always)]
    fn from(variant: Lincmdselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lincmdselect {
    type Ux = u8;
}
impl crate::IsEnum for Lincmdselect {}
#[doc = "Field `LINCMD` reader - LIN Command"]
pub type LincmdR = crate::FieldReader<Lincmdselect>;
impl LincmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lincmdselect> {
        match self.bits {
            0 => Some(Lincmdselect::None),
            1 => Some(Lincmdselect::SoftwareControlTransmitCmd),
            2 => Some(Lincmdselect::AutoTransmitCmd),
            _ => None,
        }
    }
    #[doc = "Normal USART transmission"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Lincmdselect::None
    }
    #[doc = "Break field is transmitted when DATA is written"]
    #[inline(always)]
    pub fn is_software_control_transmit_cmd(&self) -> bool {
        *self == Lincmdselect::SoftwareControlTransmitCmd
    }
    #[doc = "Break, synch and identifier are automaticcaly transmitted when DATA is written with the identifier"]
    #[inline(always)]
    pub fn is_auto_transmit_cmd(&self) -> bool {
        *self == Lincmdselect::AutoTransmitCmd
    }
}
#[doc = "Field `LINCMD` writer - LIN Command"]
pub type LincmdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lincmdselect>;
impl<'a, REG> LincmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Normal USART transmission"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Lincmdselect::None)
    }
    #[doc = "Break field is transmitted when DATA is written"]
    #[inline(always)]
    pub fn software_control_transmit_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Lincmdselect::SoftwareControlTransmitCmd)
    }
    #[doc = "Break, synch and identifier are automaticcaly transmitted when DATA is written with the identifier"]
    #[inline(always)]
    pub fn auto_transmit_cmd(self) -> &'a mut crate::W<REG> {
        self.variant(Lincmdselect::AutoTransmitCmd)
    }
}
impl R {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&self) -> ChsizeR {
        ChsizeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&self) -> SbmodeR {
        SbmodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&self) -> ColdenR {
        ColdenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&self) -> SfdeR {
        SfdeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&self) -> EncR {
        EncR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&self) -> PmodeR {
        PmodeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&self) -> FifoclrR {
        FifoclrR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    pub fn lincmd(&self) -> LincmdR {
        LincmdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Character Size"]
    #[inline(always)]
    pub fn chsize(&mut self) -> ChsizeW<'_, CtrlbSpec> {
        ChsizeW::new(self, 0)
    }
    #[doc = "Bit 6 - Stop Bit Mode"]
    #[inline(always)]
    pub fn sbmode(&mut self) -> SbmodeW<'_, CtrlbSpec> {
        SbmodeW::new(self, 6)
    }
    #[doc = "Bit 8 - Collision Detection Enable"]
    #[inline(always)]
    pub fn colden(&mut self) -> ColdenW<'_, CtrlbSpec> {
        ColdenW::new(self, 8)
    }
    #[doc = "Bit 9 - Start of Frame Detection Enable"]
    #[inline(always)]
    pub fn sfde(&mut self) -> SfdeW<'_, CtrlbSpec> {
        SfdeW::new(self, 9)
    }
    #[doc = "Bit 10 - Encoding Format"]
    #[inline(always)]
    pub fn enc(&mut self) -> EncW<'_, CtrlbSpec> {
        EncW::new(self, 10)
    }
    #[doc = "Bit 13 - Parity Mode"]
    #[inline(always)]
    pub fn pmode(&mut self) -> PmodeW<'_, CtrlbSpec> {
        PmodeW::new(self, 13)
    }
    #[doc = "Bit 16 - Transmitter Enable"]
    #[inline(always)]
    pub fn txen(&mut self) -> TxenW<'_, CtrlbSpec> {
        TxenW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Enable"]
    #[inline(always)]
    pub fn rxen(&mut self) -> RxenW<'_, CtrlbSpec> {
        RxenW::new(self, 17)
    }
    #[doc = "Bits 22:23 - FIFO Clear"]
    #[inline(always)]
    pub fn fifoclr(&mut self) -> FifoclrW<'_, CtrlbSpec> {
        FifoclrW::new(self, 22)
    }
    #[doc = "Bits 24:25 - LIN Command"]
    #[inline(always)]
    pub fn lincmd(&mut self) -> LincmdW<'_, CtrlbSpec> {
        LincmdW::new(self, 24)
    }
}
#[doc = "USART Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
