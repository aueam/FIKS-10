use std::collections::{HashMap, HashSet};
use std::env;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use crate::Expression::{Val, Xor};
use crate::Value::{True, False, Var};

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Equation {
    left: Expression,
    right: Expression,
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Expression {
    Val(Value),
    Xor(Vec<Value>),
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Value {
    True,
    False,
    Var(i32),
}

impl Equation {
    fn count_members(&self) -> usize {
        let mut counter = 0;

        match &self.left {
            Val(_) => counter += 1,
            Xor(vals) => counter += vals.len()
        }

        match &self.right {
            Val(_) => counter += 1,
            Xor(vals) => counter += vals.len()
        }

        counter
    }

    fn has_var(&self, var: &i32) -> Result<bool, ()> {
        match &self.left {
            Val(val) => {
                if let Var(v) = val {
                    if var == v { return Ok(true); }
                }
            }
            Xor(vals) => {
                for val in vals {
                    if let Var(v) = val {
                        if var == v { return Ok(true); }
                    }
                }
            }
        }

        match &self.right {
            Val(val) => {
                if let Var(v) = val {
                    if var == v { return Ok(false); }
                }
            }
            Xor(vals) => {
                for val in vals {
                    if let Var(v) = val {
                        if var == v { return Ok(false); }
                    }
                }
            }
        }

        Err(())
    }

    fn evaluate(&self, var: &i32, substitution_for: i32, substitution: Vec<Value>) -> Result<Self, ()> {
        let side = match self.has_var(var) {
            Ok(side) => side,
            Err(_) => return Err(())
        };

        let mut vals: Vec<Value> = Vec::new();
        match &self.left {
            Val(val) => { if !side { vals.push(val.clone()) } }
            Xor(vls) => {
                for val in vls {
                    if val != &Var(*var) { vals.push(val.clone()) }
                }
            }
        }
        match &self.right {
            Val(val) => { if side { vals.push(val.clone()) } }
            Xor(v) => {
                for val in v {
                    if val != &Var(*var) { vals.push(val.clone()) }
                }
            }
        }

        // substitute
        let vals = if substitution_for >= 0 {
            let mut sub = 0;
            let mut new_vals: Vec<Value> = Vec::new();
            for val in &vals {
                if val == &Var(substitution_for) {
                    sub += 1;
                } else {
                    new_vals.push(val.clone())
                }
            }
            let mut vals = new_vals;
            for _ in 0..sub {
                vals.extend(substitution.clone())
            }
            vals
        } else { vals };

        let mut eq = Equation {
            left: Val(Var(*var)),
            right: if vals.len() == 1 {
                Val(vals[0].clone())
            } else {
                Xor(vals)
            },
        };
        match eq.solve_for_left() {
            Ok(_) => Ok(eq),
            Err(_) => Err(())
        }
    }

    /// Returns Err if equation is unsolvable (same variable on both sides)
    fn solve_for_left(&mut self) -> Result<(), ()> {
        let looking_for_val = if let Val(val) = &self.left {
            val
        } else { panic!("Left side of equation has to be Val") };

        let simplified_equation = match &self.right {
            Val(val) => {
                if val == looking_for_val {
                    return Err(());
                }
                Val(val.clone())
            }
            Xor(expression) => {
                let mut found_numbers: Vec<bool> = Vec::new();
                let mut found_vars: HashSet<i32> = HashSet::new();

                for val in expression {
                    if val == looking_for_val { return Err(()); }
                    match val {
                        True => found_numbers.push(true),
                        False => found_numbers.push(false),
                        Var(var) => {
                            if found_vars.contains(var) {
                                found_vars.remove(var);
                                found_numbers.push(false);
                            } else { found_vars.insert(*var); }
                        }
                    }
                }

                let mut result = found_numbers.pop().expect("Expect at least one val in expression");
                if !found_numbers.is_empty() {
                    for num in found_numbers {
                        result ^= num
                    }
                }

                let mut vals: Vec<Value> = Vec::new();

                vals.push(match result {
                    true => True,
                    false => False
                });

                for var in found_vars {
                    vals.push(Var(var))
                }

                if vals.len() == 1 {
                    Val(vals[0].clone())
                } else {
                    Xor(vals)
                }
            }
        };
        self.right = simplified_equation;

        Ok(())
    }

    fn analyse(&self) -> Vec<i32> {
        let mut vars: Vec<i32> = Vec::new();
        match &self.left {
            Val(val) => {
                if let Var(var) = val { vars.push(*var) }
            }
            Xor(vals) => {
                for val in vals {
                    if let Var(var) = val { vars.push(*var) }
                }
            }
        }
        vars
    }

    fn check_equation_validity(&self, variables: &HashMap<i32, bool>) -> bool {
        let mut vars: Vec<i32> = Vec::new();
        let mut must_equal_to: Option<bool> = None;

        // collect variables and "must_equal_to"
        match &self.left {
            Val(val) => {
                match val {
                    True => must_equal_to = Some(true),
                    False => must_equal_to = Some(false),
                    Var(var) => vars.push(*var)
                }
            }
            Xor(expression) => {
                for val in expression {
                    match val {
                        True => must_equal_to = Some(true),
                        False => must_equal_to = Some(false),
                        Var(var) => vars.push(*var)
                    }
                }
            }
        }

        match &self.right {
            Val(val) => {
                match val {
                    True => must_equal_to = Some(true),
                    False => must_equal_to = Some(false),
                    Var(var) => vars.push(*var)
                }
            }
            Xor(expression) => {
                for val in expression {
                    match val {
                        True => must_equal_to = Some(true),
                        False => must_equal_to = Some(false),
                        Var(var) => vars.push(*var)
                    }
                }
            }
        }

        if must_equal_to.is_none() {
            panic!("Something is very wrong")
        }

        // substitute variables
        let mut substituted_vars: Vec<bool> = Vec::new();
        for var in vars {
            match variables.get(&var) {
                None => return true,
                Some(val) => substituted_vars.push(*val)
            }
        }

        let mut result = substituted_vars.pop().expect("Expect at least one val in expression");
        if !substituted_vars.is_empty() {
            for num in substituted_vars {
                result ^= num
            }
        }

        if Some(result) == must_equal_to {
            return true;
        }
        false
    }
}

fn is_in_extracted(extracted_equations: &HashMap<i32, Vec<Equation>>, analysis: &[i32], except: &i32) -> Result<(i32, Expression), ()> {
    for eqs in extracted_equations.values() {
        for eq in eqs {
            if let Val(Var(i)) = eq.left {
                if &i == except {
                    continue
                } else if analysis.contains(&i) {
                    return Ok((i, eq.right.clone()))
                }
            } else { panic!("This shouldn't happen") }
        }
    }
    Err(())
}

fn solve(variables: Vec<i32>, extracted_constants: Vec<Equation>, mut extracted_equations: Vec<Equation>) -> Result<(i32, String), ()> {
    // DEBUG
    // println!();
    // println!("{:?}", variables);
    // println!();
    // println!("extracted_constants:");
    // for eq in &extracted_constants {
    //     println!("{}", eq);
    // }
    // println!();
    // println!("extracted_equations:");
    // for eq in &extracted_equations {
    //     println!("{}", eq);
    // }

    let mut vars: HashMap<i32, bool> = HashMap::new();
    let mut solutions: Vec<Vec<bool>> = Vec::new();

    for const_ in &extracted_constants {
        if let Val(Var(left)) = const_.left {
            match &const_.right {
                Val(val) => {
                    match val {
                        True => {
                            if let Some(v) = vars.get(&left) {
                                if v == &false {
                                    return Err(());
                                }
                            } else {
                                vars.insert(left, true);
                            }
                        },
                        False => {
                            if let Some(v) = vars.get(&left) {
                                if v == &true {
                                    return Err(());
                                }
                            } else {
                                vars.insert(left, false);
                            }
                        },
                        Var(_) => panic!()
                    };
                }
                Xor(_) => panic!()
            }
        } else { panic!("This shouldn't happen") }
    }

    extracted_equations.sort_by_key(|eq| {
        if let Xor(values) = &eq.right {
            values.len()
        } else {
            0
        }
    });

    // DEBUG
    // println!();
    // println!("{:?}", variables);
    // println!();
    // println!("extracted_constants:");
    // for eq in &extracted_constants {
    //     println!("{}", eq);
    // }
    // println!();
    // println!("extracted_equations:");
    // for eq in &extracted_equations {
    //     println!("{}", eq);
    // }

    solve_recursion(vars, &variables, &mut solutions, &extracted_equations);

    let solutions_length = solutions.len() as i32;
    if solutions_length == 0 {
        return Err(());
    }

    Ok((solutions_length, bool_vec_to_string(&solutions[0])))
}

fn solve_recursion (
    variables: HashMap<i32, bool>,
    all_vars: &Vec<i32>,
    solutions: &mut Vec<Vec<bool>>,
    extracted_equations: &Vec<Equation>
) {
    // DEBUG
    // println!();
    // println!();
    // println!("{:?}", all_vars);
    // println!("{}", variables.len());
    // println!("{:?}", variables);
    // println!("{:?}", solutions.len());
    // println!("extracted_equations:");
    // for eq in extracted_equations {
    //     println!("{}", eq);
    // }

    let mut valid = true;
    for eq in extracted_equations {
        if !eq.check_equation_validity(&variables) {
            valid = false;
        }
    }

    if valid && variables.len() == all_vars.len() {
        let mut solution: Vec<bool> = Vec::new();
        for v in all_vars {
            solution.push(*variables.get(v).unwrap())
        }
        solutions.push(solution);
        return;
    }
    if variables.len() == all_vars.len() || !valid {
        return;
    }

    // kvuli tomuhle to asi dlouho trva :((((
    let mut current_var = -1;
    for var in all_vars {
        if variables.contains_key(var) {
            continue
        }
        current_var = *var;
        break;
    }

    if current_var == -1 {
        panic!("This shouldn't happen")
    }

    let mut f = variables.clone();
    let mut t = variables.clone();
    f.insert(current_var, false);
    t.insert(current_var, true);
    solve_recursion(f, all_vars, solutions, extracted_equations); // try new var with 0
    solve_recursion(t, all_vars, solutions, extracted_equations); // try new var with 1
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Zadejte cestu k souboru pro zadani.");
        println!("{:?} {}", args, args.len());
        return;
    }

    let file_name = &args[1];

    let mut file = match File::open(file_name) {
        Ok(file) => {
            file
        }
        Err(e) => {
            println!("Err: {}", e);
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("cannot read file");

    let mut c: Vec<&str> = contents.split('\n').collect();

    let first_line = c.remove(0);

    let f_l: Vec<&str> = first_line.split_whitespace().collect();

    let vars: i32 = match f_l[0].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("failed to parse first line");
            return;
        }
    };

    let variables = (1..=vars).collect::<Vec<i32>>();

    let scripts: i32 = match f_l[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("failed to parse first line");
            return;
        }
    };

    let mut lefts: Vec<Vec<Value>> = Vec::new();

    for a in 1..scripts+1 {
        let mut left: Vec<Value> = Vec::new();

        for (i, line) in c.iter().enumerate() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|x| x.parse().ok())
                .collect();

            if numbers.contains(&a) {
                left.push(Var(i as i32 + 1));
            }
        }

        lefts.push(left);
    }

    lefts.sort_by_key(|inner_vec| inner_vec.len());

    // println!("{:#?}", lefts);

    let mut equations: Vec<Equation> = Vec::new();

    for left in lefts {
        equations.push(Equation {
            left: Xor(left),
            right: Val(True),
        });
    }

    let mut extracted_constants: HashMap<i32, Vec<Equation>> = HashMap::new();
    let mut extracted_equations: HashMap<i32, Vec<Equation>> = HashMap::new();

    if equations.is_empty() {
        println!("0");
        return;
    }

    for (index, eq) in equations.iter().enumerate() {
        // DEBUG
        // println!("{}", eq);

        let mut tmp_extracted_constants: HashMap<i32, Vec<Equation>> = HashMap::new();
        let mut tmp_extracted_equations: HashMap<i32, Vec<Equation>> = HashMap::new();

        let analysis = eq.analyse();
        for var in &analysis {
            let (sub_for, ex) = match is_in_extracted(&extracted_constants, &analysis, var) {
                Ok(ok) => ok,
                Err(_) => {
                    match is_in_extracted(&extracted_equations, &analysis, var) {
                        Ok(ok) => ok,
                        Err(_) => (-0i32, Val(True))
                    }
                }
            };

            let sub = match ex {
                Val(val) => vec![val],
                Xor(vals) => vals
            };

            match eq.evaluate(var, sub_for, sub.clone()) {
                Ok(eq) => {
                    // DEBUG
                    // println!("eq members: {} eq: {}", eq.count_members(), eq);

                    if eq.count_members() == 2 {
                        tmp_extracted_constants.entry(index as i32).or_default().push(eq)
                    } else {
                        tmp_extracted_equations.entry(index as i32).or_default().push(eq)
                    }
                }
                Err(_) => {
                    // DEBUG
                    // println!("cannot evaluate {} with substitution for {} and with substitution {:?}", &var, sub_for, sub)
                }
            }
        }

        merge_maps(&mut extracted_constants, tmp_extracted_constants);
        merge_maps(&mut extracted_equations, tmp_extracted_equations);
    }

    // DEBUG
    // println!("\n");
    // println!("extracted_constants:");
    // for (index, eqs) in &extracted_constants {
    //     println!("{}. script:", index);
    //     for eq in eqs { println!("{}", eq); }
    // }
    // println!();
    // println!("extracted_equations:");
    // for (index, eqs) in &extracted_equations {
    //     println!("{}. script:", index);
    //     for eq in eqs { println!("{}", eq); }
    // }

    match solve(variables, merge_equations(&extracted_constants), merge_equations(&extracted_equations)) {
        Ok((solutions, string)) => println!("{}\n{}", solutions, string),
        Err(_) => println!("0")
    }
}

