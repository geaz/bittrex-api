extern crate mockito;
extern crate bittrex_api;

use mockito::{mock, Matcher};
use bittrex_api::BittrexClient;
use bittrex_api::values::BittrexOrderType;

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
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let markets = bittrex_client.get_markets().unwrap();

    // Assert
    assert_eq!(markets.len(), 2);
    assert_eq!(markets[0].market_currency, "LTC");
}

#[test]
fn should_get_currencies_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getcurrencies")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "Currency" : "BTC",
                    "CurrencyLong" : "Bitcoin",
                    "MinConfirmation" : 2,
                    "TxFee" : 0.00020000,
                    "IsActive" : true,
                    "CoinType" : "BITCOIN",
                    "BaseAddress" : null
                }, {
                    "Currency" : "LTC",
                    "CurrencyLong" : "Litecoin",
                    "MinConfirmation" : 5,
                    "TxFee" : 0.00200000,
                    "IsActive" : true,
                    "CoinType" : "BITCOIN",
                    "BaseAddress" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let currencies = bittrex_client.get_currencies().unwrap();

    // Assert
    assert_eq!(currencies.len(), 2);
    assert_eq!(currencies[0].currency, "BTC");
    assert_eq!(currencies[0].base_address, None);
}

#[test]
fn should_get_valid_ticker_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getticker?market=BTC-LTC")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                "Bid" : 2.05670368,
                "Ask" : 3.35579531,
                "Last" : 3.35579531
            }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let ticker = bittrex_client.get_ticker("BTC-LTC").unwrap();

    // Assert
    assert_eq!(ticker.bid, 2.05670368);
    assert_eq!(ticker.ask, 3.35579531);
    assert_eq!(ticker.last, 3.35579531);
}

#[test]
#[should_panic(expected="INVALID_MARKET")]
fn should_handle_invalid_ticker_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getticker?market=BT-LT")
        .with_status(200)
        .with_body(r#"{"success":false,"message":"INVALID_MARKET","result":null}"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    bittrex_client.get_ticker("BT-LT").unwrap();
}

#[test]
fn should_get_market_summaries_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getmarketsummaries")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "MarketName" : "BTC-888",
                    "High" : 0.00000919,
                    "Low" : 0.00000820,
                    "Volume" : 74339.61396015,
                    "Last" : 0.00000820,
                    "BaseVolume" : 0.64966963,
                    "TimeStamp" : "2014-07-09T07:19:30.15",
                    "Bid" : 0.00000820,
                    "Ask" : 0.00000831,
                    "OpenBuyOrders" : 15,
                    "OpenSellOrders" : 15,
                    "PrevDay" : 0.00000821,
                    "Created" : "2014-03-20T06:00:00",
                    "DisplayMarketName" : null
                }, {
                    "MarketName" : "BTC-A3C",
                    "High" : 0.00000072,
                    "Low" : 0.00000001,
                    "Volume" : 166340678.42280999,
                    "Last" : 0.00000005,
                    "BaseVolume" : 17.59720424,
                    "TimeStamp" : "2014-07-09T07:21:40.51",
                    "Bid" : 0.00000004,
                    "Ask" : 0.00000005,
                    "OpenBuyOrders" : 18,
                    "OpenSellOrders" : 18,
                    "PrevDay" : 0.00000002,
                    "Created" : "2014-05-30T07:57:49.637",
                    "DisplayMarketName" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let summaries = bittrex_client.get_market_summaries().unwrap();

    // Assert
    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].market_name, "BTC-888");
    assert_eq!(summaries[0].last, 0.00000820);
}

#[test]
fn should_get_market_summary_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getmarketsummary?market=btc-ltc")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "MarketName" : "BTC-LTC",
                    "High" : 0.01350000,
                    "Low" : 0.01200000,
                    "Volume" : 3833.97619253,
                    "Last" : 0.01349998,
                    "BaseVolume" : 47.03987026,
                    "TimeStamp" : "2014-07-09T07:22:16.72",
                    "Bid" : 0.01271001,
                    "Ask" : 0.01291100,
                    "OpenBuyOrders" : 45,
                    "OpenSellOrders" : 45,
                    "PrevDay" : 0.01229501,
                    "Created" : "2014-02-13T00:00:00",
                    "DisplayMarketName" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let summary = bittrex_client.get_market_summary("btc-ltc").unwrap();

    // Assert
    assert_eq!(summary.market_name, "BTC-LTC");
}

