#[derive(Debug)]
pub struct RawSymbol {
    pub symbol_type: RawSymbolType,
    pub value1: u32,
    pub value2: u32,
}

#[derive(Debug)]
pub enum RawSymbolType {
    DotData,
    DotText,
}