#![feature(vec_remove_item)]

use std::io;

fn main() {
    let mut market = new_market(100, 8, 1000, 10);
    let starting_shares: u32 = 100;
    let starting_funds: u32 = 100;
    market = market.new_firm("the founder-owned firm".into(), starting_shares.clone(), starting_funds.clone());
    market = market.new_firm("the shareholder-owned firm".into(), starting_shares.clone(), starting_funds.clone());
    market = market.new_firm("the stakeholder-owned firm".into(), starting_shares.clone(), starting_funds.clone());
    market.clone().display_firms();
    let mut done = false;
    while !done {
        market = market.play_round();
        market = market.end_round();
    }
}

// the three strategies
    // 1. founder-owned
        // owned by employees
        // will do what leads to highest wages

    // 2. shareholder-owned
        // (doesn't start as shareholder-owned: sells shares to become that way)
        // owned by a mix of employees and shareholders
        // will do what leads to highest wages and share price (depending on ownership breakdown)

    // 3. stakeholder-owned
        // owned by a mix of employees, shareholders, and users
        // will do what leads to highest wages, share price, and user preference-fulfillment 
        // (depending on ownership breakdown) 

// decision-flow:
    // present a choice
    // look at ownership
    // decide what to do

// initial choice: set the parameters of firm.play_round()
    // 

// we cooooould try instead of a head to head competition, a "time trial" where the different strategies race to sell to the whole market
    // the advantage is that it would be much easier to program the strategies
    // but, does it really prove the point?

// recording playthroughs and pitting them against each other doesn't work, because interactions change depending on the other players dynamically

pub fn new_market(labor_s: u32, minimum_w: u32, users_: u32, min_u: u32) -> Market {
    Market {
        firms: vec![],
        labor_supply: labor_s,
        workers_left: labor_s,
        min_wage: minimum_w,
        users: users_,
        users_left: users_,
        min_usefulness_per_user: min_u,
    }
}

#[derive(Clone)]
pub struct Market {
    firms: Vec<Firm>,
    labor_supply: u32,
    workers_left: u32,
    min_wage: u32,
    users: u32,
    users_left: u32,
    min_usefulness_per_user: u32,
}

impl Market {
    pub fn new_firm(mut self, name_: String, starting_shares: u32, starting_funds: u32) -> Market {
        // TODO only allows new firms with names not already taken
        let new_firm = Firm {
            name: name_,

            shares_remaining: starting_shares,
            shares_to_sell: 0,
            share_market_boost: 0,
            share_price: 0,

            employees: 0,
            number_to_hire: 0,
            wage_amount: 0,

            current_funds: starting_funds,
            previous_funds: vec![],
            profit_trend: 0,

            usefulness: 0,
            servings: 0,

            user_preference_fulfillment: 0,
            employee_preference_fulfillment: 0,
        };

        self.firms.push(new_firm);
        return self
    }

    pub fn play_round(self) -> Market {
        &self.display();
        println!("Playing round...");
        let mut new_market = self.clone();
        new_market.firms = vec![];
        for mut firm in self.firms.into_iter() {
            firm = firm.play_round();
            new_market.firms.push(firm);
        }
        return new_market
    }

    pub fn end_round(self) -> Market {
        println!("Resolving round...");
        let mut market: Market = self.sell_goods();
        market = market.pay_employees();
        market = market.set_share_prices();
        market = market.sell_shares();
        market = market.recruit_employees();
        market.clone().display_firms();
        return market
    }

    pub fn display(&self) {
        println!("Displaying market");
        println!("-- workers available for hire: {}", self.labor_supply);
        println!("-- minimum wage workers will take: {}", self.min_wage);
        println!("-- number of users seeking to buy a product: {}", self.users);
        println!("-- minimum quality of product users will buy: {}", self.min_usefulness_per_user);
    }

    pub fn display_firms(self) {
        for firm in self.firms.into_iter() {
            firm.display();
        }
    }

    // the following five functions are run at the end of every round (in this order)

