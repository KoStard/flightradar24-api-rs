use bigdecimal::BigDecimal;

pub struct Point {
    // [-90;+90]
    pub latitude: BigDecimal,
    // [-180;+180)
    pub longitude: BigDecimal,
}
