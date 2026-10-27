use super::functions::ReportDevice;
use super::socket::SmartSocet;
use super::thermometer::SmartTermometr;

#[derive(Debug)]
pub enum SmartDevice {
    Thermometer(SmartTermometr),
    Socket(SmartSocet),
}

// Реализуем трейт для перечисления, чтобы прозрачно вызывать методы
impl ReportDevice for SmartDevice {
    fn name(&self) -> &str {
        match self {
            SmartDevice::Thermometer(t) => t.name(),
            SmartDevice::Socket(s) => s.name(),
        }
    }

    fn print_status(&self) {
        match self {
            SmartDevice::Thermometer(t) => t.print_status(),
            SmartDevice::Socket(s) => s.print_status(),
        }
    }
}
