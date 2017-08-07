use time;
use std;
use std::str;

use hmac::{Hmac, Mac, MacResult};
use sha2::Sha512;
use generic_array::typenum::U64;

use reqwest::{Client, Proxy};
use reqwest::header::Headers;

use serde;
use serde_json;

use bittrex_error::{ BittrexError, BittrexErrorType };
use bittrex_values::*;

const API_URL: &str = "https://bittrex.com/api/v1.1";

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
    pub fn get_markets(&self) -> Result<Vec<BittrexMarket>, BittrexError> {
        let markets = self.call_public_api::<BittrexAPIResult<BittrexMarket>>(&format!("{}/public/getmarkets", API_URL))?;
        self.check_return_array_response(markets)
    }

    /// Returns all available currencies
    pub fn get_currencies(&self) -> Result<Vec<BittrexCurrency>, BittrexError> {
        let currencies = self.call_public_api::<BittrexAPIResult<BittrexCurrency>>(&format!("{}/public/getcurrencies", API_URL))?;
        self.check_return_array_response(currencies)
    }

    pub fn get_ticker(&self, market: &str) -> Result<BittrexTicker, BittrexError> {
        let ticker = self.call_public_api::<BittrexAPIResult<BittrexTicker>>(&format!("{}/public/getticker?market={}", API_URL, market))?;
        self.check_return_single_response(ticker)        
    }

    pub fn get_market_summaries(&self) -> Result<Vec<BittrexMarketSummary>, BittrexError> {
        let summaries = self.call_public_api::<BittrexAPIResult<BittrexMarketSummary>>(&format!("{}/public/getmarketsummaries", API_URL))?;
        self.check_return_array_response(summaries)
    }

    pub fn get_market_summary(&self, market: &str) -> Result<BittrexMarketSummary, BittrexError> {
        let summary = self.call_public_api::<BittrexAPIResult<BittrexMarketSummary>>(&format!("{}/public/getticker?getmarketsummary?market={}", API_URL, market))?;
        self.check_return_single_response(summary)
    }

    pub fn get_order_book(&self, market: &str, book_type: &str) -> Result<BittrexPublicOrderBook, BittrexError> {
        let order_book = self.call_public_api::<BittrexAPIResult<BittrexPublicOrderBook>>(&format!("{}/public/getticker?getorderbook=?market={}&type={}", API_URL, market, book_type))?;
        self.check_return_single_response(order_book)
    }

    /*pub fn get_market_history(&self, market: &str) -> String {
        self.call_public_api(&format!("{}/public/getmarkethistory?market={}", API_URL, market))
    }

    pub fn get_open_orders(&self, market: &str) -> String {
        self.call_private_api(&format!("{}/market/getopenorders?market={}", API_URL, market))
    }

    pub fn get_order(&self, order_id: &str) -> String {
        self.call_private_api(&format!("{}/account/getorder?uuid={}", API_URL, order_id))
    }

    pub fn get_order_history(&self) -> String {
        self.call_private_api(&format!("{}/account/getorderhistory?", API_URL))
    }

    pub fn get_withdrawal_history(&self, currency: &str) -> String {
        self.call_private_api(&format!("{}/account/getwithdrawalhistory?currency={}", API_URL, currency))
    }

    pub fn get_deposit_history(&self, currency: &str) -> String {
        self.call_private_api(&format!("{}/account/getdeposithistory?currency={}", API_URL, currency))
    }

    pub fn get_balances(&self) -> String {
        self.call_private_api(&format!("{}/account/getbalances?", API_URL))
    }

    pub fn get_balance(&self, currency: &str) -> String {
        self.call_private_api(&format!("{}/account/getbalance?currency={}", API_URL, currency))
    }

    pub fn get_deposit_address(&self, currency: &str) -> String {
        self.call_private_api(&format!("{}/account/getdepositaddress?currency={}", API_URL, currency))
    }

    pub fn withdraw(&self, currency: &str, quantity: f64, address: &str, payment_id: &str) {
        self.call_private_api(&format!("{}/account/withdraw?currency={}&quantity={}&address={}&paymentid={}", API_URL, currency, quantity, address, payment_id));
    }

    pub fn buy_limit(&self, market: &str, quantity: f64, rate: f64) {
        self.call_private_api(&format!("{}/market/buylimit?market={}&quantity={}&rate={}", API_URL, market, quantity, rate));
    }

    pub fn sell_limit(&self, market: &str, quantity: f64, rate: f64) {
        self.call_private_api(&format!("{}/market/selllimit?market={}&quantity={}&rate={}", API_URL, market, quantity, rate));
    }

    pub fn cancel_order(&self, order_id: &str) {
        self.call_private_api(&format!("{}/market/cancel?uuid={}", API_URL, order_id));
    }
*/
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

    fn check_return_array_response<T>(&self, bittrex_api_result: BittrexAPIResult<T>) -> Result<Vec<T>, BittrexError> {
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
    use super::BittrexAPI;
    
    #[test]
    fn it_works() {
        let bittrex_api = BittrexAPI::new("", "");
        let currencies = bittrex_api.get_currencies().unwrap();

        println!("{}", currencies[0]);

        let markets = bittrex_api.get_markets().unwrap();

        println!("{}", markets[0]);
    }
}