fn merge_maps(map1: &mut HashMap<i32, Vec<Equation>>, map2: HashMap<i32, Vec<Equation>>) {
    for (key, equations) in map2 {
        map1
            .entry(key)
            .and_modify(|existing_equations| existing_equations.extend(equations.clone()))
            .or_insert(equations);
    }
}

fn merge_equations(map: &HashMap<i32, Vec<Equation>>) -> Vec<Equation> {
    let mut merged_equations: Vec<Equation> = Vec::new();

    for equations in map.values() {
        merged_equations.extend(equations.iter().cloned());
    }

    merged_equations.sort();
    merged_equations.dedup();

    merged_equations
}

fn bool_vec_to_string(bool_vec: &Vec<bool>) -> String {
    let mut result = String::new();

    for &b in bool_vec {
        if b {
            result.push('1');
        } else {
            result.push('0');
        }
    }

    result
}

impl Display for Equation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} = {}", self.left, self.right)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Val(val) => write!(f, "{}", match val {
                True => "1".to_owned(),
                False => "0".to_owned(),
                Var(i) => format!("i_{}", i),
            }),
            Xor(vals) => {
                let mut string = String::new();
                for val in vals {
                    string.push_str(&match val {
                        True => "1 XOR ".to_owned(),
                        False => "0 XOR ".to_owned(),
                        Var(i) => format!("i_{} XOR ", i),
                    })
                }
                write!(f, "{}", string.trim_end_matches(" XOR "))
            }
        }
    }
}