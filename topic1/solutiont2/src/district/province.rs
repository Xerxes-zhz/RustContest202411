
use std::mem;
#[derive(Debug, Clone)]
pub struct Province {
    root_city: String,
    sub_city: Vec<String>,
}
#[derive(Debug)]
enum State {
    String,
    Start,
    Array,
    Object,
    End,
}
#[derive(Debug)]
enum ContextType {
    Root,
    Sub,
    Batch,
    Null, // 无数据时占位用
}
#[derive(Debug)]
struct Context {
    // context stack
    pub context_type: Vec<ContextType>,
    pub state: Vec<State>,
    // data
    pub root_city: String,
    pub sub_city: Vec<String>,
    pub provinces: Vec<Province>,
    pub batches: Vec<Vec<Province>>,
}

impl Province {
    pub fn prepare(&mut self) {
        self.sub_city.push(self.root_city.clone());
        self.sub_city.sort();
        self.sub_city.dedup();
    }
    pub fn into_dsu_iter(self) -> impl Iterator<Item = (String, String)> {
        let root = self.root_city;
        self.sub_city
            .into_iter()
            .map(move |sub| (root.clone(), sub))
    }
    // {{{ 有限自动状态机解析非标json
    pub fn from_json_using_fsm(json: String) -> Vec<Vec<Province>> {
        let mut chars = json.chars().peekable();
        let mut current = Context {
            state: vec![State::Start],
            root_city: String::new(),
            sub_city: Vec::new(),
            provinces: Vec::new(),
            batches: Vec::new(),
            context_type: vec![ContextType::Null],
        };
        // 基于FSM
        while let Some(&c) = chars.peek() {
            match c {
                ' ' | '\n' | ',' => {
                    chars.next();
                    continue;
                } // 分隔符跳过
                _ => {}
            }
            if let Some(s) = current.state.last() {
                match s {
                    State::Start => current.state.push(State::Object),
                    State::End => {
                        chars.next();
                        if chars.peek() != None {
                            panic!("not empty")
                        }
                        current.state.pop();
                        break;
                    }
                    State::Object => match c {
                        '"' => {
                            current.state.push(State::String);
                        }
                        ':' => {
                            if let Some(ct) = current.context_type.last() {
                                match ct {
                                    ContextType::Batch => {
                                        current.context_type.push(ContextType::Root);
                                        current.state.push(State::Object);
                                    }
                                    ContextType::Root => {
                                        current.context_type.push(ContextType::Sub);
                                        current.state.push(State::Array);
                                    }
                                    _ => {}
                                }
                            };
                            chars.next(); //comsume :
                        }
                        '}' => {
                            if let Some(ct) = current.context_type.last() {
                                match ct {
                                    ContextType::Root => {
                                        current.context_type.pop();
                                        current.batches.push(mem::take(&mut current.provinces));
                                    }
                                    ContextType::Batch => {
                                        current.context_type.pop();
                                    }
                                    ContextType::Null => {
                                        current.context_type.pop();
                                        current.state.push(State::End);
                                    }
                                    _ => {}
                                }
                            }
                            current.state.pop();
                            chars.next();
                        }
                        '{' => {
                            if let Some(ct) = current.context_type.last() {
                                match ct {
                                    ContextType::Null => {
                                        current.context_type.push(ContextType::Batch)
                                    }
                                    ContextType::Batch => {
                                        current.context_type.push(ContextType::Root)
                                    }
                                    ContextType::Root => current.state.push(State::String),
                                    _ => {}
                                }
                            }
                            chars.next();
                        }
                        _ => panic!("object start with key with {}", c),
                    },
                    State::String => {
                        match c {
                            // consume first "
                            '"' => {
                                chars.next();
                            }
                            _ => {}
                        }
                        let mut string = String::new();
                        while let Some(_c) = chars.next_if(|c| *c != '"') {
                            string.push(_c);
                        }

                        if let Some(ct) = current.context_type.last() {
                            match ct {
                                ContextType::Root => {
                                    current.root_city = string;
                                }
                                ContextType::Sub => {
                                    current.sub_city.push(string);
                                }
                                _ => {}
                            }
                        };
                        current.state.pop();
                        chars.next(); //consume last "
                    }
                    State::Array => match c {
                        ']' => {
                            current.provinces.push(Province {
                                root_city: mem::take(&mut current.root_city),
                                sub_city: mem::take(&mut current.sub_city),
                            });
                            current.context_type.pop();
                            current.state.pop();
                            chars.next();
                        }
                        '"' => {
                            current.state.push(State::String);
                        }
                        '[' => {
                            chars.next();
                        }
                        _ => {
                            chars.next();
                        }
                    },
                }
            }
        }
        current.batches
    }
    // }}}
}