#[test]
fn should_get_order_book_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getorderbook?market=BTC-LTC&type=Both")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                "buy" : [{
                        "Quantity" : 12.37000000,
                        "Rate" : 0.02525000
                    }
                ],
                "sell" : [{
                        "Quantity" : 32.55412402,
                        "Rate" : 0.02540000
                    }, {
                        "Quantity" : 60.00000000,
                        "Rate" : 0.02550000
                    }, {
                        "Quantity" : 60.00000000,
                        "Rate" : 0.02575000
                    }, {
                        "Quantity" : 84.00000000,
                        "Rate" : 0.02600000
                    }
                ]
            }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let summary = bittrex_client.get_order_book("BTC-LTC", BittrexOrderType::Both).unwrap();

    // Assert
    assert_eq!(summary.buy.len(), 1);
    assert_eq!(summary.buy[0].quantity, 12.37000000);
    assert_eq!(summary.sell.len(), 4);
}

#[test]
fn should_get_market_history_successfully() {
    // Arrange
    let _mock = mock("GET", "/public/getmarkethistory?market=BTC-DOGE")
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "Id" : 319435,
                    "TimeStamp" : "2014-07-09T03:21:20.08",
                    "Quantity" : 0.30802438,
                    "Price" : 0.01263400,
                    "Total" : 0.00389158,
                    "FillType" : "FILL",
                    "OrderType" : "BUY"
                }, {
                    "Id" : 319433,
                    "TimeStamp" : "2014-07-09T03:21:20.08",
                    "Quantity" : 0.31820814,
                    "Price" : 0.01262800,
                    "Total" : 0.00401833,
                    "FillType" : "PARTIAL_FILL",
                    "OrderType" : "BUY"
                }, {
                    "Id" : 319379,
                    "TimeStamp" : "2014-07-09T02:58:48.127",
                    "Quantity" : 49.64643541,
                    "Price" : 0.01263200,
                    "Total" : 0.62713377,
                    "FillType" : "FILL",
                    "OrderType" : "SELL"
                }, {
                    "Id" : 319378,
                    "TimeStamp" : "2014-07-09T02:58:46.27",
                    "Quantity" : 0.35356459,
                    "Price" : 0.01263200,
                    "Total" : 0.00446622,
                    "FillType" : "PARTIAL_FILL",
                    "OrderType" : "BUY"
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let history = bittrex_client.get_market_history("BTC-DOGE").unwrap();

    // Assert
    assert_eq!(history.len(), 4);
    assert_eq!(history[0].quantity, 0.30802438);
}

#[test]
fn should_buy_limit_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/market/buylimit\?market=BTC-LTC&quantity=1.2&rate=1.3(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                    "uuid" : "e606d53c-8d70-11e3-94b5-425861b86ab6"
                }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let buy_limit = bittrex_client.buy_limit("BTC-LTC", 1.2, 1.3).unwrap();

    // Assert
    assert_eq!(buy_limit.uuid, "e606d53c-8d70-11e3-94b5-425861b86ab6".to_string());
}

#[test]
fn should_sell_limit_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/market/selllimit\?market=BTC-LTC&quantity=1.2&rate=1.3(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                    "uuid" : "e606d53c-8d70-11e3-94b5-425861b86ab6"
                }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let sell_limit = bittrex_client.sell_limit("BTC-LTC", 1.2, 1.3).unwrap();

    // Assert
    assert_eq!(sell_limit.uuid, "e606d53c-8d70-11e3-94b5-425861b86ab6".to_string());
}

#[test]
fn should_cancel_order_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/market/cancel\?uuid=e606d53c-8d70-11e3-94b5-425861b86ab6(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : null
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    bittrex_client.cancel_order("e606d53c-8d70-11e3-94b5-425861b86ab6").unwrap();
}

