// Sonic
//
// Fast, lightweight and schema-less search backend
// Copyright: 2019, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

pub static STOPWORDS_KAT: &[&str] = &[
    "ა.შ.",
    "აგერ",
    "აგრეთვე",
    "ალბათ",
    "ამაზე",
    "ამას",
    "ამასთან",
    "ამასთანავე",
    "ამგვარად",
    "ამდენად",
    "ამით",
    "ამის",
    "ამისთვის",
    "ამიტომ",
    "ამიტომაც",
    "ამჟამად",
    "ამჯერად",
    "ან",
    "ანუ",
    "არ",
    "არა",
    "არადა",
    "არათუ",
    "არამარტო",
    "არამედ",
    "არამხოლოდ",
    "არანაკლებ",
    "არასოდეს",
    "არაუადრეს",
    "არაუგვიანეს",
    "არაუმეტეს",
    "არსად",
    "არსაიდან",
    "არც",
    "არცერთ",
    "ასევე",
    "ასეც",
    "აქამდე",
    "აღარ",
    "აღარც",
    "ბოლოს",
    "ბოლოსკენ",
    "გამო",
    "გამუდმებით",
    "განსაკუთრებით",
    "გარდა",
    "გარეშე",
    "და",
    "დასასრულს",
    "დასაწყისში",
    "დროულად",
    "ე.ი.",
    "ე.წ.",
    "ეგებ",
    "ერთადერთი",
    "ერთადერთმა",
    "ერთ-ერთი",
    "ერთხელ",
    "ესოდე",
    "ვერ",
    "ვითომ",
    "ვინაიდან",
    "ვინძლო",
    "ვისაც",
    "ზემოაღნიშნულმა",
    "ზოგჯერ",
    "თავად",
    "თავადაც",
    "თავადვე",
    "თავდაპირველად",
    "თავიდანვე",
    "თავის მხრივ",
    "თან",
    "თანაც",
    "თანახმადაც",
    "თანდათან",
    "თვით",
    "თვითონ",
    "თვითონაც",
    "თვითონვე",
    "თითოეულმა",
    "თითქოს",
    "თუ",
    "თუკი",
    "თუმცა",
    "თუმცაღა",
    "თუნდაც",
    "იმავდროულად",
    "იმავე",
    "იმან",
    "იმას",
    "იმდენად",
    "იმთავითვე",
    "იმით",
    "იმის",
    "იმისთვის",
    "იმიტომ",
    "ისევე",
    "ისეთი",
    "ისეც",
    "იშვიათად",
    "კერძოდ",
    "კვლავ",
    "კი",
    "კიდევ",
    "მაგალითად",
    "მაგან",
    "მაგას",
    "მაგით",
    "მაგის",
    "მაგრამ",
    "მათი",
    "მაინც",
    "მანამ",
    "მანამდე",
    "მართალია",
    "მარტო",
    "მაშასადამე",
    "მაშინ",
    "მაშინვე",
    "მერე",
    "მეტად",
    "მთელი",
    "მიერ",
    "მით",
    "მიმართ",
    "მისივე",
    "მსგავსი",
    "მხოლოდ",
    "ნაწილობრივ",
    "ნეტავ",
    "ნეტავი",
    "ნუ",
    "ნურასოდეს",
    "ნურც",
    "ნუღარ",
    "ნუღარც",
    "ოდენ",
    "ოდესღაც",
    "ოღონდ",
    "პირველი",
    "პირიქით",
    "პრინციპში",
    "რადგან",
    "რადგანაც",
    "რათა",
    "რაკი",
    "რამდენად",
    "რამდენადაც",
    "რამეთუ",
    "რამენაირად",
    "რამეფრად",
    "რანაირადაც",
    "რასაკვირველია",
    "რასაც",
    "რაღაც",
    "რაც",
    "რითაც",
    "რისთვისაც",
    "როგორადაც",
    "როგორიც",
    "როგორიცაა",
    "როგორღაც",
    "როგორც",
    "როდესაც",
    "როდესღაც",
    "რომ",
    "რომელიმე",
    "რომელიც",
    "რომელსაც",
    "რომლებიც",
    "რომლითაც",
    "რომლის",
    "როცა",
    "საბოლოოდ",
    "სადაც",
    "სადღაც",
    "საერთოდ",
    "სათანადოდ",
    "საიდანაც",
    "სამომავლოდ",
    "სანამ",
    "სანამდე",
    "სრულად",
    "სულ",
    "სწორედ",
    "სხვადასხვა",
    "სხვები",
    "უკვე",
    "უნდა",
    "უსათუოდ",
    "უფრო",
    "უცებ",
    "უცნაურად",
    "ფაქტობრივად",
    "ყველა",
    "ყოველგვარი",
    "ყოველთვის",
    "ყოველი",
    "ყოველივე",
    "შედარებით",
    "შედეგად",
    "შემდგომ",
    "შემდგომში",
    "შემდეგ",
    "შესახებ",
    "შორის",
    "ჩვეულებრივ",
    "წინააღმდეგ",
    "წინაშე",
    "ხან",
    "ხოლმე",
    "ხოლო",
    "ხშირად",
    "ჯერაც",
    "ჯერჯერობით",
    "ამის გარდა",
    "ამის გარეშე",
    "ამის მიუხედავად",
    "ამასთან ერთად",
    "ამის მიხედვით",
    "ამის ნაცვლად",
    "ამის პასუხად",
    "ამასთან შედარებით",
    "ამბობს, რომ",
    "ამ დროს",
    "ამ თემაზე",
    "ამ მიზნით",
    "ამის საპირისპიროდ",
    "ამის გამო",
    "ამ მხრივ",
    "ამის უარსაყოფად",
    "ამის შედეგად",
    "ამ შემთხვევაში",
    "ამავე დროს",
    "ამას გარდა",
    "ამასთან დაკავშირებით",
    "ამის შემდეგ",
    "ამის შესაბამისად",
    "ამის შესახებ",
    "ამისგან განსხვავებით",
    "არა მარტო",
    "არა მხოლოდ",
    "არა უადრეს",
    "არა უგვიანეს",
    "არც ერთი",
    "არც კი",
    "არც მეორე",
    "ასე ვთქვათ",
    "ასე მაგალითად",
    "ასე რომ",
    "ასე შემდეგ",
    "ასევე განიხილავს",
    "აქედან გამომდინარე",
    "აქედან დასკვნა",
    "აღნიშნა რომ",
    "აღნიშნულთან დაკავშირებით",
    "აცხადებს რომ",
    "ბოლო ერთი",
    "ბოლო პერიოდში",
    "ბოლო წლებში",
    "გამოთქვა იმედი",
    "განაცხადა, რომ",
    "განმარტა, რომ",
    "გარდა ამისა",
    "გარშემო არსებული",
    "და სხვ.",
    "და სხვა",
    "დაადასტურა, რომ",
    "ეგრეთ წოდებული",
    "ეგრეთ წოდებულმა",
    "ერთი თვალსაზრისით",
    "ერთი მხრივ",
    "ერთის მხრივ",
    "ეს კი",
    "ესე იგი",
    "ვიდრე არ",
    "თავიდან ბოლომდე",
    "თუ რამდენად",
    "თუ როგორ",
    "იგივეა რაც",
    "იმ შემთხვევაში",
    "იმაზე მეტი",
    "იმაზე, რომ",
    "იმას, რომ",
    "იმასთან დაკავშირებით",
    "იმდენად რამდენადაც",
    "იმედი გამოთქვა",
    "იმის გამო",
    "იმის თაობაზე",
    "იმის საწინააღმდეგოდ",
    "იმისათვის, რომ",
    "იმისთვის, რათა",
    "იმისთვის, რომ",
    "იმიტომ, რომ",
    "ის, რომელიც",
    "ისე როგორც",
    "ისე, რომ",
    "ისევე როგორც",
    "ისეთი როგორიც",
    "იქიდან გამომდინარე",
    "კიდევ ერთხელ",
    "მაგრამ თუ",
    "მათ შორის",
    "მათი ვარაუდით",
    "მანამ, სანამ",
    "მას შემდეგ",
    "მაშინ, როცა",
    "მაშინაც კი",
    "მეორე მხრივ",
    "მეორეც ერთი",
    "მერე მეორე",
    "მით უფრო",
    "მიიჩნევს, რომ",
    "მისი განმარტებით",
    "მისი თქმით",
    "მისივე თქმით",
    "მიუხედავად ამისა",
    "ნურც კი",
    "პირველ რიგში",
    "რა დროსაც",
    "რა მიზეზითაც",
    "რაც შეეხება",
    "რაც შეიძლება",
    "რის გამოც",
    "რის საფუძველზედაც",
    "რის საფუძველზეც",
    "რის შედეგადაც",
    "რის შემდეგაც",
    "როგორც კი",
    "რომ არა",
    "რომ თუ",
    "რომელთა გამოც",
    "რომლის თანახმად",
    "რომლის თანახმადაც",
    "რომლის მიხედვითაც",
    "რომლის შესახებ",
    "საკითხთან დაკავშირებით",
    "სულ მცირე",
    "სულ ცოტა",
    "სხვა კუთხით",
    "სხვა მხრივ",
    "სხვა რამ",
    "სხვათა შორის",
    "უფრო მეტიც",
    "ყოველივე ეს",
    "შემდეგ უკვე",
    "ჩვენი განცხადებით",
    "ჯერ ერთი",
    "ჯერ კიდევ",
    "ამ ბოლო დროს",
    "ამა თუ იმ",
    "ასე თუ ისე",
    "აქედან ჩანს, რომ",
    "ბოლოს და ბოლოს",
    "გამომდინარე იქიდან, რომ",
    "და ასე შემდეგ",
    "ვინაიდან და რადგანაც",
    "თუ რის საფუძველზე",
    "იმის გათვალისწინებით, რომ",
    "იმის გამო, რომ",
    "იმის ნაცვლად, რომ",
    "ისევ და ისევ",
    "იქვე აღნიშნა, რომ",
    "იქიდან გამომდინარე, რომ",
    "კიდევ და კიდევ",
    "მაინც და მაინც",
    "მას შემდეგ, რაც",
    "მიუხედავად იმისა, თუ",
    "მიუხედავად იმისა, რომ",
    "როგორც უკვე ითქვა",
    "როდის და რატომ",
    "უფრო და უფრო",
];
