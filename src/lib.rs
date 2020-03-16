#![feature(non_ascii_idents)]
//#[allow(unused_imports)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

//include!("./dicts.rs");
use crate::dicts::{Ё, СЛОВАРЬ};
use crate::настройки::{Решение, Выбор};
use rand::seq::SliceRandom;
use std::error::Error as Ошибка;
//use crate::настройки::{Настройки, SettingsBuilder};
//
mod dicts;
//mod настройки;

mod настройки;

use crate::настройки::Настройки;

type Источник = str;
type Исправлено = String;

/// Найти и исправить ошибки в тексте
pub fn исправить_текст(источник: &Источник, настройки: &Настройки) -> Исправлено {
    let буквы = источник.chars();
    let mut исправлено: Исправлено = String::with_capacity(источник.len());
    let mut последнее_исправление = 0;

    let mut начало = None;
    let mut исправить = false;
    let mut смещение = 0;
    for (_, буква) in буквы.enumerate() {
        match буква {
            'а'..='я' | 'А'..='Я' | 'ё' | 'Ё' => {
                if начало.is_none() {
                    // Начало слова
                    начало = Some(смещение);
                    исправить = false;
                }
                // Проверка, требует ли слово проверки
                if !исправить {
                    if настройки.испр_ё && (буква == 'ё' || буква == 'Ё') {
                        исправить = true;
                    } else if настройки.испр_е && (буква == 'е' || буква == 'Е') {
                        исправить = true;
                    }
                }
            }
            _ => {
                if исправить {
                    // Был конец слова
                    if let Some(нач) = начало {
                        let исправления = найти_исправления(&источник, нач, смещение, &настройки);
                        исправить_срез(&mut исправлено, &mut последнее_исправление, &источник, нач, смещение, исправления, &настройки);
                    }
                }
                начало = None;
            }
        }
        смещение += буква.len_utf8();
    }
    if исправить {
        // В самом конце строки тоже есть слово для проверки
        if let Some(нач) = начало {
            let исправления = найти_исправления(&источник, нач, смещение, &настройки);
            исправить_срез(&mut исправлено, &mut последнее_исправление, &источник, нач, смещение, исправления, &настройки);
        }
    }

    исправлено.to_string()
}

/// Список смещений букв для точного исправления.
/// None означает, что исправления не нужны.
type Исправления = Option<Vec<usize>>;

/// По найденному слову и согласно настройкам понять, нужно ли что-то исправлять
fn найти_исправления(источник: &str, начало: usize, конец: usize, настройки: &Настройки) -> Исправления {
    let слово = &источник[ начало..конец ].to_string();
    let канон = слово.to_lowercase().to_string();
    println!("Канон: {:?}", канон);
    let найдено = СЛОВАРЬ.get(&канон[..]);
    println!("Найден канон: {:?}", найдено);
    match найдено {
        Some(Ё::МожетЁ(варианты)) => выбрать_нужно_ли_исправлять_на_ё(&варианты, &настройки),
        Some(Ё::ТочноЁ(варианты)) => выбрать_исправление_на_ё(&варианты, &настройки),
        _ => None,
    }
    // //println!("FIX WORD '{}': FOUND '{:?}'", слово, found);
    // match found {
    //     Some(Ё::МожетЁ(v)) => fix_word_maybe_yo(слово, v, &настройки),
    //     Some(Ё::ТочноЁ(v)) => fix_word_def_yo(слово, v, &настройки),
    //     Some(Ё::СловарьЁ) => слово.to_string(),
    //     None => слово.to_string(),
    // }
    //слово.to_string()

    //Vec::new()
}

/// Определить, может быть можно заменять на Ё
fn выбрать_нужно_ли_исправлять_на_ё(варианты: &[Vec<usize>], настройки: &Настройки) -> Исправления {
    match настройки.решение {
        Решение::ОставитьЕ => None,
        Решение::ЗаменитьЕНаЁ => выбрать_исправление_на_ё(&варианты, настройки),
        Решение::ВыбратьСлучайно => panic!("Ещё не реализовано"),
        Решение::Догадаться => panic!("Ещё не реализовано"),
    }
}