#[test]
fn should_get_open_orders_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/market/getopenorders\?&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "Uuid" : null,
                    "OrderUuid" : "09aa5bb6-8232-41aa-9b78-a5a1093e0211",
                    "Exchange" : "BTC-LTC",
                    "OrderType" : "LIMIT_SELL",
                    "Quantity" : 5.00000000,
                    "QuantityRemaining" : 5.00000000,
                    "Limit" : 2.00000000,
                    "CommissionPaid" : 0.00000000,
                    "Price" : 0.00000000,
                    "PricePerUnit" : null,
                    "Opened" : "2014-07-09T03:55:48.77",
                    "Closed" : null,
                    "CancelInitiated" : false,
                    "ImmediateOrCancel" : false,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null
                }, {
                    "Uuid" : null,
                    "OrderUuid" : "8925d746-bc9f-4684-b1aa-e507467aaa99",
                    "Exchange" : "BTC-LTC",
                    "OrderType" : "LIMIT_BUY",
                    "Quantity" : 100000.00000000,
                    "QuantityRemaining" : 100000.00000000,
                    "Limit" : 0.00000001,
                    "CommissionPaid" : 0.00000000,
                    "Price" : 0.00000000,
                    "PricePerUnit" : null,
                    "Opened" : "2014-07-09T03:55:48.583",
                    "Closed" : null,
                    "CancelInitiated" : false,
                    "ImmediateOrCancel" : false,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let open_orders = bittrex_client.get_open_orders().unwrap();

    // Assert
    assert_eq!(open_orders.len(), 2);
    assert_eq!(open_orders[0].quantity, 5.0);
}

#[test]
fn should_get_open_orders_by_market_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/market/getopenorders\?market=BTC-LTC(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "Uuid" : null,
                    "OrderUuid" : "09aa5bb6-8232-41aa-9b78-a5a1093e0211",
                    "Exchange" : "BTC-LTC",
                    "OrderType" : "LIMIT_SELL",
                    "Quantity" : 5.00000000,
                    "QuantityRemaining" : 5.00000000,
                    "Limit" : 2.00000000,
                    "CommissionPaid" : 0.00000000,
                    "Price" : 0.00000000,
                    "PricePerUnit" : null,
                    "Opened" : "2014-07-09T03:55:48.77",
                    "Closed" : null,
                    "CancelInitiated" : false,
                    "ImmediateOrCancel" : false,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let open_orders = bittrex_client.get_open_orders_by_market("BTC-LTC").unwrap();

    // Assert
    assert_eq!(open_orders.len(), 1);
    assert_eq!(open_orders[0].quantity, 5.0);
}

#[test]
fn should_get_balances_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getbalances\?&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "Currency" : "DOGE",
                    "Balance" : 0.00000000,
                    "Available" : 0.00000000,
                    "Pending" : 0.00000000,
                    "CryptoAddress" : "DLxcEt3AatMyr2NTatzjsfHNoB9NT62HiF",
                    "Requested" : false,
                    "Uuid" : null
                }, {
                    "Currency" : "BTC",
                    "Balance" : 14.21549076,
                    "Available" : 14.21549076,
                    "Pending" : 0.00000000,
                    "CryptoAddress" : "1Mrcdr6715hjda34pdXuLqXcju6qgwHA31",
                    "Requested" : false,
                    "Uuid" : null
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let balances = bittrex_client.get_balances().unwrap();

    // Assert
    assert_eq!(balances.len(), 2);
    assert_eq!(balances[0].currency, "DOGE");
}

#[test]
fn should_get_balance_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getbalance\?currency=BTC&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                "Currency" : "BTC",
                "Balance" : 14.21549076,
                "Available" : 14.21549076,
                "Pending" : 0.00000000,
                "CryptoAddress" : "1Mrcdr6715hjda34pdXuLqXcju6qgwHA31",
                "Requested" : false,
                "Uuid" : null
            }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let balance = bittrex_client.get_balance("BTC").unwrap();

    // Assert
    assert_eq!(balance.currency, "BTC");
}

#[test]
fn should_get_deposit_address_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getdepositaddress\?currency=VTC&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                "Currency" : "VTC",
                "Address" : "Vy5SKeKGXUHKS2WVpJ76HYuKAu3URastUo"
            }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let deposit_address = bittrex_client.get_deposit_address("VTC").unwrap();

    // Assert
    assert_eq!(deposit_address.currency, "VTC");
    assert_eq!(deposit_address.address, "Vy5SKeKGXUHKS2WVpJ76HYuKAu3URastUo");
}

#[test]
fn should_withdraw_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/withdraw\?currency=BTC&quantity=1.2&address=ADRESS&paymentid=&(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                    "uuid" : "e606d53c-8d70-11e3-94b5-425861b86ab6"
                }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let withdraw = bittrex_client.withdraw("BTC", 1.2, "ADRESS", "").unwrap();

    // Assert
    assert_eq!(withdraw.uuid, "e606d53c-8d70-11e3-94b5-425861b86ab6".to_string());
}

