use time;
use std::str;
use hmac::{Hmac, Mac, MacResult};
use sha2::Sha512;
use generic_array::typenum::U64;
use serde;
use reqwest::{Client, Proxy};
use reqwest::header::Headers;

use error::{ BittrexError, BittrexErrorType };
use values::*;

const API_URL: &str = "https://bittrex.com/api/v1.1";

pub struct BittrexClient {
    api_url: String,
    api_key: String,
    api_secret: String,
    http_proxy: Option<String>,
    https_proxy: Option<String>
}

impl BittrexClient {
    pub fn new(api_key: String, api_secret: String) -> Self {
        BittrexClient { api_url: API_URL.to_string(), api_key: api_key, api_secret: api_secret, http_proxy: None, https_proxy: None }
    }

    pub fn new_override_api_url(api_key: String, api_secret: String, api_url: String) -> Self {
        BittrexClient { api_url: api_url, api_key: api_key, api_secret: api_secret, http_proxy: None, https_proxy: None }
    }

    pub fn new_with_proxy(api_key: String, api_secret: String, http_proxy: Option<String>, https_proxy: Option<String>) -> Self {
        BittrexClient { api_url: API_URL.to_string(), api_key: api_key, api_secret: api_secret, http_proxy: http_proxy, https_proxy: https_proxy }
    }

    /// Returns all available market data
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let markets = bittrex_client.get_markets().unwrap();
    /// ```
    pub fn get_markets(&self) -> Result<Vec<BittrexMarket>, BittrexError> {
        println!("{}", self.api_url);
        let markets = self.call_public_api::<BittrexAPIVecResult<BittrexMarket>>(&format!("{}/public/getmarkets", self.api_url))?;
        self.check_return_vec_response(markets)
    }

    /// Returns all available currencies
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let currencies = bittrex_client.get_currencies().unwrap();
    /// ```
    pub fn get_currencies(&self) -> Result<Vec<BittrexCurrency>, BittrexError> {
        let currencies = self.call_public_api::<BittrexAPIVecResult<BittrexCurrency>>(&format!("{}/public/getcurrencies", self.api_url))?;
        self.check_return_vec_response(currencies)
    }

    /// Returns ticker by market name
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let ticker = bittrex_client.get_ticker("BTC-LTC").unwrap();
    /// ```
    pub fn get_ticker(&self, market: &str) -> Result<BittrexTicker, BittrexError> {
        let ticker = self.call_public_api::<BittrexAPIResult<BittrexTicker>>(&format!("{}/public/getticker?market={}", self.api_url, market))?;
        self.check_return_single_response(ticker)        
    }

    /// Returns all market summaries
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let summaries = bittrex_client.get_market_summaries().unwrap();
    /// ```
    pub fn get_market_summaries(&self) -> Result<Vec<BittrexMarketSummary>, BittrexError> {
        let summaries = self.call_public_api::<BittrexAPIVecResult<BittrexMarketSummary>>(&format!("{}/public/getmarketsummaries", self.api_url))?;
        self.check_return_vec_response(summaries)
    }

    /// Returns market summary by market name
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let summary = bittrex_client.get_market_summary("BTC-LTC").unwrap();
    /// ```
    pub fn get_market_summary(&self, market: &str) -> Result<BittrexMarketSummary, BittrexError> {
        let summary = self.call_public_api::<BittrexAPIVecResult<BittrexMarketSummary>>(&format!("{}/public/getmarketsummary?market={}", self.api_url, market))?;
        self.check_return_single_vec_response(summary)
    }

    /// Returns the order book of the given market.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    /// use bittrex_api::values::BittrexOrderType;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let order_book = bittrex_client.get_order_book("BTC-LTC", BittrexOrderType::Both).unwrap();
    /// ```
    pub fn get_order_book(&self, market: &str, book_type: BittrexOrderType) -> Result<BittrexPublicOrderBook, BittrexError> {
        let order_book = self.call_public_api::<BittrexAPIResult<BittrexPublicOrderBook>>(&format!("{}/public/getorderbook?market={}&type={}", self.api_url, market, book_type))?;
        self.check_return_single_response(order_book)
    }

    /// Returns the market history of the given market.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let market_history = bittrex_client.get_market_history("BTC-LTC").unwrap();
    /// ```
    pub fn get_market_history(&self, market: &str) -> Result<Vec<BittrexTrade>, BittrexError> {
        let market_history = self.call_public_api::<BittrexAPIVecResult<BittrexTrade>>(&format!("{}/public/getmarkethistory?market={}", self.api_url, market))?;
        self.check_return_vec_response(market_history)
    }

    /// Returns the open orders of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let open_orders = bittrex_client.get_open_orders().unwrap();
    /// ```
    pub fn get_open_orders(&self) -> Result<Vec<BittrexOpenOrder>, BittrexError> {
        let open_orders = self.call_private_api::<BittrexAPIVecResult<BittrexOpenOrder>>(&format!("{}/market/getopenorders?", self.api_url))?;
        self.check_return_vec_response(open_orders)
    }

