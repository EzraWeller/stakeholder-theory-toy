#![feature(vec_remove_item)]

use std::io;

fn main() {
    let mut market = new_market(1000, 8, 10000, 10);
    market = market.new_established_firm("the founder-owned firm".into());
    market = market.new_established_firm("the shareholder-owned firm".into());
    market = market.new_established_firm("the stakeholder-owned firm".into());
    market.clone().display_firms();
    let mut done = false;
    while !done {
        market = market.play_round();
        market = market.end_round();
    }
}

// ****** THIS ONE ******
// make all three -literally- start the same way by skipping to an identical (successful) snapshot, except that
// their ownership structures are different, and so they make different choices. Selling shares is no longer an option,
// because we assume all the shares have already been sold.
// ****** THIS ONE ******

    // employee-owned
        // 50% raise wages, 50% raise usefulness (indirect way to raise wages) when not in danger
        // when in danger, respond to the danger
            // (raise wages if in danger of losing employees, raise usefulness if in danger of losing customers)
    // shareholder-owned
        // 50% pump share price, 50% raise usefulness (indirect way to raise share price) when not in danger
        // when in danger, respond to the danger
    // stakeholder-owned
        // 100% raise usefulness when not in danger
        // when in danger, respond to the danger

    // 'safe' means that the firm checks its ability to profit and its ability to keep its customers and employees 
    // (they aren't in danger of being taken by other firms)

    // we can do this by adding a function that compares the usefulness, servings, and wage_amount of firm X's last turn to
    // to those of the other firms on the last turn, and checking users_left and workers_left on the last turn:
    // if other firms' usefulness or servings were close to or greater than X's AND there weren't many users left, DANGER
    // if other firms' wage_amount were close to or greater than X's AND there weren't many workers left, DANGER

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
    pub fn new_firm(mut self, name_: String, /* starting_shares: u32,*/ starting_funds: u32) -> Market {
        // TODO only allows new firms with names not already taken
        let new_firm = Firm {
            name: name_,

            // shares_remaining: starting_shares,
            // shares_to_sell: 0,
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

            employee_danger: 0,
            customer_danger: 0,
        };

        self.firms.push(new_firm);
        return self
    }

    pub fn new_established_firm(mut self, name_: String) -> Market {
        // TODO only allows new firms with names not already taken
        let new_firm = Firm {
            name: name_,

            // shares_remaining: shares,
            // shares_to_sell: 0,
            share_market_boost: 0,
            share_price: 0,

            employees: 25,
            number_to_hire: 0,
            wage_amount: 0,

            current_funds: 1000,
            previous_funds: vec![800,650,500,400,320,250,200,150,100],
            profit_trend: 166,

            usefulness: 0,
            servings: 0,

            user_preference_fulfillment: 0,
            employee_preference_fulfillment: 0,

            employee_danger: 0,
            customer_danger: 0,
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
        // market = market.sell_shares();
        market = market.recruit_employees();
        market.clone().display_firms();
        return market
    }

    pub fn display(&self) {
        println!("Displaying market");
        println!("-- total workers available for hire: {}", self.labor_supply);
        println!("-- workers unhired last round: {}", self.workers_left);
        println!("-- minimum wage workers will take: {}", self.min_wage);
        println!("-- total users seeking to buy a product: {}", self.users);
        println!("-- number of users who didn't buy a product last round: {}", self.users_left);
        println!("-- minimum quality of product users will buy: {}", self.min_usefulness_per_user);
    }

    pub fn display_firms(self) {
        for firm in self.firms.into_iter() {
            firm.display();
        }
    }

    // the following five functions are run at the end of every round (in this order)

    pub fn sell_goods(mut self) -> Market { // rename??
        // reads the usefulness created by each firm the previous round and has users buy the most useful products
        // assumes each user has a standard ideal usefulness (e.g. each person wants exactly one car, but the best car)
        // firms that created more usefulness will get more customers
        // since there is a limited number of users, some firms may fail to "sell" all of their usefulness
        // firms receive funds proportional to their sales (not to their usefulness)
        // firms receive user_preference fulfillment proportional to the usefulness dispursed (bought by users)
        self.users_left = self.users;
        let mut c: usize = 0;
        let mut firms: Vec<(usize, Firm)> = self.clone().firms.into_iter().map(|f| { c += 1; (c, f) }).collect();
        firms.sort_by(|a, b| b.1.usefulness.cmp(&a.1.usefulness));
        for firm_data in firms {
            // firm.sell_goods(); ??
            // firm.pay_employees(); ??
            // firm.set_share_prices(); ??
            // firm.sell_shares(); ??
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

    /*
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
    */
    
    pub fn recruit_employees(mut self) -> Market {
        // reads wage_amount and number_to_hire for each firm, 
        // then assigns employees to the firms according to who is paying most 
        // (and paying more than the minimum)
        self.workers_left = self.labor_supply;
        let mut c: usize = 0;
        let mut firms: Vec<(usize, Firm)> = self.clone().firms.into_iter().map(|f| { c += 1; (c, f) }).collect();
        firms.sort_by(|a, b| b.1.wage_amount.cmp(&a.1.wage_amount));
        for firm_data in firms {
            // firm.recruit_employees(); ??
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
            let market_copy = self.clone();
            self.firms[firm_data.0 - 1].customer_danger = *&self.firms[firm_data.0 - 1].check_customer_danger(market_copy.clone());
            self.firms[firm_data.0 - 1].employee_danger = *&self.firms[firm_data.0 - 1].check_employee_danger(market_copy);
        }
        return self
    }
}

#[derive(Clone)]
pub struct Firm {
    name: String,

    // shares_remaining: u32,
    // shares_to_sell: u32,
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

    employee_danger: u32,
    customer_danger: u32,
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
        // let shares_to_sell = inputs[5];

        assert!(servings > 0 as u32, "Servings must be > 0.");
        assert!(0 as u32 <= funds_to_wages_percentage && funds_to_wages_percentage <= 100 as u32, "Funds to wage percentage must be between 0 and 100, inclusive.");
        assert!(&wage_to_usefulness + &wage_to_pumping == 100, "Wage portions must sum to 100.");

        self.wage_amount = &self.current_funds * &funds_to_wages_percentage / 100 / &number_to_hire;
        self.number_to_hire = number_to_hire;
        self.usefulness = &self.employees * wage_to_usefulness / &servings;
        self.servings = servings;
        self.share_market_boost = &self.employees * wage_to_pumping;
        // self.shares_to_sell = shares_to_sell;
        return self
    }

    pub fn play_robo_round(mut self, inputs: [u32; 5] ) -> Firm {

        assert!(inputs[3] > 0 as u32, "Servings must be > 0.");
        assert!(0 as u32 <= inputs[0] && inputs[0] <= 100 as u32, "Funds to wage percentage must be between 0 and 100, inclusive.");
        assert!(&inputs[2] + &inputs[4] == 100, "Wage portions must sum to 100.");

        self.wage_amount = &self.current_funds * &inputs[0] / 100 / &inputs[1];
        self.number_to_hire = inputs[1];
        self.usefulness = &self.employees * inputs[2] / &servings;
        self.servings = inputs[3];
        self.share_market_boost = &self.employees * inputs[4];
        return self
    }

    pub fn decide_strategy(&self, firm_type: String) -> [u32; 5] {
        
    }

    pub fn display(&self) {
        println!("Displaying firm '{}'", &self.name);
        // println!("-- shares remaining: {}", &self.shares_remaining);
        println!("-- share price: {}", &self.share_price);
        println!("-- employees: {}", &self.employees);
        println!("-- current funds: {}", &self.current_funds);
        println!("-- previous funds: {:?}", &self.previous_funds);
        println!("-- profit trend: {}", &self.profit_trend);
        println!("-- user preference-fulfillment: {}", &self.user_preference_fulfillment);
        println!("-- employee preference-fulfillment: {}", &self.user_preference_fulfillment);
        println!("-- employee DANGER: {}", &self.employee_danger);
        println!("-- customer DANGER: {}", &self.customer_danger);
    }

    pub fn check_customer_danger(&mut self, market: Market) -> u32 {
        // if other firms' usefulness or servings were close to or greater than X's AND there weren't many users left, DANGER
        let mut firm_count: u32 = 0;
        let mut sum_of_usefulnesses: u32 = 0;
        let customer_danger_percentage: f32 = (market.users.clone() - market.users_left.clone()) as f32 / 
            market.users.clone() as f32 * 100 as f32;
        for firm in market.firms.into_iter() {
            if firm.name == self.name { continue }
            firm_count += 1;
            sum_of_usefulnesses += firm.usefulness * firm.servings;
        }
        let usefulness_bar: u32 = &sum_of_usefulnesses / &firm_count;
        let usefulness_dif: i32 = (self.usefulness * self.servings) as i32 - usefulness_bar as i32;
        let mut usefulness_danger_percentage: f32 = 100.0;
        if usefulness_dif > 0 {
            usefulness_danger_percentage = (usefulness_dif as f32 / (self.usefulness * self.servings) as f32) * 100 as f32;
        }
        ((usefulness_danger_percentage + customer_danger_percentage * 3.0) / 4.0) as u32
    }

    pub fn check_employee_danger(&mut self, market: Market) -> u32 {
        // if other firms' wage_amount were close to or greater than X's AND there weren't many workers left, DANGER
        let mut firm_count: u32 = 0;
        let mut sum_of_wage_amounts: u32 = 0;
        let employee_danger_percentage: f32 = (market.labor_supply.clone() - market.workers_left.clone()) as f32 / 
            market.labor_supply.clone() as f32 * 100 as f32;
        for firm in market.firms.into_iter() {
            if firm.name == self.name { continue }
            firm_count += 1;
            sum_of_wage_amounts += firm.wage_amount;
        }
        let wage_amount_bar: u32 = &sum_of_wage_amounts / &firm_count;
        let wage_amount_dif: i32 = self.wage_amount as i32 - wage_amount_bar as i32;
        let mut wage_amount_danger_percentage: f32 = 100.0;
        if wage_amount_dif > 0 {
            wage_amount_danger_percentage = wage_amount_dif as f32 / self.wage_amount as f32 * 100 as f32;
        }
        ((wage_amount_danger_percentage + employee_danger_percentage * 3.0) / 4.0) as u32
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
    // inputs.push(get_input("Enter number of shares to sell:").parse().unwrap());
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