#[test]
fn should_get_order_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getorder\?uuid=ORDERID(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : {
                "AccountId" : null,
                "OrderUuid" : "0cb4c4e4-bdc7-4e13-8c13-430e587d2cc1",
                "Exchange" : "BTC-SHLD",
                "Type" : "LIMIT_BUY",
                "Quantity" : 1000.00000000,
                "QuantityRemaining" : 1000.00000000,
                "Limit" : 0.00000001,
                "Reserved" : 0.00001000,
                "ReserveRemaining" : 0.00001000,
                "CommissionReserved" : 0.00000002,
                "CommissionReserveRemaining" : 0.00000002,
                "CommissionPaid" : 0.00000000,
                "Price" : 0.00000000,
                "PricePerUnit" : null,
                "Opened" : "2014-07-13T07:45:46.27",
                "Closed" : null,
                "IsOpen" : true,
                "Sentinel" : "6c454604-22e2-4fb4-892e-179eede20972",
                "CancelInitiated" : false,
                "ImmediateOrCancel" : false,
                "IsConditional" : false,
                "Condition" : "NONE",
                "ConditionTarget" : null
            }
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let order = bittrex_client.get_order("ORDERID").unwrap();

    // Assert
    assert_eq!(order.order_uuid, "0cb4c4e4-bdc7-4e13-8c13-430e587d2cc1".to_string());
}

#[test]
fn should_get_order_history_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getorderhistory\?&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "OrderUuid" : "fd97d393-e9b9-4dd1-9dbf-f288fc72a185",
                    "Exchange" : "BTC-LTC",
                    "TimeStamp" : "2014-07-09T04:01:00.667",
                    "OrderType" : "LIMIT_BUY",
                    "Limit" : 0.00000001,
                    "Quantity" : 100000.00000000,
                    "QuantityRemaining" : 100000.00000000,
                    "Commission" : 0.00000000,
                    "Price" : 0.00000000,
                    "PricePerUnit" : null,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null,
                    "ImmediateOrCancel" : false
                }, {
                    "OrderUuid" : "17fd64d1-f4bd-4fb6-adb9-42ec68b8697d",
                    "Exchange" : "BTC-ZS",
                    "TimeStamp" : "2014-07-08T20:38:58.317",
                    "OrderType" : "LIMIT_SELL",
                    "Limit" : 0.00002950,
                    "Quantity" : 667.03644955,
                    "QuantityRemaining" : 0.00000000,
                    "Commission" : 0.00004921,
                    "Price" : 0.01968424,
                    "PricePerUnit" : 0.00002950,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null,
                    "ImmediateOrCancel" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let order_history = bittrex_client.get_order_history().unwrap();

    // Assert
    assert_eq!(order_history.len(), 2);
    assert_eq!(order_history[0].order_uuid, "fd97d393-e9b9-4dd1-9dbf-f288fc72a185");
}

#[test]
fn should_get_order_history_by_market_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getorderhistory\?market=BTC-LTC&(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "OrderUuid" : "fd97d393-e9b9-4dd1-9dbf-f288fc72a185",
                    "Exchange" : "BTC-LTC",
                    "TimeStamp" : "2014-07-09T04:01:00.667",
                    "OrderType" : "LIMIT_BUY",
                    "Limit" : 0.00000001,
                    "Quantity" : 100000.00000000,
                    "QuantityRemaining" : 100000.00000000,
                    "Commission" : 0.00000000,
                    "Price" : 0.00000000,
                    "PricePerUnit" : null,
                    "IsConditional" : false,
                    "Condition" : null,
                    "ConditionTarget" : null,
                    "ImmediateOrCancel" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let order_history = bittrex_client.get_order_history_by_market("BTC-LTC").unwrap();

    // Assert
    assert_eq!(order_history.len(), 1);
    assert_eq!(order_history[0].order_uuid, "fd97d393-e9b9-4dd1-9dbf-f288fc72a185");
}

#[test]
fn should_get_withdrawal_history_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getwithdrawalhistory\?&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "PaymentUuid" : "b52c7a5c-90c6-4c6e-835c-e16df12708b1",
                    "Currency" : "BTC",
                    "Amount" : 17.00000000,
                    "Address" : "1DeaaFBdbB5nrHj87x3NHS4onvw1GPNyAu",
                    "Opened" : "2014-07-09T04:24:47.217",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00020000,
                    "TxId" : null,
                    "Canceled" : true,
                    "InvalidAddress" : false
                }, {
                    "PaymentUuid" : "f293da98-788c-4188-a8f9-8ec2c33fdfcf",
                    "Currency" : "XC",
                    "Amount" : 7513.75121715,
                    "Address" : "XVnSMgAd7EonF2Dgc4c9K14L12RBaW5S5J",
                    "Opened" : "2014-07-08T23:13:31.83",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00002000,
                    "TxId" : "b4a575c2a71c7e56d02ab8e26bb1ef0a2f6cf2094f6ca2116476a569c1e84f6e",
                    "Canceled" : false,
                    "InvalidAddress" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let withdrawal_history = bittrex_client.get_withdrawal_history().unwrap();

    // Assert
    assert_eq!(withdrawal_history.len(), 2);
    assert_eq!(withdrawal_history[0].payment_uuid, "b52c7a5c-90c6-4c6e-835c-e16df12708b1");
}