    /// Returns the open orders of the given market and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let open_orders = bittrex_client.get_open_orders_by_market("BTC-LTC").unwrap();
    /// ```
    pub fn get_open_orders_by_market(&self, market: &str) -> Result<Vec<BittrexOpenOrder>, BittrexError> {
        let open_orders = self.call_private_api::<BittrexAPIVecResult<BittrexOpenOrder>>(&format!("{}/market/getopenorders?market={}", self.api_url, market))?;
        self.check_return_vec_response(open_orders)
    }

    /// Returns the order given by the order_id.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let order = bittrex_client.get_order("ORDERID").unwrap();
    /// ```
    pub fn get_order(&self, order_id: &str) -> Result<BittrexOrder, BittrexError> {
        let order = self.call_private_api::<BittrexAPIResult<BittrexOrder>>(&format!("{}/account/getorder?uuid={}", self.api_url, order_id))?;
        self.check_return_single_response(order)
    }

    /// Returns the order history of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let order_history = bittrex_client.get_order_history().unwrap();
    /// ```
    pub fn get_order_history(&self) -> Result<Vec<BittrexHistoryOrder>, BittrexError> {
        let order_history = self.call_private_api::<BittrexAPIVecResult<BittrexHistoryOrder>>(&format!("{}/account/getorderhistory?", self.api_url))?;
        self.check_return_vec_response(order_history)
    }

    /// Returns the order history of the given market and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let order_history = bittrex_client.get_order_history_by_market("BTC-LTC").unwrap();
    /// ```
    pub fn get_order_history_by_market(&self, market: &str) -> Result<Vec<BittrexHistoryOrder>, BittrexError> {
        let order_history = self.call_private_api::<BittrexAPIVecResult<BittrexHistoryOrder>>(&format!("{}/account/getorderhistory?market={}", self.api_url, market))?;
        self.check_return_vec_response(order_history)
    }