/// На Ё точно нужно заменять, но какой вариант выбрать согласно настройкам?
fn выбрать_исправление_на_ё(варианты: &[Vec<usize>], настройки: &Настройки) -> Исправления {
    // Для единственного варианта (что будет почти всегда) брать единственное значение
    let кол_во = варианты.len();
    if кол_во == 1 {
        return Some(варианты[0].to_vec());
    }

    // Выбор согласно настройкам
    let испр = match настройки.выбор {
        Выбор::Первый => варианты[0].to_vec(),
        Выбор::Последний => варианты[кол_во - 1].to_vec(),
        Выбор::Случайный => варианты.choose(&mut rand::thread_rng())?.to_vec(),
        Выбор::Догадаться => panic!("Ещё не реализовано"),
    };

    Some(испр)
}

/// Исправление буквы в указанном срезе
fn исправить_срез(mut исправлено: &str, mut последнее_исправление: &mut usize, источник: &str, начало_среза: usize, длина_среза: usize, исправления: Исправления, настройки: &Настройки) {
    // Копирование из источников до начала слова
    let добавок = &источник[*последнее_исправление .. начало_среза];
    println!("Добавок: {}", &добавок);
    // исправлено.append_str();

    // // Проход по каждому исправлению в слове
    // for исправление in &исправления {
    //     // Определение смещения в источнике от последнего исправления
    //     let смещ = начало_среза + исправление;

    //     // Копирование строки от последнего исправления до найденного смещения
    // }
}

