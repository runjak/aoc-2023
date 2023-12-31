use std::{
    collections::{HashMap, HashSet, VecDeque},
    error::Error,
    fs,
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum SignalType {
    Low,
    High,
}

impl SignalType {
    /// Returns `true` if the signal type is [`High`].
    ///
    /// [`High`]: SignalType::High
    #[must_use]
    fn is_high(&self) -> bool {
        matches!(self, Self::High)
    }
}

#[derive(Debug)]
struct Signal {
    from: String,
    signal_type: SignalType,
    to: String,
}

impl Signal {
    // No must_use, bc it's used in tests
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        let signal_label = if self.signal_type.is_high() {
            "-high->".to_string()
        } else {
            "-low->".to_string()
        };

        let parts = [self.from.to_string(), signal_label, self.to.to_string()];

        parts.join(" ")
    }
}

#[derive(Debug, PartialEq)]
enum Module {
    Broadcaster {
        name: String,
        outputs: Vec<String>,
    },
    FlipFlop {
        name: String,
        is_on: Vec<bool>,
        outputs: Vec<String>,
    },
    Conjunction {
        name: String,
        inputs: HashMap<String, SignalType>,
        outputs: Vec<String>,
    },
}

impl Module {
    /// Returns `true` if the module is [`Conjunction`].
    ///
    /// [`Conjunction`]: Module::Conjunction
    #[must_use]
    fn is_conjunction(&self) -> bool {
        matches!(self, Self::Conjunction { .. })
    }

    #[must_use]
    fn get_name(&self) -> &String {
        match self {
            Module::Broadcaster { name, outputs: _ } => name,
            Module::FlipFlop {
                name,
                is_on: _,
                outputs: _,
            } => name,
            Module::Conjunction {
                name,
                inputs: _,
                outputs: _,
            } => name,
        }
    }

    #[must_use]
    fn get_outputs(&self) -> &Vec<String> {
        match self {
            Module::Broadcaster { name: _, outputs } => outputs,
            Module::FlipFlop {
                name: _,
                is_on: _,
                outputs,
            } => outputs,
            Module::Conjunction {
                name: _,
                inputs: _,
                outputs,
            } => outputs,
        }
    }
}

type ModuleCatalog = HashMap<String, Module>;

fn parse_input(input: String) -> ModuleCatalog {
    let modules = input.lines().filter_map(|line| -> Option<Module> {
        let (name, outputs) = line.split_once(" -> ")?;
        let outputs = outputs
            .split(", ")
            .map(|output| output.to_string())
            .collect::<Vec<_>>();

        if name == "broadcaster" {
            return Some(Module::Broadcaster {
                name: name.to_string(),
                outputs,
            });
        }

        let prefix = &name[0..1];
        let name = name[1..].to_string();

        if prefix == "%" {
            return Some(Module::FlipFlop {
                name,
                is_on: Vec::from([false]),
                outputs,
            });
        }

        if prefix == "&" {
            return Some(Module::Conjunction {
                name,
                inputs: HashMap::new(),
                outputs,
            });
        }

        None
    });

    let mut catalog: ModuleCatalog = modules
        .map(|module| (module.get_name().to_string(), module))
        .collect();

    let conjunction_names = catalog
        .values()
        .filter_map(|module| -> Option<String> {
            if module.is_conjunction() {
                return Some(module.get_name().to_string());
            }

            None
        })
        .collect::<HashSet<_>>();

    let mut inputs_by_name: HashMap<String, Vec<String>> = HashMap::new();

    for module in catalog.values() {
        let targets = module
            .get_outputs()
            .iter()
            .filter(|output| conjunction_names.contains(*output));

        for target in targets {
            match inputs_by_name.get_mut(target) {
                Some(existing) => {
                    existing.push(module.get_name().to_string());
                }
                None => {
                    inputs_by_name.insert(
                        target.to_string(),
                        Vec::from([module.get_name().to_string()]),
                    );
                }
            }
        }
    }

    for conjunction_name in conjunction_names.iter() {
        let inputs = inputs_by_name.get(conjunction_name);
        let conjunction = catalog.get_mut(conjunction_name);
        if let (Some(conjunction), Some(inputs_to_add)) = (conjunction, inputs) {
            match conjunction {
                Module::Conjunction {
                    name: _,
                    inputs,
                    outputs: _,
                } => {
                    for input in inputs_to_add.iter() {
                        inputs.insert(input.to_string(), SignalType::Low);
                    }
                }
                _ => {}
            }
        }
    }

    catalog
}

