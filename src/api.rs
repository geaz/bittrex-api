#[cfg(test)]
use mockito;

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

#[cfg(not(test))]
const API_URL: &str = "https://bittrex.com/api/v1.1";
#[cfg(test)]
const API_URL: &str = mockito::SERVER_URL;

pub struct BittrexAPI<'a> {
    api_key: &'a str,
    api_secret: &'a str,
    http_proxy: Option<&'a str>,
    https_proxy: Option<&'a str>
}

impl<'a> BittrexAPI<'a> {
    pub fn new(api_key: &'a str, api_secret: &'a str) -> Self {
        BittrexAPI { api_key: api_key, api_secret: api_secret, http_proxy: None, https_proxy: None }
    }

    pub fn new_with_proxy(api_key: &'a str, api_secret: &'a str, http_proxy: Option<&'a str>, https_proxy: Option<&'a str>) -> Self {
        BittrexAPI { api_key: api_key, api_secret: api_secret, http_proxy: http_proxy, https_proxy: https_proxy }
    }

    /// Returns all available market data
    ///
    /// # Examples
    /// 
    /// ```rust,no_run
    /// use bittrex_api::api::BittrexAPI;
    ///
    /// let bittrex_api = BittrexAPI::new("APIKEY", "APISECRET");
    /// let markets = bittrex_api.get_markets().unwrap();
    /// ```
    pub fn get_markets(&self) -> Result<Vec<BittrexMarket>, BittrexError> {
        println!("{}", API_URL);
        let markets = self.call_public_api::<BittrexAPIResult<BittrexMarket>>(&format!("{}/public/getmarkets", API_URL))?;
        self.check_return_vec_response(markets)
    }

    /// Returns all available currencies
    pub fn get_currencies(&self) -> Result<Vec<BittrexCurrency>, BittrexError> {
        let currencies = self.call_public_api::<BittrexAPIResult<BittrexCurrency>>(&format!("{}/public/getcurrencies", API_URL))?;
        self.check_return_vec_response(currencies)
    }

    pub fn get_ticker(&self, market: &str) -> Result<BittrexTicker, BittrexError> {
        let ticker = self.call_public_api::<BittrexAPIResult<BittrexTicker>>(&format!("{}/public/getticker?market={}", API_URL, market))?;
        self.check_return_single_response(ticker)        
    }

    pub fn get_market_summaries(&self) -> Result<Vec<BittrexMarketSummary>, BittrexError> {
        let summaries = self.call_public_api::<BittrexAPIResult<BittrexMarketSummary>>(&format!("{}/public/getmarketsummaries", API_URL))?;
        self.check_return_vec_response(summaries)
    }

    pub fn get_market_summary(&self, market: &str) -> Result<BittrexMarketSummary, BittrexError> {
        let summary = self.call_public_api::<BittrexAPIResult<BittrexMarketSummary>>(&format!("{}/public/getticker?getmarketsummary?market={}", API_URL, market))?;
        self.check_return_single_response(summary)
    }

    pub fn get_order_book(&self, market: &str, book_type: &str) -> Result<BittrexPublicOrderBook, BittrexError> {
        let order_book = self.call_public_api::<BittrexAPIResult<BittrexPublicOrderBook>>(&format!("{}/public/getticker?getorderbook=?market={}&type={}", API_URL, market, book_type))?;
        self.check_return_single_response(order_book)
    }

    pub fn get_market_history(&self, market: &str) -> Result<BittrexTrade, BittrexError> {
        let market_history = self.call_public_api::<BittrexAPIResult<BittrexTrade>>(&format!("{}/public/getmarkethistory?market={}", API_URL, market))?;
        self.check_return_single_response(market_history)
    }

    pub fn get_open_orders(&self) -> Result<Vec<BittrexOpenOrder>, BittrexError> {
        let open_orders = self.call_private_api::<BittrexAPIResult<BittrexOpenOrder>>(&format!("{}/market/getopenorders", API_URL))?;
        self.check_return_vec_response(open_orders)
    }

    pub fn get_open_orders_by_market(&self, market: &str) -> Result<Vec<BittrexOpenOrder>, BittrexError> {
        let open_orders = self.call_private_api::<BittrexAPIResult<BittrexOpenOrder>>(&format!("{}/market/getopenorders?market={}", API_URL, market))?;
        self.check_return_vec_response(open_orders)
    }

    pub fn get_order(&self, order_id: &str) -> Result<BittrexOrder, BittrexError> {
        let order = self.call_private_api::<BittrexAPIResult<BittrexOrder>>(&format!("{}/account/getorder?uuid={}", API_URL, order_id))?;
        self.check_return_single_response(order)
    }

    pub fn get_order_history(&self) -> Result<Vec<BittrexHistoryOrder>, BittrexError> {
        let order_history = self.call_private_api::<BittrexAPIResult<BittrexHistoryOrder>>(&format!("{}/account/getorderhistory?", API_URL))?;
        self.check_return_vec_response(order_history)
    }

