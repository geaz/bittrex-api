use std::fmt;

#[derive(Debug)]
pub enum BittrexOrderType {
    Sell,
    Buy,
    Both,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexAPIResult<T> {
    pub success: bool,
    pub message: String,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexAPIVecResult<T> {
    pub success: bool,
    pub message: String,
    pub result: Option<Vec<T>>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexUuid {
    #[serde(rename = "uuid")]
    pub uuid: String,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexAddress {
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Address")]
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexCurrency {
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "CurrencyLong")]
    pub currency_long: String,
    #[serde(rename = "MinConfirmation")]
    pub min_confirmation: u32,
    #[serde(rename = "TxFee")]
    pub tx_fee: f32,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "CoinType")]
    pub coin_type: Option<String>,
    #[serde(rename = "BaseAddress")]
    pub base_address: Option<String>,
    #[serde(rename = "Notice")]
    pub notice: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexMarket {
    #[serde(rename = "MarketCurrency")]
    pub market_currency: String,
    #[serde(rename = "BaseCurrency")]
    pub base_currency: String,
    #[serde(rename = "MarketCurrencyLong")]
    pub market_currency_long: String,
    #[serde(rename = "BaseCurrencyLong")]
    pub base_currency_long: String,
    #[serde(rename = "MinTradeSize")]
    pub min_trade_size: f32,
    #[serde(rename = "MarketName")]
    pub market_name: String,
    #[serde(rename = "IsActive")]
    pub is_active: bool,
    #[serde(rename = "Created")]
    pub created: String,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexMarketSummary {
    #[serde(rename = "MarketName")]
    pub market_name: String,
    #[serde(rename = "High")]
    pub high: f32,
    #[serde(rename = "Low")]
    pub low: f32,
    #[serde(rename = "Volume")]
    pub volume: f64,
    #[serde(rename = "Last")]
    pub last: f64,
    #[serde(rename = "BaseVolume")]
    pub base_volume: f64,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Bid")]
    pub bid: f32,
    #[serde(rename = "Ask")]
    pub ask: f32,
    #[serde(rename = "OpenBuyOrders")]
    pub open_buy_orders: u32,
    #[serde(rename = "OpenSellOrders")]
    pub open_sell_orders: u32,
    #[serde(rename = "PrevDay")]
    pub prev_day: f32,
    #[serde(rename = "Created")]
    pub created: String,
    #[serde(rename = "DisplayMarketName")]
    pub display_market_name: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexTicker {
    #[serde(rename = "Ask")]
    pub ask: f32,
    #[serde(rename = "Bid")]
    pub bid: f32,
    #[serde(rename = "Last")]
    pub last: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexPublicOrderBook {
    #[serde(rename = "buy")]
    pub buy: Vec<BittrexPublicOrder>,
    #[serde(rename = "sell")]
    pub sell: Vec<BittrexPublicOrder>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexPublicOrder {
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "Rate")]
    pub rate: f32,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexTrade {
    #[serde(rename = "Id")]
    pub id: u32,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "Price")]
    pub price: f32,
    #[serde(rename = "Total")]
    pub total: f32,
    #[serde(rename = "FillType")]
    pub fill_type: String,
    #[serde(rename = "OrderType")]
    pub order_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexOpenOrder {
    #[serde(rename = "Uuid")]
    pub uuid: Option<String>,
    #[serde(rename = "OrderUuid")]
    pub order_uuid: String,
    #[serde(rename = "Exchange")]
    pub exchange: String,
    #[serde(rename = "OrderType")]
    pub order_type: String,
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "QuantityRemaining")]
    pub quantity_remaining: f32,
    #[serde(rename = "Limit")]
    pub limit: f32,
    #[serde(rename = "CommissionPaid")]
    pub comission_paid: f32,
    #[serde(rename = "Price")]
    pub price: f32,
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: Option<f32>,
    #[serde(rename = "Opened")]
    pub opened: String,
    #[serde(rename = "Closed")]
    pub closed: Option<String>,
    #[serde(rename = "CancelInitiated")]
    pub cancel_initiated: bool,
    #[serde(rename = "ImmediateOrCancel")]
    pub immediate_or_cancel: bool,
    #[serde(rename = "IsConditional")]
    pub is_conditional: bool,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "ConditionalTarget")]
    pub conditional_target: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexHistoryOrder {
    #[serde(rename = "OrderUuid")]
    pub order_uuid: String,
    #[serde(rename = "Exchange")]
    pub exchange: String,
    #[serde(rename = "TimeStamp")]
    pub time_stamp: String,
    #[serde(rename = "OrderType")]
    pub order_type: String,
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "QuantityRemaining")]
    pub quantity_remaining: f32,
    #[serde(rename = "Limit")]
    pub limit: f32,
    #[serde(rename = "Commission")]
    pub comission: f32,
    #[serde(rename = "Price")]
    pub price: f32,
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: Option<f32>,
    #[serde(rename = "ImmediateOrCancel")]
    pub immediate_or_cancel: bool,
    #[serde(rename = "IsConditional")]
    pub is_conditional: bool,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "ConditionalTarget")]
    pub conditional_target: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexOrder {
    #[serde(rename = "AccountId")]
    pub account_id: Option<String>,
    #[serde(rename = "OrderUuid")]
    pub order_uuid: String,
    #[serde(rename = "Exchange")]
    pub exchange: String,
    #[serde(rename = "Type")]
    pub order_type: String,
    #[serde(rename = "Quantity")]
    pub quantity: f32,
    #[serde(rename = "QuantityRemaining")]
    pub quantity_remaining: f32,
    #[serde(rename = "Limit")]
    pub limit: f32,
    #[serde(rename = "Reserved")]
    pub reserved: f32,
    #[serde(rename = "ReserveRemaining")]
    pub reserve_remaining: f32,
    #[serde(rename = "CommissionReserved")]
    pub commission_reserved: f32,
    #[serde(rename = "CommissionReserveRemaining")]
    pub commission_reserve_remaining: f32,
    #[serde(rename = "CommissionPaid")]
    pub comission_paid: f32,
    #[serde(rename = "Price")]
    pub price: f32,
    #[serde(rename = "PricePerUnit")]
    pub price_per_unit: Option<f32>,
    #[serde(rename = "Opened")]
    pub opened: String,
    #[serde(rename = "Closed")]
    pub closed: Option<String>,
    #[serde(rename = "IsOpen")]
    pub is_open: bool,
    #[serde(rename = "Sentinel")]
    pub sentinel: String,
    #[serde(rename = "CancelInitiated")]
    pub cancel_initiated: bool,
    #[serde(rename = "ImmediateOrCancel")]
    pub immediate_or_cancel: bool,
    #[serde(rename = "IsConditional")]
    pub is_conditional: bool,
    #[serde(rename = "Condition")]
    pub condition: Option<String>,
    #[serde(rename = "ConditionalTarget")]
    pub conditional_target: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexTransaction {
    #[serde(rename = "PaymentUuid")]
    pub payment_uuid: String,
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Amount")]
    pub amount: f32,
    #[serde(rename = "Address")]
    pub address: String,
    #[serde(rename = "Opened")]
    pub opened: String,
    #[serde(rename = "Authorized")]
    pub authorized: bool,
    #[serde(rename = "PendingPayment")]
    pub pending_payment: bool,
    #[serde(rename = "TxCost")]
    pub tx_cost: f32,
    #[serde(rename = "TxId")]
    pub tx_id: Option<String>,
    #[serde(rename = "Canceled")]
    pub canceled: bool,
    #[serde(rename = "InvalidAddress")]
    pub invalid_address: bool,
}

#[derive(Serialize, Deserialize)]
pub struct BittrexBalance {
    #[serde(rename = "Currency")]
    pub currency: String,
    #[serde(rename = "Balance")]
    pub balance: f32,
    #[serde(rename = "Available")]
    pub available: f32,
    #[serde(rename = "Pending")]
    pub pending: f32,
    #[serde(rename = "CryptoAddress")]
    pub crypto_address: Option<String>,
}

impl fmt::Display for BittrexOrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Display for BittrexUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.uuid)
    }
}