    pub fn sell_goods(mut self) -> Market {
        // reads the usefulness created by each firm the previous round and has users buy the most useful products
        // assumes each user has a standard ideal usefulness (e.g. each person wants exactly one car, but the best car)
        // firms that created more usefulness will get more customers
        // since there is a limited number of users, some firms may fail to "sell" all of their usefulness
        // firms receive funds proportional to their sales (not to their usefulness)
        // firms receive user_preference fulfillment proportional to the usefulness dispursed (bought by users)
        let mut c: usize = 0;
        let mut firms: Vec<(usize, Firm)> = self.clone().firms.into_iter().map(|f| { c += 1; (c, f) }).collect();
        firms.sort_by(|a, b| b.1.usefulness.cmp(&a.1.usefulness));
        for firm_data in firms {
            self.firms[firm_data.0 - 1].previous_funds.insert(0, firm_data.1.current_funds.clone());
            let usefulness: u32 = self.firms[firm_data.0 - 1].usefulness.clone();
            let servings: u32 = self.firms[firm_data.0 - 1].servings.clone();
            if usefulness >= self.min_usefulness_per_user {
                if self.users_left >= servings {
                    self.firms[firm_data.0 - 1].user_preference_fulfillment = &usefulness * &servings;
                    self.firms[firm_data.0 - 1].current_funds += 10 * &servings;
                    self.users_left -= servings;
                } else {
                    println!("There are not enough unsatisfied users left for {} to sell to.", &self.firms[firm_data.0 - 1].name);
                    self.firms[firm_data.0 - 1].user_preference_fulfillment = &usefulness * &self.users_left;
                    self.firms[firm_data.0 - 1].current_funds += 10 * &self.users_left;
                    self.users_left = 0;
                }
            } else {
                continue
            }
        }
        self.users_left = self.users;
        return self
    }

    pub fn pay_employees(self) -> Market {
        // for each firm, reads wage_amount and employees, then pays each
        // employee that amount out of current_funds. If current_funds goes below 0,
        // that firm has lost.
        let mut new_market = self.clone();
        new_market.firms = vec![];
        for mut firm in self.firms.into_iter() {
            let employee_pay: u32 = &firm.wage_amount * &firm.employees;
            println!("{} paying employees {}", &firm.name, &employee_pay);
            if employee_pay <= firm.current_funds {
                firm.current_funds = firm.current_funds - employee_pay;
                new_market.firms.push(firm);
            } else {
                println!("The firm '{}' has run out of funds. Game over.", &firm.name);
            }
        }
        return new_market
    }

    pub fn set_share_prices(self) -> Market {
        // for each firm, sets share_price proportional to current_funds,
        // profit trend, and share_market_boost
        let mut new_market = self.clone();
        new_market.firms = vec![];
        for mut firm in self.firms.into_iter() {
            firm.share_price = ( firm.current_funds + firm.profit_trend as u32 + firm.share_market_boost ) / 100;
            new_market.firms.push(firm);
        }
        return new_market
    }

    pub fn sell_shares(self) -> Market {
        // reads share_price, shares_remaining, and shares_to_sell from each firm
        // and sells that many shares for that price, generating funds for the firms
        let mut new_market = self.clone();
        new_market.firms = vec![];
        for mut firm in self.firms.into_iter() {
            if firm.shares_to_sell <= firm.shares_remaining {
                println!("Selling {} shares for {} total revenue", &firm.share_price, &firm.share_price * &firm.shares_to_sell);
                firm.current_funds += firm.share_price * firm.shares_to_sell;
                firm.shares_remaining -= firm.shares_to_sell;
                firm.profit_trend = calc_profit_trend(firm.current_funds.clone(), firm.previous_funds.clone());
                new_market.firms.push(firm);
            } else {
                println!("{} does not have enough shares remaining.", &firm.name);
                firm.current_funds += firm.share_price * firm.shares_remaining;
                firm.shares_remaining = 0;
                firm.profit_trend = calc_profit_trend(firm.current_funds.clone(), firm.previous_funds.clone());
                new_market.firms.push(firm);
            }
        }
        return new_market
    }
    
    pub fn recruit_employees(mut self) -> Market {
        // reads wage_amount and number_to_hire for each firm, 
        // then assigns employees to the firms according to who is paying most 
        // (and paying more than the minimum)
        let mut c: usize = 0;
        let mut firms: Vec<(usize, Firm)> = self.clone().firms.into_iter().map(|f| { c += 1; (c, f) }).collect();
        firms.sort_by(|a, b| b.1.wage_amount.cmp(&a.1.wage_amount));
        for firm_data in firms {
            let wage_amount: u32 = self.firms[firm_data.0 - 1].wage_amount.clone();
            let number_to_hire: u32 = self.firms[firm_data.0 - 1].number_to_hire.clone();
            if wage_amount >= self.min_wage {
                if self.workers_left >= number_to_hire {
                    self.firms[firm_data.0 - 1].employee_preference_fulfillment = &wage_amount * &number_to_hire;
                    self.firms[firm_data.0 - 1].employees = number_to_hire;
                    self.workers_left -= number_to_hire;
                } else {
                    println!("There are not enough workers left for {} to hire.", &self.firms[firm_data.0 - 1].name);
                    self.firms[firm_data.0 - 1].employee_preference_fulfillment = &wage_amount * &self.workers_left;
                    self.firms[firm_data.0 - 1].employees = self.workers_left.clone();
                    self.workers_left = 0;
                }
            } else {
                continue
            }
        }
        self.workers_left = self.labor_supply;
        return self
    }
}