    /// Returns the withdrawal history of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let withdrawal_history = bittrex_client.get_withdrawal_history().unwrap();
    /// ```
    pub fn get_withdrawal_history(&self) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let withdrawal_history = self.call_private_api::<BittrexAPIVecResult<BittrexTransaction>>(&format!("{}/account/getwithdrawalhistory?", self.api_url))?;
        self.check_return_vec_response(withdrawal_history)
    }

    /// Returns the withdrawal history of the given market and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let withdrawal_history = bittrex_client.get_withdrawal_history_by_currency("BTC-LTC").unwrap();
    /// ```
    pub fn get_withdrawal_history_by_currency(&self, currency: &str) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let withdrawal_history = self.call_private_api::<BittrexAPIVecResult<BittrexTransaction>>(&format!("{}/account/getwithdrawalhistory?currency={}", self.api_url, currency))?;
        self.check_return_vec_response(withdrawal_history)
    }

    /// Returns the deposit history of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let deposit_history = bittrex_client.get_deposit_history().unwrap();
    /// ```
    pub fn get_deposit_history(&self) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let deposit_history = self.call_private_api::<BittrexAPIVecResult<BittrexTransaction>>(&format!("{}/account/getdeposithistory?", self.api_url))?;
        self.check_return_vec_response(deposit_history)
    }

    /// Returns the deposit history of the given currency and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let deposit_history = bittrex_client.get_deposit_history_by_currency("BTC").unwrap();
    /// ```
    pub fn get_deposit_history_by_currency(&self, currency: &str) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let deposit_history = self.call_private_api::<BittrexAPIVecResult<BittrexTransaction>>(&format!("{}/account/getdeposithistory?currency={}", self.api_url, currency))?;
        self.check_return_vec_response(deposit_history)
    }

    /// Returns the balances of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let balances = bittrex_client.get_balances().unwrap();
    /// ```
    pub fn get_balances(&self) -> Result<Vec<BittrexBalance>, BittrexError> {
        let balances = self.call_private_api::<BittrexAPIVecResult<BittrexBalance>>(&format!("{}/account/getbalances?", self.api_url))?;
        self.check_return_vec_response(balances)
    }

    /// Returns the balance of the given currency and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let balance = bittrex_client.get_balance("BTC").unwrap();
    /// ```
    pub fn get_balance(&self, currency: &str) -> Result<BittrexBalance, BittrexError> {
        let balance = self.call_private_api::<BittrexAPIResult<BittrexBalance>>(&format!("{}/account/getbalance?currency={}", self.api_url, currency))?;
        self.check_return_single_response(balance)
    }

    /// Returns the deposit address of the given currency and of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let deposit_history = bittrex_client.get_deposit_address("BTC").unwrap();
    /// ```
    pub fn get_deposit_address(&self, currency: &str) -> Result<BittrexAddress, BittrexError> {
        let deposit_address = self.call_private_api::<BittrexAPIResult<BittrexAddress>>(&format!("{}/account/getdepositaddress?currency={}", self.api_url, currency))?;
        self.check_return_single_response(deposit_address)
    }

    /// Withdraws tokens of the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let withdraw_uuid = bittrex_client.withdraw("BTC", 1.5, "BITCOINADDRESS", "").unwrap();
    /// ```
    pub fn withdraw(&self, currency: &str, quantity: f64, address: &str, payment_id: &str) -> Result<BittrexUuid, BittrexError> {
        let withdraw = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/account/withdraw?currency={}&quantity={}&address={}&paymentid={}", self.api_url, currency, quantity, address, payment_id))?;
        self.check_return_single_response(withdraw)
    }

    /// Places a buy order on the given market for the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let buy_uuid = bittrex_client.buy_limit("BTC-LTC", 1.5, 0.00023).unwrap();
    /// ```
    pub fn buy_limit(&self, market: &str, quantity: f64, rate: f64) -> Result<BittrexUuid, BittrexError> {
        let buy_limit = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/market/buylimit?market={}&quantity={}&rate={}", self.api_url, market, quantity, rate))?;
        self.check_return_single_response(buy_limit)
    }

    /// Places a sell order on the given market for the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// let sell_uuid = bittrex_client.sell_limit("BTC-LTC", 1.5, 0.00023).unwrap();
    /// ```
    pub fn sell_limit(&self, market: &str, quantity: f64, rate: f64) -> Result<BittrexUuid, BittrexError> {
        let sell_limit = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/market/selllimit?market={}&quantity={}&rate={}", self.api_url, market, quantity, rate))?;
        self.check_return_single_response(sell_limit)
    }

    /// Cancels an order for the user given by the api_key and api_secret.
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::BittrexClient;
    ///
    /// let bittrex_client = BittrexClient::new("APIKEY".to_string(), "APISECRET".to_string());
    /// bittrex_client.cancel_order("ORDERID").unwrap();
    /// ```
    pub fn cancel_order(&self, order_id: &str) -> Result<(), BittrexError> {
        self.call_private_api::<BittrexAPIResult<()>>(&format!("{}/market/cancel?uuid={}", self.api_url, order_id))?;
        Ok(())
    }

    fn call_public_api<T>(&self, url: &str) -> Result<T, BittrexError> where for<'de> T: serde::Deserialize<'de> {
        let client = self.get_client()?;
        let mut resp = client.get(url)?.send()?;
        let result : T = resp.json()?;
        
        Ok(result)
    }

    fn call_private_api<T>(&self, url: &str) -> Result<T, BittrexError> where for<'de> T: serde::Deserialize<'de> {
        let url_with_key = format!("{}&apikey={}&nonce={}", url, self.api_key, time::precise_time_ns());
        let hmac = self.sign_call(&url_with_key);
        
        let mut headers = Headers::new();
        headers.set_raw("apisign", self.to_hex_string(hmac.code()));

        let client = self.get_client()?;
        let mut resp = client.get(&url_with_key)?.headers(headers).send()?;
        let result : T = resp.json()?;
        
        Ok(result)
    }

    fn sign_call(&self, msg: &str) -> MacResult<U64> {
        let mut hmac = Hmac::<Sha512>::new(self.api_secret.as_bytes());
        hmac.input(msg.as_bytes());
        
        MacResult::from_slice(hmac.result().code())
    }

    fn get_client(&self) -> Result<Client, BittrexError> {
        let mut client_builder = Client::builder()?;

        match self.http_proxy {
            Some(ref proxy) => { client_builder.proxy(Proxy::http(proxy)?); () }
            _ => ()
        }

        match self.https_proxy {
            Some(ref proxy) => { client_builder.proxy(Proxy::https(proxy)?); () }
            _ => ()
        }
        
        Ok(client_builder.build()?)
    }

    fn check_return_single_response<T>(&self, bittrex_api_result: BittrexAPIResult<T>) -> Result<T, BittrexError> {
        match bittrex_api_result.success {
            true => Ok(bittrex_api_result.result.unwrap()),
            false => Err(BittrexError { error_type: BittrexErrorType::APIError, message: bittrex_api_result.message })
        }
    }

    fn check_return_vec_response<T>(&self, bittrex_api_result: BittrexAPIVecResult<T>) -> Result<Vec<T>, BittrexError> {
        match bittrex_api_result.success {
            true => Ok(bittrex_api_result.result.unwrap()),
            false => Err(BittrexError { error_type: BittrexErrorType::APIError, message: bittrex_api_result.message })
        }
    }

    fn check_return_single_vec_response<T>(&self, bittrex_api_result: BittrexAPIVecResult<T>) -> Result<T, BittrexError> {
        match bittrex_api_result.success {
            true => {
                let mut result = bittrex_api_result.result.unwrap();
                match result.len() {
                    1 => Ok(result.remove(0)),
                    0 => Err(BittrexError { error_type: BittrexErrorType::NoResults, message: "Maybe check your parameters?".to_string() }),
                    _ => Err(BittrexError { error_type: BittrexErrorType::APIError, message: "Multiple results found! Maybe check your parameters?".to_string() })
                }
            },
            false => Err(BittrexError { error_type: BittrexErrorType::APIError, message: bittrex_api_result.message })
        }
    }

    fn to_hex_string(&self, bytes: &[u8]) -> String {
        let strs: Vec<String> = bytes.iter()
                                    .map(|b| format!("{:02X}", b))
                                    .collect();
        strs.join("")
    }
}