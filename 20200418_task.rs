use std::collections::HashMap;

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

fn main() {
  let input_str: String = "Ямуцймямщтрэ фцпк, щцпхзхрм уёлмс щчцщцихг т рпфмхмхрё. Чцщмжй ъзф эзцщ, фг хмпзфмъхц чцлфмхрф рэ юмххцщър хз ьзударйгм р пзщъзйрф рэ й еър ьзударйгм юмххцщър ймшръд. Тзт? Фг хзслмф щйцрэ млрхцфгаумххртцй, щйцрэ щцёпхртцй й щзфцс Шцщщрр.\nНащшяф шр нащшяфяэ сдфхг бршлублтргмвп убрюфщяшюрп ая втяхэд эрвигрсд гбрухфщп ущсхьщ врэяуя юхаяыябюяуя юр шхэьх юрбяфр, яыяюзргхьмюяуя щ юхясбргщэяуя дурврющп хуя врэявяшюрющп.\nЩз нарсщ оюлдсючх ьыррсэухомяи х ьыръхщмяи ямч ъмфзомсщзв варыуъхчыо, чыяыэзс юямъая ъмюмурмяи х ормшншхомяи о дсшыосдсючыс юыфъмъхс чашия юсчюм, ъмюхшхл, юмрхфщм, ьэсрмясшиюяом – юшыоыщ, оюлчыц НСФЪЭМОЮЯОСЪЪЫЮЯХ. О аьэмошсъхх пыюармэюяоыщ щз юыфрмрхщ вмыю х ъсэмфнсэхва.\nЁф ъмэюё жюбщёюлжз, жз щдлвыжз в изклзшжжз кизкзъклызыщлх кщёзэмйклым рвжзыжвдзы, ийзпыюлщжвч ыбшлзржвдзы в ъюкийвжпвижзклв. Ъчйздйщлвбё в ызездвлщ ъмэмл ызбызэвлхкш ы эзъйзэюлюех. Рюклжзклх в изйшэзржзклх ъмэмл зкёювыщлхкш в жвдзём жю клщжмл жмажф, ийюыйщлшлкш ы июйюавлзд ийзсезьз. Ощёклыз в жщьезклх, езах в зъёщж, ихшжклыз в жщйдзёщжвч, авызлжфг клйщо эймь июйюэ эймьзё в ъюббщклюжрвызклх, ийюэщлюехклыз, жщпвзжщевбё в ыйщаэм жщйзэзы – ийюаэю ыкюьз ыйщаэм в жюжщывклх д ймккдзём жщйзэм, - ыкю цлз ёф ъмэюё езыдз в жюбщёюлжз дмехлвывйзыщлх, ыкю цлз йщкпыюлюл ёщойзыфё пыюлзё.\nШфиц Миффнъи.".to_string();



  let solution = vec![25,16,20,7,24];

  println!("{:?}", input_str.split("\n").zip(solution.into_iter()).map(|x| make_shifted(x.0, x.1)).collect::<Vec<String>>());
  


//  let _str: String = input_str.split("\n").skip(0).next().unwrap_or("").to_owned();

//  try_reference(&_str);
  
//  brute_force_solution(&_str);

}

#[allow(dead_code)]
fn brute_force_solution(text: &str) {
  let alphabet: Vec<char> = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars().collect();
  let upper_alphabet: Vec<char> = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".chars().collect();
  assert_eq!(alphabet.len(), 33);
  assert_eq!(upper_alphabet.len(), 33);
//  println!("alphabet lens: {} {}", alphabet.len(), upper_alphabet.len());

//  println!("{:?}", alphabet.iter().map(|x| { *x as u32 }).collect::<Vec<_>>());

  for i in 0..33 {
    let opt: String = text.chars().map(|x| {
      let x1: u32 = x as u32;
      match x1 {
        1025 => upper_alphabet[((6 + i)%33) as usize],
        0..=1039 => x,
        1040..=1045 => upper_alphabet[((x1 - 1040 + i)%33) as usize],
        1046..=1071 => upper_alphabet[((x1 - 1040 + 1 + i)%33) as usize],
        1072..=1077 => alphabet[((x1 - 1072 + i)%33) as usize],
        1105 => alphabet[((6 + i)%33) as usize],
        1078..=1103 => alphabet[((x1 - 1072 + 1 + i)%33) as usize],
        _ => x
      }
    }).collect();
    println!("\n{}\n{}", i, opt);
  }
}

fn make_shifted(text: &str, shift: u32) -> String {
  let alphabet: Vec<char> = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя".chars().collect();
  let upper_alphabet: Vec<char> = "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ".chars().collect();

  let res: String = text.chars().map(|x| {
    let x1: u32 = x as u32;
    match x1 {
      1025 => upper_alphabet[((6 + shift)%33) as usize],
      0..=1039 => x,
      1040..=1045 => upper_alphabet[((x1 - 1040 + shift)%33) as usize],
      1046..=1071 => upper_alphabet[((x1 - 1040 + 1 + shift)%33) as usize],
      1072..=1077 => alphabet[((x1 - 1072 + shift)%33) as usize],
      1105 => alphabet[((6 + shift)%33) as usize],
      1078..=1103 => alphabet[((x1 - 1072 + 1 + shift)%33) as usize],
      _ => x
    }
  }).collect();
  res
}

#[allow(dead_code)]
fn try_reference(text: &str) {
  let reference = vec!['о', 'e', 'а', 'и', 'н', 'т', 'с', 'р', 'в', 'л', 'к', 'м', 'д', 'п', 'у', 'я', 'ы', 'ь', 'г', 'з', 'б', 'ч', 'й', 'х', 'ж', 'ш', 'ю', 'ц', 'щ', 'э', 'ф', 'ъ', 'ё'];

  println!("{}: {}", function!(), text.len());

  let l_text = text.to_lowercase();

  let mut fr: HashMap<char, u32> = HashMap::new();
  for s in l_text.chars() {
    *fr.entry(s).or_insert(0) += 1;
  }

  let mut count_vec: Vec<_> = fr.iter().filter(|i| ![' ', '\n', '.', '?', '!', ',', '-', '–'].contains(i.0)).collect();

  count_vec.sort_by(|a, b| a.1.cmp(b.1));
  count_vec.reverse();
  println!("{:?}\n{}", count_vec, count_vec.len());

  count_vec.reverse();
  let char_map: HashMap<char, char> = count_vec.into_iter().zip(reference.into_iter()).map(|x| ((x.0).0.clone(), x.1)).collect();

  let res: String = l_text.chars().map(|x| char_map.get(&x).unwrap_or(&x).clone()).collect();
  println!("{}", res);
}
