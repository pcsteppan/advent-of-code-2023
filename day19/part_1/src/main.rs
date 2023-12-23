use rayon::prelude::*;
use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let system = System::from_str(&input);
    let accepted_items_sum = system.solve();
    println!("part 1: {}", accepted_items_sum);

    let accepted_items_count = system.find_all_accepted_items_count();
    println!("part 2: {}", accepted_items_count);
}

#[derive(Debug, Clone)]
struct Item([u16; 4]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum RuleType {
    LT,
    GT,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule {
    rule_type: RuleType,
    attribute_index: usize,
    value: u16,
    destination: String,
}

#[derive(Debug, Clone)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default_destination: String,
}

#[derive(Debug, Clone)]
struct System {
    workflows: HashMap<String, Workflow>,
    items: Vec<Item>,
}

impl System {
    fn from_str(input: &str) -> Self {
        let (workflows, items) = input.split_once("\r\n\r\n").unwrap();

        let mut workflows: Vec<_> = workflows
            .lines()
            .map(|line| {
                // px{a<2006:qkq,m>2090:A,rfg}
                let name = line.split('{').next().unwrap().to_string();
                let rules_and_default_destination: Vec<_> = line
                    .split('{')
                    .nth(1)
                    .unwrap()
                    .split('}')
                    .next()
                    .unwrap()
                    .split(',')
                    .collect();

                let rules = rules_and_default_destination
                    [..rules_and_default_destination.len() - 1]
                    .iter()
                    .map(|condition| {
                        let mut condition = condition.split(':');
                        let rule = condition.next().unwrap();
                        println!("{}", rule);
                        let attribute_index = match rule.chars().next().unwrap() {
                            'x' => 0,
                            'm' => 1,
                            'a' => 2,
                            's' => 3,
                            _ => panic!("Invalid attribute"),
                        };
                        let rule_type = match rule.chars().nth(1).unwrap() {
                            '<' => RuleType::LT,
                            '>' => RuleType::GT,
                            _ => panic!("Invalid rule type"),
                        };
                        let value = rule[2..].parse().unwrap();
                        let destination = condition.next().unwrap().parse().unwrap();
                        Rule {
                            rule_type,
                            attribute_index,
                            value,
                            destination,
                        }
                    })
                    .collect::<Vec<_>>();
                let default_destination = rules_and_default_destination
                    .last()
                    .unwrap()
                    .parse()
                    .unwrap();

                Workflow {
                    name,
                    rules,
                    default_destination,
                }
            })
            .collect();
        let mut workflow_map = HashMap::new();

        for workflow in workflows.clone() {
            workflow_map.insert(workflow.name.clone(), workflow);
        }

        let items = items
            .lines()
            .map(|line| {
                let item_str = line[1..line.len() - 1].to_string();
                let item_attributes: Vec<u16> = item_str
                    .split(',')
                    .map(|attribute| {
                        let mut attribute = attribute.split('=');
                        attribute.next();
                        let value = attribute.next().unwrap().parse().unwrap();
                        value
                    })
                    .collect();

                Item([
                    item_attributes[0],
                    item_attributes[1],
                    item_attributes[2],
                    item_attributes[3],
                ])
            })
            .collect::<Vec<_>>();

        Self {
            workflows: workflow_map,
            items,
        }
    }

    fn find_all_accepted_items_count(&self) -> usize {
        (1..4001)
            .into_par_iter()
            .map(|x| {
                println!("{}", x);
                let mut sub_count = 0;
                for m in 1..=4000 {
                    println!("{}", m);
                    for a in 1..=4000 {
                        for s in 1..=4000 {
                            let item = Item([x, m, a, s]);
                            if self.is_item_accepted(&item) {
                                sub_count += 1;
                            }
                        }
                    }
                }
                sub_count
            })
            .sum::<usize>()
    }

    fn is_item_accepted(&self, item: &Item) -> bool {
        let mut curr_workflow_name = "in";

        while curr_workflow_name != "A" && curr_workflow_name != "R" {
            let workflow = self.workflows.get(curr_workflow_name).unwrap();

            let mut use_default = true;
            for rule in workflow.rules.iter() {
                let attribute_value = item.0[rule.attribute_index];
                match rule.rule_type {
                    RuleType::LT => {
                        if attribute_value < rule.value {
                            curr_workflow_name = &rule.destination;
                            use_default = false;
                            break;
                        }
                    }
                    RuleType::GT => {
                        if attribute_value > rule.value {
                            curr_workflow_name = &rule.destination;
                            use_default = false;
                            break;
                        }
                    }
                }
            }

            if use_default {
                curr_workflow_name = &workflow.default_destination;
            }
        }

        curr_workflow_name == "A"
    }

    fn find_accepted_items(&self) -> Vec<&Item> {
        self.items
            .iter()
            .filter(|item| self.is_item_accepted(item))
            .collect()
    }

    fn solve(&self) -> usize {
        self.find_accepted_items()
            .iter()
            .map(|item| item.0.iter().sum::<u16>() as usize)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

        let system = System::from_str(input);
        //dbg!(system.clone());

        let accepted_items = system.find_accepted_items();
        assert_eq!(3, accepted_items.len());
        //dbg!(accepted_items);

        let accepted_items_sum = system.solve();
        assert_eq!(19114, accepted_items_sum);
    }
}
