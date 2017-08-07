use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct BittrexAPIResult<T> {
    pub success: bool,
    pub message: String,
    pub result: Vec<T>
}

#[derive(Serialize, Deserialize)]
pub struct BittrexCurrency {
    #[serde(rename="Currency")]
    pub currency: String,
    #[serde(rename="CurrencyLong")]
    pub currency_long: String,
    #[serde(rename="MinConfirmation")]
    pub min_confirmation: u32,
    #[serde(rename="TxFee")]
    pub tx_fee: f32,
    #[serde(rename="IsActive")]
    pub is_active: bool,
    #[serde(rename="CoinType")]
    pub coin_type: Option<String>,
    #[serde(rename="BaseAddress")]
    pub base_address: Option<String>,
    #[serde(rename="Notice")]
    pub notice: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct BittrexMarket {
    #[serde(rename="MarketCurrency")]
    pub market_currency: String,
    #[serde(rename="BaseCurrency")]
    pub base_currency: String,
    #[serde(rename="MarketCurrencyLong")]
    pub market_currency_long: String,
    #[serde(rename="BaseCurrencyLong")]
    pub base_currency_long: String,
    #[serde(rename="MinTradeSize")]
    pub min_trade_size: f32,
    #[serde(rename="MarketName")]
    pub market_name: String,
    #[serde(rename="IsActive")]
    pub is_active: bool,
    #[serde(rename="Created")]
    pub created: String
}

#[derive(Serialize, Deserialize)]
pub struct BittrexMarketSummary {
    #[serde(rename="MarketName")]
    pub market_name: String,
    #[serde(rename="High")]
    pub high: f32,
    #[serde(rename="Low")]
    pub low: f32,
    #[serde(rename="Volume")]
    pub volume: f64,
    #[serde(rename="TimeStamp")]
    pub time_stamp: String,
    #[serde(rename="Bid")]
    pub bid: f32,
    #[serde(rename="Ask")]
    pub ask: f32,
    #[serde(rename="OpenBuyOrders")]
    pub open_buy_orders: u32,
    #[serde(rename="OpenSellOrders")]
    pub open_sell_orders: u32,
    #[serde(rename="PrevDay")]
    pub prev_day: f32,
    #[serde(rename="Created")]
    pub created: String,
    #[serde(rename="DisplayMarketName")]
    pub display_market_name: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct BittrexTicker {
    #[serde(rename="Ask")]
    pub bid: f32,
    #[serde(rename="Bid")]
    pub ask: f32,
    #[serde(rename="Last")]
    pub last: f32
}

#[derive(Serialize, Deserialize)]
pub struct BittrexPublicOrderBook {
    #[serde(rename="Buy")]
    pub buy: Vec<BittrexPublicOrder>,
    #[serde(rename="Sell")]
    pub sell: Vec<BittrexPublicOrder>
}

#[derive(Serialize, Deserialize)]
pub struct BittrexPublicOrder {
    #[serde(rename="Quantity")]
    pub quantity: f32,
    #[serde(rename="Rate")]
    pub rate: f32
}

#[derive(Serialize, Deserialize)]
pub struct BittrexTrade {
    #[serde(rename="Id")]
    pub id: u32,
    #[serde(rename="TimeStamp")]
    pub time_stamp: String,
    #[serde(rename="Quantity")]
    pub quantity: f32,
    #[serde(rename="Price")]
    pub price: f32,
    #[serde(rename="Total")]
    pub total: f32,
    #[serde(rename="FillType")]
    pub fill_type: String,
    #[serde(rename="OrderType")]
    pub order_type: String
}

impl fmt::Display for BittrexCurrency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Min. Confirmations: {}, Tx Fee: {})", self.currency, self.min_confirmation, self.tx_fee)
    }
}

impl fmt::Display for BittrexMarket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (Min. Trade Size: {})", self.market_name, self.min_trade_size)
    }
}

impl fmt::Display for BittrexMarketSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (High: {}, Low: {}, Volume: {})", self.market_name, self.high, self.low, self.volume)
    }
}

impl fmt::Display for BittrexTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Ask: {}, Bid: {}, Last: {})", self.ask, self.bid, self.last)
    }
}

impl fmt::Display for BittrexPublicOrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Buy Quantity: {}, Sell Quantity: {})", self.buy.len(), self.sell.len())
    }
}

impl fmt::Display for BittrexPublicOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Quantity: {}, Rate: {})", self.quantity, self.rate)
    }
}

impl fmt::Display for BittrexTrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ID: {} (Quantity: {}, Price: {}, Total: {})", self.id, self.quantity, self.price, self.total)
    }
}