impl fmt::Display for BittrexAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Currency: {} (Address: {})", self.currency, self.address)
    }
}

impl fmt::Display for BittrexCurrency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (Min. Confirmations: {}, Tx Fee: {})",
            self.currency,
            self.min_confirmation,
            self.tx_fee
        )
    }
}

impl fmt::Display for BittrexMarket {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (Min. Trade Size: {})",
            self.market_name,
            self.min_trade_size
        )
    }
}

impl fmt::Display for BittrexMarketSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} (High: {}, Low: {}, Volume: {})",
            self.market_name,
            self.high,
            self.low,
            self.volume
        )
    }
}

impl fmt::Display for BittrexTicker {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Ask: {}, Bid: {}, Last: {})",
            self.ask,
            self.bid,
            self.last
        )
    }
}

impl fmt::Display for BittrexPublicOrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "(Buy Quantity: {}, Sell Quantity: {})",
            self.buy.len(),
            self.sell.len()
        )
    }
}

impl fmt::Display for BittrexPublicOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Quantity: {}, Rate: {})", self.quantity, self.rate)
    }
}

impl fmt::Display for BittrexTrade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ID: {} (Quantity: {}, Price: {}, Total: {})",
            self.id,
            self.quantity,
            self.price,
            self.total
        )
    }
}

impl fmt::Display for BittrexOpenOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uuid: {} (Exchange: {}, Order Type: {}, Quantity: {}, Limit: {})",
            self.order_uuid,
            self.exchange,
            self.order_type,
            self.quantity,
            self.limit
        )
    }
}

impl fmt::Display for BittrexHistoryOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uuid: {} (Exchange: {}, Type: {}, Quantity: {}, Limit: {})",
            self.order_uuid,
            self.exchange,
            self.order_type,
            self.quantity,
            self.limit
        )
    }
}

impl fmt::Display for BittrexOrder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uuid: {} (Exchange: {}, Type: {}, Quantity: {}, Limit: {}, Is Open: {})",
            self.order_uuid,
            self.exchange,
            self.order_type,
            self.quantity,
            self.limit,
            self.is_open
        )
    }
}

impl fmt::Display for BittrexTransaction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Uuid: {} (Currency: {}, Ammount: {}, Address: {}, Pending: {})",
            self.payment_uuid,
            self.currency,
            self.amount,
            self.address,
            self.pending_payment
        )
    }
}

impl fmt::Display for BittrexBalance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Currency: {} (Balance: {}, Available: {}, Pending: {})",
            self.currency,
            self.balance,
            self.available,
            self.pending
        )
    }
}