// /// Get value from СЛОВАРЬ
// pub fn find_yo(src: &str) -> Option<Yo> {
//     let key = src.to_lowercase();
//     СЛОВАРЬ.get(&key[..]).cloned()
// }
// 
// /// Get positions of possible 'ё' letters
// pub fn get_yo_pos(src: &str) -> Option<Vec<Vec<usize>>> {
//     let yo = find_yo(&src);
//     if yo.is_none() {
//         return None;
//     }
//     match yo.unwrap() {
//         Yo::YoDict => None,
//         _ => None,
//     }
// }
// 
// /// Исправление буквы е на ё
// fn replace_ye(word: &mut Vec<char>, pos: usize, настройки: &Настройки) {
//     let ye = word.nth(pos);
//     if let Some(ch) = ye {
//         if ch == 'е' {
//             println!("replace word {:?} at pos {:?}", &word, &pos);
//             word.replace(pos, &настройки.replace_yo);
//         } else if ch == 'Е' {
//             word.replace(pos, &настройки.replace_YO);
//         }
//     }
// }
// 
// /// Исправить в слове возможно е на ё
// fn fix_word_maybe_yo(word: &str, variants: &Vec<Vec<usize>>, настройки: &Настройки) -> String {
//     let fixes: Vec<String> = variants.iter().map(|variant| {
//         let mut fixed: Vec<char> = word.chars().collect();
//         for idx in variant.iter().rev() {
//             replace_ye(&mut fixed, *idx, &настройки);
//         }
//         fixed
//     }).collect::<Chars>().as_str();
//     fixes[0].clone()
// }
// 
// /// Исправить в слове определённо е на ё
// fn fix_word_def_yo(word: &str, variants: &Vec<Vec<usize>>, настройки: &Настройки) -> String {
//     let fixes: Vec<String> = variants.iter().map(|variant| {
//         let mut fixed: Vec<char> = word.chars().collect();
//         for idx in variant.iter().rev() {
//             replace_ye(&mut fixed, *idx, &настройки);
//         }
//         fixed
//     }).collect::<Chars>().as_str();
//     fixes[0].clone()
// }
// 
// /// Исправить слово
// pub fn исправить_срез(src: &str, beg: usize, end: usize, настройки: &Настройки) -> String {
//     //println!("Fix word in {} from {} to {}", &src, &beg, &end);
//     let word = &src[ beg..end ].to_string();
//     let lower = word.to_lowercase().to_string();
//     let found = СЛОВАРЬ.get(&lower[..]); //println!("FIX WORD '{}'", word);
//     //println!("FIX WORD '{}': FOUND '{:?}'", word, found);
//     match found {
//         Some(Yo::MbYo(v)) => fix_word_maybe_yo(word, v, &настройки),
//         Some(Yo::DefYo(v)) => fix_word_def_yo(word, v, &настройки),
//         Some(Yo::YoDict) => word.to_string(),
//         None => word.to_string(),
//     }
//     //word.to_string()
// }
// 
// /// Найти и исправить ошибки в тексте
// pub fn исправить_текст(src: &str, настройки: &Настройки) -> String {
//     let mut начало = None;
//     let mut исправить = false;
//     let mut dst = src.to_string();
//     //for (pos, c) in src.char_indices() {
//     let mut смещение = 0;
//     for (pos, c) in src.chars().enumerate() {
// //println!("pos {} char {} byteo {}", &pos, &c, &смещение);
//         match c {
//             'а'..='я' | 'А'..='Я' | 'ё' | 'Ё' => {
//                 if начало.is_none() {
//                     // Начало слова
//                     начало = Some(смещение);
//                     исправить = false;
//                 }
//                 // Есть ли что исправлять
//                 if !исправить {
//                     if настройки.испр_ё && (c == 'ё' || c == 'Ё') {
//                         исправить = true;
//                     } else if настройки.испр_е && (c == 'е' || c == 'Е') {
//                         исправить = true;
//                     }
//                 }
//             }
//             _ => {
//                 if исправить {
//                     // Был конец слова
//                     if let Some(s) = начало {
//                         //let word = &src[ s..pos ];
//                         let fixed = исправить_срез(&src, s, смещение, &настройки);
//                         //println!("Fixed word 1 '{:?}'", &fixed);
//                         dst.replace_range(s..смещение, &fixed);
//                     }
//                 }
//                 начало = None;
//             }
//         }
//         //pos += 1;
//         смещение += c.len_utf8();
//     }
//     if исправить {
//         // В самом конце строки тоже есть слово для проверки
//         if let Some(s) = начало {
//             //let word = &src[ s.. ];
//             let fixed = исправить_срез(&src, s, смещение, &настройки);
//             //println!("Fixed word 2 '{:?}'", &fixed);
//             //println!("Fix2 word '{:?}'", word);
//             dst.replace_range(s..смещение, &fixed);
//         }
//     }
// 
//     println!("Dst '{:?}'", &dst);
//     dst.to_string()
// }
// 
// 
#[cfg(test)]
mod tests {
    use super::*;

//     #[test]
//     fn find_yo_01() {
//         assert_eq!(Some(Yo::MbYo(vec![vec![4]])), find_yo("акушер"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![6]])), find_yo("воплощенность"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![6]])), find_yo("заостренность"));
//         assert_eq!(Some(Yo::YoDict), find_yo("моё"));
//         assert_eq!(Some(Yo::YoDict), find_yo("одёргивать"));
//         //assert_eq!(Some(Yo::YoDict), find_yo("побуждённый"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![5]])), find_yo("притрешь"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![8]])), find_yo("рыбоподъемник"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![7], vec![5]])), find_yo("трёхведерный"));
//         assert_eq!(Some(Yo::YoDict), find_yo("ёршик"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![8], vec![]])), find_yo("четырёхвесельный"));
//         //assert_eq!(Some(Yo::DefYo(vec![vec![5, 8], vec![5, 10]])), find_yo("четырехведерный"));
//     }
// 
//     #[test]
//     fn no_yo_01() {
//         let настройки = SettingsBuilder::new().build();
//         assert_eq!(None, find_yo("юань"));
//         assert_eq!(None, find_yo("дубивший"));
//         assert_eq!(None, find_yo("коллективизируются"));
//         assert_eq!(None, find_yo("настораживаться"));
//         assert_eq!(None, find_yo("настройка"));
//         assert_eq!(None, find_yo("покуривать"));
//         assert_eq!(None, find_yo("проэкзаменованный"));
//         assert_eq!(None, find_yo("смута"));
//         assert_eq!(None, find_yo("утирать"));
//         assert_eq!(None, find_yo("загорбок"));
//         assert_eq!(None, find_yo("заграждать"));
//         assert_eq!(None, find_yo("ЧПУ"));
//     }