#[derive(Clone)]
pub struct Firm {
    name: String,

    shares_remaining: u32,
    shares_to_sell: u32,
    share_market_boost: u32,
    share_price: u32,

    employees: u32,
    number_to_hire: u32,
    wage_amount: u32,

    current_funds: u32,
    previous_funds: Vec<u32>,
    profit_trend: i32,

    usefulness: u32,
    servings: u32,

    user_preference_fulfillment: u32,
    employee_preference_fulfillment: u32,
}

impl Firm {
    pub fn play_round(mut self) -> Firm {
        println!("Gathering inputs for '{}'...", &self.name);
        let inputs: Vec<u32> = get_firm_inputs();
        let funds_to_wages_percentage = inputs[0];
        let number_to_hire = inputs[1];
        let wage_to_usefulness = inputs[2];
        let servings = inputs[3];
        let wage_to_pumping = inputs[4];
        let shares_to_sell = inputs[5];

        assert!(servings > 0 as u32, "Servings must be > 0.");
        assert!(0 as u32 <= funds_to_wages_percentage && funds_to_wages_percentage <= 100 as u32, "Funds to wage percentage must be between 0 and 100, inclusive.");
        assert!(&wage_to_usefulness + &wage_to_pumping == 100, "Wage portions must sum to 100.");

        self.wage_amount = &self.current_funds * &funds_to_wages_percentage / 100 / &number_to_hire;
        self.number_to_hire = number_to_hire;
        self.usefulness = &self.employees * wage_to_usefulness / &servings;
        self.servings = servings;
        self.share_market_boost = &self.employees * wage_to_pumping;
        self.shares_to_sell = shares_to_sell;
        return self
    }

    pub fn display(&self) {
        println!("Displaying firm '{}'", &self.name);
        println!("-- shares remaining: {}", &self.shares_remaining);
        println!("-- share price: {}", &self.share_price);
        println!("-- employees: {}", &self.employees);
        println!("-- current funds: {}", &self.current_funds);
        println!("-- previous funds: {:?}", &self.previous_funds);
        println!("-- profit trend: {}", &self.profit_trend);
        println!("-- user preference-fulfillment: {}", &self.user_preference_fulfillment);
        println!("-- employee preference-fulfillment: {}", &self.user_preference_fulfillment);
    }
}

impl PartialEq for Firm {
    fn eq(&self, other: &Firm) -> bool {
        self.name == other.name
    }
}

// Helpers

pub fn get_input(prompt: &str) -> String {
    println!("{}",prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Cannot read input.");
    input.trim().to_string()
}

pub fn get_firm_inputs() -> Vec<u32> {
    let mut inputs: Vec<u32> = vec![];
    inputs.push(get_input("Enter percentage of funds to spend on wages (integer from 0..100):").parse().unwrap());
    inputs.push(get_input("Enter number of workers to employ:").parse().unwrap());
    let percentage: u32 = get_input("Enter percentage of wages to use producing goods (integer from 0..100):").parse().unwrap();
    inputs.push(percentage.clone());
    inputs.push(get_input("Enter the number of goods to produce (must be more than 0):").parse().unwrap());
    println!("Setting percentage of wages to use pumping share price to {}.", 100 - &percentage);
    inputs.push(100 - percentage);
    inputs.push(get_input("Enter number of shares to sell:").parse().unwrap());
    return inputs
}

pub fn calc_profit_trend(current_funds: u32, previous_funds: Vec<u32>) -> i32 {
    let first: i32 = current_funds as i32 - previous_funds[0].clone() as i32;
    if previous_funds.len() > 2 {
       let second = previous_funds[0] as i32 - previous_funds[1].clone() as i32;
       let third = previous_funds[1] as i32 - previous_funds[2] as i32;
       return (first + second + third) / 3 as i32
    }
    if previous_funds.len() > 1 {
       let second = previous_funds[0] as i32 - previous_funds[1].clone() as i32;
       return (first + second) / 2 as i32
    }
    first
}