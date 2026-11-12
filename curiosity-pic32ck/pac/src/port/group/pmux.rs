#[doc = "Register `PMUX[%s]` reader"]
pub type R = crate::R<PmuxSpec>;
#[doc = "Register `PMUX[%s]` writer"]
pub type W = crate::W<PmuxSpec>;
#[doc = "Peripheral Multiplexing for Even-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxeselect {
    #[doc = "0: External Interrupts"]
    A = 0,
    #[doc = "1: ADC and Analog Comparator"]
    B = 1,
    #[doc = "2: SERCOMn (UART, I2C, SPI)"]
    C = 2,
    #[doc = "3: SERCOMn Alt (UART, I2C, SPI)"]
    D = 3,
    #[doc = "4: External Bus Interface"]
    E = 4,
    #[doc = "5: Timer/Counter controller"]
    F = 5,
    #[doc = "6: Timer/Counter controller and Positional Decoder Alt"]
    G = 6,
    #[doc = "7: SQI/CAN/USB/CM33 Trace"]
    H = 7,
    #[doc = "8: SD/MMC Host Controller (Memory Card Interface)"]
    I = 8,
    #[doc = "9: SPI_IXS Audio"]
    J = 9,
    #[doc = "10: Parallel Capture Controller"]
    K = 10,
    #[doc = "11: Ethernet"]
    L = 11,
    #[doc = "12: GCLK/CCL/AC Alt"]
    M = 12,
    #[doc = "13: Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    P = 13,
}
impl From<Pmuxeselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxeselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxeselect {}
#[doc = "Field `PMUXE` reader - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeR = crate::FieldReader<Pmuxeselect>;
impl PmuxeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxeselect> {
        match self.bits {
            0 => Some(Pmuxeselect::A),
            1 => Some(Pmuxeselect::B),
            2 => Some(Pmuxeselect::C),
            3 => Some(Pmuxeselect::D),
            4 => Some(Pmuxeselect::E),
            5 => Some(Pmuxeselect::F),
            6 => Some(Pmuxeselect::G),
            7 => Some(Pmuxeselect::H),
            8 => Some(Pmuxeselect::I),
            9 => Some(Pmuxeselect::J),
            10 => Some(Pmuxeselect::K),
            11 => Some(Pmuxeselect::L),
            12 => Some(Pmuxeselect::M),
            13 => Some(Pmuxeselect::P),
            _ => None,
        }
    }
    #[doc = "External Interrupts"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxeselect::A
    }
    #[doc = "ADC and Analog Comparator"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxeselect::B
    }
    #[doc = "SERCOMn (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Pmuxeselect::C
    }
    #[doc = "SERCOMn Alt (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxeselect::D
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == Pmuxeselect::E
    }
    #[doc = "Timer/Counter controller"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxeselect::F
    }
    #[doc = "Timer/Counter controller and Positional Decoder Alt"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == Pmuxeselect::G
    }
    #[doc = "SQI/CAN/USB/CM33 Trace"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxeselect::H
    }
    #[doc = "SD/MMC Host Controller (Memory Card Interface)"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == Pmuxeselect::I
    }
    #[doc = "SPI_IXS Audio"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == Pmuxeselect::J
    }
    #[doc = "Parallel Capture Controller"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Pmuxeselect::K
    }
    #[doc = "Ethernet"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == Pmuxeselect::L
    }
    #[doc = "GCLK/CCL/AC Alt"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == Pmuxeselect::M
    }
    #[doc = "Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    #[inline(always)]
    pub fn is_p(&self) -> bool {
        *self == Pmuxeselect::P
    }
}
#[doc = "Field `PMUXE` writer - Peripheral Multiplexing for Even-Numbered Pin"]
pub type PmuxeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxeselect>;
impl<'a, REG> PmuxeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Interrupts"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::A)
    }
    #[doc = "ADC and Analog Comparator"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::B)
    }
    #[doc = "SERCOMn (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::C)
    }
    #[doc = "SERCOMn Alt (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::D)
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::E)
    }
    #[doc = "Timer/Counter controller"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::F)
    }
    #[doc = "Timer/Counter controller and Positional Decoder Alt"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::G)
    }
    #[doc = "SQI/CAN/USB/CM33 Trace"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::H)
    }
    #[doc = "SD/MMC Host Controller (Memory Card Interface)"]
    #[inline(always)]
    pub fn i(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::I)
    }
    #[doc = "SPI_IXS Audio"]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::J)
    }
    #[doc = "Parallel Capture Controller"]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::K)
    }
    #[doc = "Ethernet"]
    #[inline(always)]
    pub fn l(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::L)
    }
    #[doc = "GCLK/CCL/AC Alt"]
    #[inline(always)]
    pub fn m(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::M)
    }
    #[doc = "Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    #[inline(always)]
    pub fn p(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxeselect::P)
    }
}
#[doc = "Peripheral Multiplexing for Odd-Numbered Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pmuxoselect {
    #[doc = "0: External Interrupts"]
    A = 0,
    #[doc = "1: ADC and Analog Comparator"]
    B = 1,
    #[doc = "2: SERCOMn (UART, I2C, SPI)"]
    C = 2,
    #[doc = "3: SERCOMn Alt (UART, I2C, SPI)"]
    D = 3,
    #[doc = "4: External Bus Interface"]
    E = 4,
    #[doc = "5: Timer/Counter controller"]
    F = 5,
    #[doc = "6: Timer/Counter controller and Positional Decoder Alt"]
    G = 6,
    #[doc = "7: SQI/CAN/USB/CM33 Trace"]
    H = 7,
    #[doc = "8: SD/MMC Host Controller (Memory Card Interface)"]
    I = 8,
    #[doc = "9: SPI_IXS Audio"]
    J = 9,
    #[doc = "10: Parallel Capture Controller"]
    K = 10,
    #[doc = "11: Ethernet"]
    L = 11,
    #[doc = "12: GCLK/CCL/AC Alt"]
    M = 12,
    #[doc = "13: Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    P = 13,
}
impl From<Pmuxoselect> for u8 {
    #[inline(always)]
    fn from(variant: Pmuxoselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pmuxoselect {
    type Ux = u8;
}
impl crate::IsEnum for Pmuxoselect {}
#[doc = "Field `PMUXO` reader - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoR = crate::FieldReader<Pmuxoselect>;
impl PmuxoR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pmuxoselect> {
        match self.bits {
            0 => Some(Pmuxoselect::A),
            1 => Some(Pmuxoselect::B),
            2 => Some(Pmuxoselect::C),
            3 => Some(Pmuxoselect::D),
            4 => Some(Pmuxoselect::E),
            5 => Some(Pmuxoselect::F),
            6 => Some(Pmuxoselect::G),
            7 => Some(Pmuxoselect::H),
            8 => Some(Pmuxoselect::I),
            9 => Some(Pmuxoselect::J),
            10 => Some(Pmuxoselect::K),
            11 => Some(Pmuxoselect::L),
            12 => Some(Pmuxoselect::M),
            13 => Some(Pmuxoselect::P),
            _ => None,
        }
    }
    #[doc = "External Interrupts"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Pmuxoselect::A
    }
    #[doc = "ADC and Analog Comparator"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Pmuxoselect::B
    }
    #[doc = "SERCOMn (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn is_c(&self) -> bool {
        *self == Pmuxoselect::C
    }
    #[doc = "SERCOMn Alt (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn is_d(&self) -> bool {
        *self == Pmuxoselect::D
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn is_e(&self) -> bool {
        *self == Pmuxoselect::E
    }
    #[doc = "Timer/Counter controller"]
    #[inline(always)]
    pub fn is_f(&self) -> bool {
        *self == Pmuxoselect::F
    }
    #[doc = "Timer/Counter controller and Positional Decoder Alt"]
    #[inline(always)]
    pub fn is_g(&self) -> bool {
        *self == Pmuxoselect::G
    }
    #[doc = "SQI/CAN/USB/CM33 Trace"]
    #[inline(always)]
    pub fn is_h(&self) -> bool {
        *self == Pmuxoselect::H
    }
    #[doc = "SD/MMC Host Controller (Memory Card Interface)"]
    #[inline(always)]
    pub fn is_i(&self) -> bool {
        *self == Pmuxoselect::I
    }
    #[doc = "SPI_IXS Audio"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        *self == Pmuxoselect::J
    }
    #[doc = "Parallel Capture Controller"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        *self == Pmuxoselect::K
    }
    #[doc = "Ethernet"]
    #[inline(always)]
    pub fn is_l(&self) -> bool {
        *self == Pmuxoselect::L
    }
    #[doc = "GCLK/CCL/AC Alt"]
    #[inline(always)]
    pub fn is_m(&self) -> bool {
        *self == Pmuxoselect::M
    }
    #[doc = "Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    #[inline(always)]
    pub fn is_p(&self) -> bool {
        *self == Pmuxoselect::P
    }
}
#[doc = "Field `PMUXO` writer - Peripheral Multiplexing for Odd-Numbered Pin"]
pub type PmuxoW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pmuxoselect>;
impl<'a, REG> PmuxoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "External Interrupts"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::A)
    }
    #[doc = "ADC and Analog Comparator"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::B)
    }
    #[doc = "SERCOMn (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn c(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::C)
    }
    #[doc = "SERCOMn Alt (UART, I2C, SPI)"]
    #[inline(always)]
    pub fn d(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::D)
    }
    #[doc = "External Bus Interface"]
    #[inline(always)]
    pub fn e(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::E)
    }
    #[doc = "Timer/Counter controller"]
    #[inline(always)]
    pub fn f(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::F)
    }
    #[doc = "Timer/Counter controller and Positional Decoder Alt"]
    #[inline(always)]
    pub fn g(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::G)
    }
    #[doc = "SQI/CAN/USB/CM33 Trace"]
    #[inline(always)]
    pub fn h(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::H)
    }
    #[doc = "SD/MMC Host Controller (Memory Card Interface)"]
    #[inline(always)]
    pub fn i(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::I)
    }
    #[doc = "SPI_IXS Audio"]
    #[inline(always)]
    pub fn j(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::J)
    }
    #[doc = "Parallel Capture Controller"]
    #[inline(always)]
    pub fn k(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::K)
    }
    #[doc = "Ethernet"]
    #[inline(always)]
    pub fn l(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::L)
    }
    #[doc = "GCLK/CCL/AC Alt"]
    #[inline(always)]
    pub fn m(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::M)
    }
    #[doc = "Peripheral Touch Controller and JTAG/SWD Debug Interfaces"]
    #[inline(always)]
    pub fn p(self) -> &'a mut crate::W<REG> {
        self.variant(Pmuxoselect::P)
    }
}
impl R {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&self) -> PmuxeR {
        PmuxeR::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&self) -> PmuxoR {
        PmuxoR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Peripheral Multiplexing for Even-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxe(&mut self) -> PmuxeW<'_, PmuxSpec> {
        PmuxeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Peripheral Multiplexing for Odd-Numbered Pin"]
    #[inline(always)]
    pub fn pmuxo(&mut self) -> PmuxoW<'_, PmuxSpec> {
        PmuxoW::new(self, 4)
    }
}
#[doc = "Peripheral Multiplexing\n\nYou can [`read`](crate::Reg::read) this register and get [`pmux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmuxSpec;
impl crate::RegisterSpec for PmuxSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`pmux::R`](R) reader structure"]
impl crate::Readable for PmuxSpec {}
#[doc = "`write(|w| ..)` method takes [`pmux::W`](W) writer structure"]
impl crate::Writable for PmuxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMUX[%s] to value 0"]
impl crate::Resettable for PmuxSpec {}