    #[test]
    fn fix_text_01() {
        //let настройки = SettingsBuilder::new().build();
        let настройки = Настройки::default();
        let оригинал = "Превед, ежик, что ты хочешь узнать?";
        let исправлено = исправить_текст(оригинал, &настройки);
        let ожидается = "Превед, ёжик, что ты хочешь узнать?".to_string();
        assert_eq!(исправлено, ожидается);
    }

//     #[test]
//     fn fix_her_01() {
//         let настройки = SettingsBuilder::new().build();
//         assert_eq!(исправить_текст("ее её ёе ёё Ее Её Ёе Ёё еЕ еЁ ёЕ ёЁ ЕЕ ЕЁ ЁЕ ЁЁ", &настройки), "её её её её Её Её Её Её еЁ еЁ еЁ еЁ ЕЁ ЕЁ ЕЁ ЕЁ".to_string())
//     }
// }

// /// Get positions of possible 'ё' letters
// pub fn get_yo_pos(src: &str) -> Option<Vec<Vec<usize>>> {
//     let key = src.to_lowercase();
//     СЛОВАРЬ.get(&key[..]).cloned()
// }
//
// /// Get positions variants for fixing of 'ё' letters
// pub fn get_yo_fix_variants(src: &str) -> Option<Vec<String>> {
//     let vars = get_yo_pos(&src);
//     if vars.is_none() {
//         return None;
//     }
//     let vars = vars.unwrap();
//     let mut res: Vec<String> = Vec::new();
//     for var in vars.iter() {
//         let mut variant = src.clone();
//         for c in var {
//             let c_src = src[c];
//             if c_src == 'е' {
//                 variant[c] = 'ё';
//             } else {
//                 variant[c] = 'Ё';
//             }
//         }
//         res.push(variant);
//     }
//
//     Some(res)
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn pos_empty() {
//         assert_eq!(None, get_yo_pos(""));
//     }
//
//     #[test]
//     fn pos_single_char() {
//         assert_eq!(None, get_yo_pos("Е"));
//         assert_eq!(None, get_yo_pos("е"));
//         assert_eq!(None, get_yo_pos("ё"));
//         assert_eq!(None, get_yo_pos("Ё"));
//     }
//
//     #[test]
//     fn pos_no_yo() {
//         assert_eq!(None, get_yo_pos("Вирджиния"));
//         assert_eq!(None, get_yo_pos("покуривать"));
//         assert_eq!(None, get_yo_pos("ТАРАНЯЩИЙ"));
//     }
//
//     #[test]
//     fn pos_yo() {
//         assert_eq!(get_yo_pos("ёкающий"), None);
//         assert_eq!(get_yo_pos("екающий"), Some(vec![vec![0]]));
//         assert_eq!(get_yo_pos("нитрошёлк"), None);
//         assert_eq!(get_yo_pos("нитрошелк"), Some(vec![vec![6]]));
//         assert_eq!(get_yo_pos("рублёвка"), None);
//         assert_eq!(get_yo_pos("рублевка"), Some(vec![vec![4]]));
//         assert_eq!(get_yo_pos("юнкерьё"), None);
//         assert_eq!(get_yo_pos("юнкерье"), Some(vec![vec![6]]));
//         assert_eq!(get_yo_pos("ЁКАЮЩИЙ"), None);
//         assert_eq!(get_yo_pos("ЕКАЮЩИЙ"), Some(vec![vec![0]]));
//         assert_eq!(get_yo_pos("НИТРОШЁЛК"), None);
//         assert_eq!(get_yo_pos("НИТРОШЕЛК"), Some(vec![vec![6]]));
//         assert_eq!(get_yo_pos("РУБЛЁВКА"), None);
//         assert_eq!(get_yo_pos("РУБЛЕВКА"), Some(vec![vec![4]]));
//         assert_eq!(get_yo_pos("ЮНКЕРЬЁ"), None);
//         assert_eq!(get_yo_pos("ЮНКЕРЬЕ"), Some(vec![vec![6]]));
//     }
//
//     #[test]
//     fn pos_2yo() {
//         assert_eq!(get_yo_pos("трехведерный"), Some(vec![vec![2, 5], vec![2, 7]]));
//         assert_eq!(get_yo_pos("ТРЕХВЕДЕРНЫЙ"), Some(vec![vec![2, 5], vec![2, 7]]));
//         assert_eq!(get_yo_pos("трехведёрный"), Some(vec![vec![2]]));
//         assert_eq!(get_yo_pos("ТРЕХВЕДЁРНЫЙ"), Some(vec![vec![2]]));
//         assert_eq!(get_yo_pos("трехвёдерный"), Some(vec![vec![2]]));
//         assert_eq!(get_yo_pos("ТРЕХВЁДЕРНЫЙ"), Some(vec![vec![2]]));
//         assert_eq!(get_yo_pos("трёхведерный"), Some(vec![vec![5], vec![7]]));
//         assert_eq!(get_yo_pos("ТРЁХВЕДЕРНЫЙ"), Some(vec![vec![5], vec![7]]));
//         assert_eq!(get_yo_pos("трёхвёдерный"), None);
//         assert_eq!(get_yo_pos("ТРЁХВЁДЕРНЫЙ"), None);
//         assert_eq!(get_yo_pos("трёхведёрный"), None);
//         assert_eq!(get_yo_pos("ТРЁХВЕДЁРНЫЙ"), None);
//     }
//
//     #[test]
//     fn pos_maybe() {
//         assert_eq!(None, get_yo_pos("маркер"));
//         assert_eq!(None, get_yo_pos("маркёр"));
//     }
//
// //    #[test]
// //    fn vars_empty() {
// //        assert_eq!(None, get_yo_fix_variants(""));
// //    }
// //
// //    #[test]
// //    fn vars_single_char() {
// //        assert_eq!(None, get_yo_fix_variants("Е"));
// //        assert_eq!(None, get_yo_fix_variants("е"));
// //        assert_eq!(None, get_yo_fix_variants("ё"));
// //        assert_eq!(None, get_yo_fix_variants("Ё"));
// //    }
// //
// //    #[test]
// //    fn vars_no_yo() {
// //        assert_eq!(None, get_yo_fix_variants("Вирджиния"));
// //        assert_eq!(None, get_yo_fix_variants("покуривать"));
// //        assert_eq!(None, get_yo_fix_variants("ТАРАНЯЩИЙ"));
// //    }
// //
// //    #[test]
// //    fn vars_yo() {
// //        assert_eq!(get_yo_fix_variants("ёкающий"), None);
// //        assert_eq!(get_yo_fix_variants("екающий"), Some(vec!["ёкающий"]));
// //        assert_eq!(get_yo_fix_variants("нитрошёлк"), None);
// //        assert_eq!(get_yo_fix_variants("нитрошелк"), Some(vec!["нитрошёлк]));
// //        assert_eq!(get_yo_fix_variants("рублёвка"), None);
// //        assert_eq!(get_yo_fix_variants("рублевка"), Some(vec![vec![4]]));
// //        assert_eq!(get_yo_fix_variants("юнкерьё"), None);
// //        assert_eq!(get_yo_fix_variants("юнкерье"), Some(vec![vec![6]]));
// //        assert_eq!(get_yo_fix_variants("ЁКАЮЩИЙ"), None);
// //        assert_eq!(get_yo_fix_variants("ЕКАЮЩИЙ"), Some(vec![vec![0]]));
// //        assert_eq!(get_yo_fix_variants("НИТРОШЁЛК"), None);
// //        assert_eq!(get_yo_fix_variants("НИТРОШЕЛК"), Some(vec![vec![6]]));
// //        assert_eq!(get_yo_fix_variants("РУБЛЁВКА"), None);
// //        assert_eq!(get_yo_fix_variants("РУБЛЕВКА"), Some(vec![vec![4]]));
// //        assert_eq!(get_yo_fix_variants("ЮНКЕРЬЁ"), None);
// //        assert_eq!(get_yo_fix_variants("ЮНКЕРЬЕ"), Some(vec![vec![6]]));
// //    }
// //
// //    #[test]
// //    fn vars_2yo() {
// //        assert_eq!(get_yo_fix_variants("трехведерный"), Some(vec![vec![2, 5], vec![2, 7]]));
// //        assert_eq!(get_yo_fix_variants("ТРЕХВЕДЕРНЫЙ"), Some(vec![vec![2, 5], vec![2, 7]]));
// //        assert_eq!(get_yo_fix_variants("трехведёрный"), Some(vec![vec![2]]));
// //        assert_eq!(get_yo_fix_variants("ТРЕХВЕДЁРНЫЙ"), Some(vec![vec![2]]));
// //        assert_eq!(get_yo_fix_variants("трехвёдерный"), Some(vec![vec![2]]));
// //        assert_eq!(get_yo_fix_variants("ТРЕХВЁДЕРНЫЙ"), Some(vec![vec![2]]));
// //        assert_eq!(get_yo_fix_variants("трёхведерный"), Some(vec![vec![5], vec![7]]));
// //        assert_eq!(get_yo_fix_variants("ТРЁХВЕДЕРНЫЙ"), Some(vec![vec![5], vec![7]]));
// //        assert_eq!(get_yo_fix_variants("трёхвёдерный"), None);
// //        assert_eq!(get_yo_fix_variants("ТРЁХВЁДЕРНЫЙ"), None);
// //        assert_eq!(get_yo_fix_variants("трёхведёрный"), None);
// //        assert_eq!(get_yo_fix_variants("ТРЁХВЕДЁРНЫЙ"), None);
// //    }
}