    pub fn get_withdrawal_history(&self) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let withdrawal_history = self.call_private_api::<BittrexAPIResult<BittrexTransaction>>(&format!("{}/account/getwithdrawalhistory", API_URL))?;
        self.check_return_vec_response(withdrawal_history)
    }

    pub fn get_withdrawal_history_by_currency(&self, currency: &str) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let withdrawal_history = self.call_private_api::<BittrexAPIResult<BittrexTransaction>>(&format!("{}/account/getwithdrawalhistory?currency={}", API_URL, currency))?;
        self.check_return_vec_response(withdrawal_history)
    }

    pub fn get_deposit_history(&self, currency: &str) -> Result<BittrexTransaction, BittrexError> {
        let deposit = self.call_private_api::<BittrexAPIResult<BittrexTransaction>>(&format!("{}/account/getdeposithistory?currency={}", API_URL, currency))?;
        self.check_return_single_response(deposit)
    }

    pub fn get_deposit_history_by_currency(&self, currency: &str) -> Result<Vec<BittrexTransaction>, BittrexError> {
        let deposit_history = self.call_private_api::<BittrexAPIResult<BittrexTransaction>>(&format!("{}/account/getdeposithistory?currency={}", API_URL, currency))?;
        self.check_return_vec_response(deposit_history)
    }

    pub fn get_balances(&self) -> Result<Vec<BittrexBalance>, BittrexError> {
        let balances = self.call_private_api::<BittrexAPIResult<BittrexBalance>>(&format!("{}/account/getbalances?", API_URL))?;
        self.check_return_vec_response(balances)
    }

    pub fn get_balance(&self, currency: &str) -> Result<BittrexBalance, BittrexError> {
        let balance = self.call_private_api::<BittrexAPIResult<BittrexBalance>>(&format!("{}/account/getbalance?currency={}", API_URL, currency))?;
        self.check_return_single_response(balance)
    }

    pub fn get_deposit_address(&self, currency: &str) -> Result<BittrexAddress, BittrexError> {
        let deposit_address = self.call_private_api::<BittrexAPIResult<BittrexAddress>>(&format!("{}/account/getdepositaddress?currency={}", API_URL, currency))?;
        self.check_return_single_response(deposit_address)
    }

    pub fn withdraw(&self, currency: &str, quantity: f64, address: &str, payment_id: &str) -> Result<BittrexUuid, BittrexError> {
        let withdraw = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/account/withdraw?currency={}&quantity={}&address={}&paymentid={}", API_URL, currency, quantity, address, payment_id))?;
        self.check_return_single_response(withdraw)
    }

    pub fn buy_limit(&self, market: &str, quantity: f64, rate: f64) -> Result<BittrexUuid, BittrexError> {
        let buy_limit = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/market/buylimit?market={}&quantity={}&rate={}", API_URL, market, quantity, rate))?;
        self.check_return_single_response(buy_limit)
    }

    pub fn sell_limit(&self, market: &str, quantity: f64, rate: f64) -> Result<BittrexUuid, BittrexError> {
        let sell_limit = self.call_private_api::<BittrexAPIResult<BittrexUuid>>(&format!("{}/market/selllimit?market={}&quantity={}&rate={}", API_URL, market, quantity, rate))?;
        self.check_return_single_response(sell_limit)
    }

    pub fn cancel_order(&self, order_id: &str) -> Result<(), BittrexError> {
        self.call_private_api::<BittrexAPIResult<()>>(&format!("{}/market/cancel?uuid={}", API_URL, order_id))?;
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
        let mut resp = client.get(url)?.headers(headers).send()?;
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

        if self.http_proxy.is_some() {
            client_builder.proxy(Proxy::http(self.http_proxy.unwrap())?);
        }
        if self.https_proxy.is_some() {
            client_builder.proxy(Proxy::https(self.https_proxy.unwrap())?);
        }
        
        Ok(client_builder.build()?)
    }

    fn check_return_vec_response<T>(&self, bittrex_api_result: BittrexAPIResult<T>) -> Result<Vec<T>, BittrexError> {
        match bittrex_api_result.success {
            true => Ok(bittrex_api_result.result),
            false => Err(BittrexError { error_type: BittrexErrorType::APIError, message: bittrex_api_result.message })
        }
    }

    fn check_return_single_response<T>(&self, mut bittrex_api_result: BittrexAPIResult<T>) -> Result<T, BittrexError> {
        match bittrex_api_result.success {
            true => match bittrex_api_result.result.len() {
                1 => Ok(bittrex_api_result.result.remove(0)),
                0 => Err(BittrexError { error_type: BittrexErrorType::NoResults, message: "Maybe check your parameters?".to_string() }),
                _ => Err(BittrexError { error_type: BittrexErrorType::APIError, message: "Multiple results found! Maybe check your parameters?".to_string() })
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

#[cfg(test)]
mod tests {
    use mockito::mock;
    use super::BittrexAPI;

    #[test]
    fn should_get_markets_successfully() {
        // Arrange
        let _mock = mock("GET", "/public/getmarkets")
            .with_status(200)
            .with_body(r#"{
                "success" : true,
                "message" : "",
                "result" : [{
                        "MarketCurrency" : "LTC",
                        "BaseCurrency" : "BTC",
                        "MarketCurrencyLong" : "Litecoin",
                        "BaseCurrencyLong" : "Bitcoin",
                        "MinTradeSize" : 0.01000000,
                        "MarketName" : "BTC-LTC",
                        "IsActive" : true,
                        "Created" : "2014-02-13T00:00:00"
                    }, {
                        "MarketCurrency" : "DOGE",
                        "BaseCurrency" : "BTC",
                        "MarketCurrencyLong" : "Dogecoin",
                        "BaseCurrencyLong" : "Bitcoin",
                        "MinTradeSize" : 100.00000000,
                        "MarketName" : "BTC-DOGE",
                        "IsActive" : true,
                        "Created" : "2014-02-13T00:00:00"
                    }
                ]
            }"#)
            .create();
        let bittrex_api = BittrexAPI::new("KEY", "SECRET");

        // Act
        let markets = bittrex_api.get_markets().unwrap();

        // Assert
        assert_eq!(markets.len(), 2);
        assert_eq!(markets[0].market_currency, "LTC");
    }
}