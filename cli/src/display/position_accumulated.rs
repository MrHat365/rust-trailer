
// use colored::*;
use trailer::models::*;
// use trailer::error::*;

use super::colored_number;

pub fn row_header() -> String {
    format!("{symbol:12}{trade_type:6}{pos_size:22}{percent_change:40}{sale_price:16}{price:16}{cost_usd:16}{balance:16}{profit:16}",
        symbol = "symbol",
        trade_type = "type",
        pos_size = "size",
        percent_change = "% change",
        sale_price = "sale price",
        price = "cur price",
        cost_usd = "cost usd",
        balance = "balance",
        profit = "profit")
}

pub fn row(position: PositionAccumulated) -> String {
    // format!("{symbol:12}{trade_type:<12}{profit:<16}",
    //     symbol        = position.position.symbol.yellow(),
    //     trade_type    = position.position.trade_type.colored_string(),
    //     profit        = colored_number(
    //                     position.btc_profit,
    //                     format!("{:.2}% (${:.2}, {:.3} btc)", position.btc_profit, position.usd_profit, position.btc_profit))
    // )

    let p = position.position;

    format!("{symbol:12}{trade_type:<6}{pos_size:<22}{percent_change:<40}{sale_price:<16}{price:<16}{cost_usd:<16}{balance:<16}{profit:<16}",
        symbol                      = p.symbol,
        trade_type                  = p.trade_type.colored_string(),
        pos_size                    = format!("{:.2} ({:.2} btc)", p.qty, p.cost_btc),
        percent_change              = colored_number(
                                        p.potential_profit_percent,
                                        format!("{:.2}% (${:.2}, {:.8} btc)", p.potential_profit_percent, p.potential_profit_usd, p.potential_profit_btc)),
        sale_price                  = format!("{:.8}",  p.sale_price),
        price                       = format!("{:.8}",  p.current_price),
        cost_usd                    = format!("${:.2}", p.cost_usd),
        balance                     = p.balance,
        profit                      = colored_number(
                                        position.unrealised_pnl,
                                        format!("{:.3} btc", position.unrealised_pnl)))

    // format!("{symbol:12}{trade_type:<12}{profit:<16}",
    //     symbol        = position.position.symbol.yellow(),
    //     trade_type    = position.position.trade_type.colored_string(),
    //     profit        = colored_number(
    //                     position.btc_profit,
    //                     format!("{:.3} btc", position.btc_profit))
    // )
}

// pub fn row_compact(position: Position) -> String {
//     format!("{symbol:12}{trade_type:<12}{cost_btc:<12}{percent_change:<20}{qty:<12.2}",
//         symbol                      = position.symbol,
//         trade_type                  = position.trade_type.colored_string(),
//         cost_btc                    = format!("{:.2}",  position.cost_btc),
//         percent_change              = colored_number(position.potential_profit_percent,     format!("{:.2}% (${:.2})", position.potential_profit_percent, position.potential_profit_usd)),
//         qty                         = position.qty)
// }