fn signal_at_module(signal: &Signal, module: &mut Module) -> Vec<Signal> {
    if &signal.to != module.get_name() {
        return Vec::new();
    }

    match module {
        Module::Broadcaster { name, outputs } => {
            return outputs
                .iter()
                .map(|output| -> Signal {
                    Signal {
                        from: name.to_string(),
                        signal_type: signal.signal_type,
                        to: output.to_string(),
                    }
                })
                .collect();
        }
        Module::FlipFlop {
            name,
            is_on,
            outputs,
        } => {
            if signal.signal_type.is_high() {
                return Vec::new();
            }

            let next_is_on = !is_on.first().unwrap_or(&false);
            is_on[0] = next_is_on;

            let signal_type = if next_is_on {
                SignalType::High
            } else {
                SignalType::Low
            };

            return outputs
                .iter()
                .map(|output| -> Signal {
                    Signal {
                        from: name.to_string(),
                        signal_type,
                        to: output.to_string(),
                    }
                })
                .collect();
        }
        Module::Conjunction {
            name,
            ref mut inputs,
            outputs,
        } => {
            inputs.insert(signal.from.to_string(), signal.signal_type);

            let all_high = inputs.values().all(|input| input.is_high());
            let signal_type = if all_high {
                SignalType::Low
            } else {
                SignalType::High
            };

            return outputs
                .iter()
                .map(|output| -> Signal {
                    Signal {
                        from: name.to_string(),
                        signal_type,
                        to: output.to_string(),
                    }
                })
                .collect();
        }
    }
}

fn trigger_button(modules: &mut ModuleCatalog) -> Vec<Signal> {
    let mut seen_signals: Vec<Signal> = Vec::new();
    let mut signals = VecDeque::from([Signal {
        from: "button".to_string(),
        signal_type: SignalType::Low,
        to: "broadcaster".to_string(),
    }]);

    while let Some(signal) = signals.pop_front() {
        if let Some(module) = modules.get_mut(&signal.to) {
            for next_signal in signal_at_module(&signal, module).into_iter() {
                signals.push_back(next_signal);
            }
        }

        seen_signals.push(signal);
    }

    seen_signals
}

fn trigger_1000_times_and_score(modules: &mut ModuleCatalog) -> u64 {
    let mut high_count: u64 = 0;
    let mut low_count: u64 = 0;

    for _ in 0..1_000 {
        let seen_signals = trigger_button(modules);

        for signal in seen_signals {
            if signal.signal_type.is_high() {
                high_count += 1;
            } else {
                low_count += 1;
            }
        }
    }

    high_count * low_count
}

fn first() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./inputs/20/example-1.txt",
        "./inputs/20/example-2.txt",
        "./inputs/20/input.txt",
    ];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let mut input = parse_input(input);

        let score = trigger_1000_times_and_score(&mut input);

        println!("Score for {}: {}", path, score);
    }

    Ok(())
}

fn count_presses_for_rx_low(input: &mut ModuleCatalog) -> u64 {
    let mut count = 0;

    let mut running = true;

    while running {
        count += 1;

        for signal in trigger_button(input).iter() {
            if signal.to == "rx" && !signal.signal_type.is_high() {
                running = false;
            }
        }
    }

    count
}

fn second() -> Result<(), Box<dyn Error>> {
    let paths = ["./inputs/20/input.txt"];

    for path in paths {
        let input = fs::read_to_string(path)?;
        let mut input = parse_input(input);

        let score = count_presses_for_rx_low(&mut input);

        println!("Button presses for {}: {}", path, score);
    }

    Ok(())
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("20-1:");
    first()?;
    println!("20-2:");
    second()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{error::Error, fs};

    use super::{parse_input, trigger_button};

    #[test]
    fn example_1_should_behave_as_described() -> Result<(), Box<dyn Error>> {
        let example = fs::read_to_string("./inputs/20/example-1.txt")?;
        let mut example = parse_input(example);

        let expected = fs::read_to_string("./inputs/20/expected-1.txt")?;

        let actual = trigger_button(&mut example);
        let actual = actual
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn example_2_should_behave_as_described() -> Result<(), Box<dyn Error>> {
        let example = fs::read_to_string("./inputs/20/example-2.txt")?;
        let mut example = parse_input(example);

        let expected = fs::read_to_string("inputs/20/expected-2-1.txt")?;

        let actual = trigger_button(&mut example);
        let actual = actual
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(actual, expected, "Comparing example-2 with expected-2-1");

        let expected = fs::read_to_string("inputs/20/expected-2-2.txt")?;

        let actual = trigger_button(&mut example);
        let actual = actual
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(actual, expected, "Comparing example-2 with expected-2-2");

        let expected = fs::read_to_string("inputs/20/expected-2-3.txt")?;

        let actual = trigger_button(&mut example);
        let actual = actual
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        assert_eq!(actual, expected, "Comparing example-2 with expected-2-3");

        Ok(())
    }
}