#[test]
fn should_get_withdrawal_history_by_currency_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getwithdrawalhistory\?currency=BTC(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "PaymentUuid" : "b52c7a5c-90c6-4c6e-835c-e16df12708b1",
                    "Currency" : "BTC",
                    "Amount" : 17.00000000,
                    "Address" : "1DeaaFBdbB5nrHj87x3NHS4onvw1GPNyAu",
                    "Opened" : "2014-07-09T04:24:47.217",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00020000,
                    "TxId" : null,
                    "Canceled" : true,
                    "InvalidAddress" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let withdrawal_history = bittrex_client.get_withdrawal_history_by_currency("BTC").unwrap();

    // Assert
    assert_eq!(withdrawal_history.len(), 1);
    assert_eq!(withdrawal_history[0].payment_uuid, "b52c7a5c-90c6-4c6e-835c-e16df12708b1");
}

#[test]
fn should_get_deposit_history_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getdeposithistory\?&apikey=(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "PaymentUuid" : "554ec664-8842-4fe9-b491-06225becbd59",
                    "Currency" : "BTC",
                    "Amount" : 0.00156121,
                    "Address" : "1K37yQZaGrPKNTZ5KNP792xw8f7XbXxetE",
                    "Opened" : "2014-07-11T03:41:25.323",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00020000,
                    "TxId" : "70cf6fdccb9bd38e1a930e13e4ae6299d678ed6902da710fa3cc8d164f9be126",
                    "Canceled" : false,
                    "InvalidAddress" : false
                }, {
                    "PaymentUuid" : "d3fdf168-3d8e-40b6-8fe4-f46e2a7035ea",
                    "Currency" : "BTC",
                    "Amount" : 0.11800000,
                    "Address" : "1Mrcar6715hjds34pdXuLqXcju6QgwHA31",
                    "Opened" : "2014-07-03T20:27:07.163",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00020000,
                    "TxId" : "3efd41b3a051433a888eed3ecc174c1d025a5e2b486eb418eaaec5efddda22de",
                    "Canceled" : false,
                    "InvalidAddress" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let deposit_history = bittrex_client.get_deposit_history().unwrap();

    // Assert
    assert_eq!(deposit_history.len(), 2);
    assert_eq!(deposit_history[0].payment_uuid, "554ec664-8842-4fe9-b491-06225becbd59");
}

#[test]
fn should_get_deposit_history_by_currency_successfully() {
    // Arrange
    let _mock = mock("GET", Matcher::Regex(r"^/account/getdeposithistory\?currency=BTC(.*)$".to_string()))
        .with_status(200)
        .with_body(r#"{
            "success" : true,
            "message" : "",
            "result" : [{
                    "PaymentUuid" : "554ec664-8842-4fe9-b491-06225becbd59",
                    "Currency" : "BTC",
                    "Amount" : 0.00156121,
                    "Address" : "1K37yQZaGrPKNTZ5KNP792xw8f7XbXxetE",
                    "Opened" : "2014-07-11T03:41:25.323",
                    "Authorized" : true,
                    "PendingPayment" : false,
                    "TxCost" : 0.00020000,
                    "TxId" : "70cf6fdccb9bd38e1a930e13e4ae6299d678ed6902da710fa3cc8d164f9be126",
                    "Canceled" : false,
                    "InvalidAddress" : false
                }
            ]
        }"#)
        .create();
    let bittrex_client = BittrexClient::new_override_api_url("KEY".to_string(), "SECRET".to_string(), mockito::SERVER_URL.to_string());

    // Act
    let deposit_history = bittrex_client.get_deposit_history_by_currency("BTC").unwrap();

    // Assert
    assert_eq!(deposit_history.len(), 1);
    assert_eq!(deposit_history[0].payment_uuid, "554ec664-8842-4fe9-b491-06225becbd59");
}