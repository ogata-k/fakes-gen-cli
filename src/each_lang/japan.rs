use crate::each_lang::Data;
use rand::Rng;
use crate::helper::gen_range;

pub struct JapanData {}

impl Data for JapanData {
    // Lorem
    const WORDS: &'static [&'static str] = &[];
    const SENTENCE: &'static [&'static str] = &[];
    const PARAGRAPH: &'static [&'static str] = &[];

    // Name
    const IS_LAST_NAME_IS_FIRST: bool = true;
    const FIRST_NAME: &'static [&'static str] = &[];
    const LAST_NAME: &'static [&'static str] = &[];

    // Company
    const COMPANY_SUFFIX: &'static [&'static str] = &[
        "株式会社", "有限会社", "合同会社", "合名会社", "合資会社", "任意組合", "匿名組合", "投資事業有限責任組合", "有限責任事業組合",
    ];
    const COMPANY_NAME: &'static [&'static str] = &[];
    const INDUSTRY: &'static [&'static str] = &[
        "農業", "電気業", "不動産取引業", "林業", "ガス業", "不動産賃貸業・管理業", "漁業", "熱供給業", "物品賃貸業", "水産養殖業", "水道業",
        "学術・開発研究機関", "通信業", "専門サービス業", "放送業", "広告業", "総合工事業", "情報サービス業", "技術サービス業", "職別工事業",
        "インターネット付随サービス業", "宿泊業", "設備工事業", "映像・音声・文字情報制作業", "飲食店食料品製造業", "鉄道業", "持ち帰り・配達飲食サービス業",
        "飲料・たばこ・飼料製造業", "道路旅客運送業", "洗濯・理容・美容・浴場業繊維工業", "道路貨物運送業", "その他の生活関連サービス業",
        "木材・木製品製造業", "水運業", "娯楽業", "家具・装備品製造業", "航空運輸業", "学校教育パルプ・紙・紙加工品製造業", "倉庫業", "その他の教育、学習支援業",
        "印刷・同関連業", "運輸に附帯するサービス業", "医療業化学工業", "郵便業", "信書便事業", "保健衛生", "石油製品・石炭製品製造業",
        "各種商品卸売業社会保険・社会福祉・介護事業", "プラスチック製品製造業", "繊維・衣服等卸売業", "郵便局ゴム製品製造業", "飲食料品卸売業",
        "協同組合", "なめし革・同製品・毛皮製造業建", "築材料、鉱物・金属材料等卸売業", "廃棄物処理業", "窯業・土石製品製造業", "機械器具卸売業",
        "自動車整備業", "鉄鋼業", "その他の卸売業", "機械等修理業", "非鉄金属製造業", "各種商品小売業", "職業紹介・労働者派遣業", "金属製品製造業",
        "織物・衣服・身の回り品小売業", "その他の事業サービス業", "はん用機械器具製造業", "飲食料品小売業", "政治・経済・文化団体", "生産用機械器具製造業",
        "機械器具小売業", "宗教", "業務用機械器具製造業", "その他の小売業", "その他のサービス業", "電子部品・デバイス・電子回路製造業", "無店舗小売業",
        "外国公務電気機械器具製造業", "銀行業", "国家公務", "情報通信機械器具製造業", "協同組織", "金融業", "地方公務", "輸送用機械器具製造業",
        "貸金業、クレジットカード業等非預金信用機関", "分類不能の産業", "その他の製造業", "金融商品取引業、商品先物取引業", "補助的金融業等", "保険業"
    ];

    // Address
    const STREET_NAME: &'static [&'static str] = &[];
    const CITY_NAME: &'static [&'static str] = &[];
    const STATE_NAME: &'static [&'static str] = &[];
    const COUNTRY_NAME: &'static [&'static str] = &[
        "アルバ", "アフガニスタン", "アンゴラ", "アングイラ", "オーランド諸島", "アルバニア", "アンドラ", "オランダ領アンティル", "アラブ首長国連邦", "アルゼンチン",
        "アルメニア", "アメリカ領サモア", "南極大陸", "フランス南領域", "アンチグア・バーブーダ", "オーストラリア", "オーストリア", "アゼルバイジャン", "ブルンジ",
        "ベルギー", "ベナン", "ブルキナファソ", "バングラデシュ", "ブルガリア", "バーレーン", "バハマ", "ボスニア・ヘルツェゴビナ", "サンバルテルミ", "ベラルーシ",
        "ベリーズ", "バミューダ", "ボリビア", "ブラジル", "バルバドス", "ブルネイダルサラーム", "ブータン", "ブーヴェ島", "ボツワナ", "中央アフリカ共和国", "カナダ",
        "ココヤシ島", "スイス", "チリ", "中国", "コートジボアール", "カメルーン", "コンゴ民主主義共和国", "コンゴ", "クック諸島", "コロンビア", "コモロ", "カーボベルデ",
        "コスタリカ", "キューバ", "クリスマス島", "ケイマン諸島", "キプロス", "チェコ共和国", "ドイツ", "ジブチ", "ドミニカ", "デンマーク", "ドミニカ共和国", "アルジェリア",
        "エクアドル", "エジプト", "エリトリア", "西サハラ", "スペイン", "エストニア", "エチオピア", "フィンランド", "フィジー", "フォークランド諸島（マルビナス諸島）",
        "フランス", "フェロー諸島", "ミクロネシア連邦", "ガボン", "イギリス", "ジョージア", "ガーンジー島", "ガーナ", "ジブラルタル", "ギニア", "グアドループ", "ガンビア",
        "ギニアビサウ", "赤道ギニア", "ギリシャ", "グレナダ", "グリーンランド", "グアテマラ", "仏領ギアナ", "グアム", "ガイアナ", "香港", "ハード島とマクドナルド諸島",
        "ホンジュラス", "クロアチア", "ハイチ", "ハンガリー", "インドネシア", "マン島", "インド", "英領インド洋", "アイルランド", "イラン", "イラク", "アイスランド",
        "イスラエル", "イタリア", "ジャマイカ", "ジャージー", "ヨルダン", "日本", "カザフスタン", "ケニア", "キルギスタン", "カンボジア", "キリバス", "セント・キッツとネヴィス",
        "韓国", "クウェート", "ラオス人民民主共和国", "レバノン", "リベリア", "リビア", "セントルシア", "リヒテンシュタイン", "スリランカ", "レソト", "リトアニア", "ルクセンブルク",
        "ラトビア", "マカオ", "サン・マルタン", "モロッコ", "モナコ", "モルダヴィア共和国", "マダガスカル", "モルジブ", "メキシコ", "マーシャル群島", "マケドニア", "マリ",
        "マルタ", "ミャンマー", "モンテネグロ", "モンゴル", "北マリアナ諸島", "モザンビーク", "モーリタニア", "モントセラト島", "マルティニク", "モーリシャス", "マラウイ",
        "マレーシア", "メイヨット", "ナミビア", "ニューカレドニア", "ニジェール", "ノーフォーク島", "ナイジェリア", "ニカラグア", "ニウエ島", "オランダ", "ノルウェー",
        "ネパール", "ナウル", "ニュージーランド", "オーマン", "パキスタン", "パナマ", "ピトケアン", "ペルー", "フィリピン", "パラオ", "パプアニューギニア", "ポーランド",
        "プエルトリコ", "ポルトガル", "パラグアイ", "仏領ポリネシア", "カタール", "レユニオン島", "ルーマニア", "ロシア連邦", "ルワンダ", "サウジアラビア", "スーダン",
        "セネガル", "シンガポール", "サウスジョージアとサウス・サンドイッチ諸島", "セントヘレナ,アセンション,およびトリスタンダクーチ", "スバールバルとヤンマイエン",
        "ソロモン諸島", "シエラレオーネ", "エルサルバドル", "サンマリノ", "ソマリア", "サン・ピエールとミクロン", "セルビア", "サントメプリンシペ", "スリナム", "スロバキア",
        "スロベニア", "スウェーデン", "スワジランド", "セイシェル", "シリア・アラブ共和国", "タークスカイコス諸島", "チャド", "トーゴ", "タイ", "タジキスタン", "トケラウ",
        "トルクメニスタン", "ティモールレステ", "トンガ", "トリニダード・トバゴ", "チュニジア", "トルコ", "ツヴァル", "台湾", "タンザニア共和国", "ウガンダ",
        "ウクライナ", "ウルグアイ", "米国", "ウズベキスタン", "バチカン市国", "セントビンセントおよびグレナディーン諸島", "ベネズエラ, ボリビア共和国",
        "バージン諸島", "ベトナム", "ヴァヌアツ", "ウォリスとフトゥーナ", "サモア", "イエメン", "南アフリカ", "ザンビア", "ジンバブエ",
    ];
    const BUILDING: &'static [&'static str] = &[];

    fn build_address(street: &str, city: &str, state: &str) -> String {
        format!("{}{}{}", state, city, street)
    }

    fn gen_zip_code<R: Rng>(rnd: &mut R, hyphen: bool) -> String {
        let a: u16 = gen_range(rnd, 0, 999);
        let b: u16 = gen_range(rnd, 0, 9999);
        return if hyphen {
            format!("{:>03}-{:>04}", a, b)
        } else {
            format!("{:>03}{:>04}", a, b)
        };
    }

    fn gen_domestic_phone_number<R: Rng>(rnd: &mut R, hyphen: bool) -> String {
        let a: u8 = gen_range(rnd, 0, 9);
        let b: u16 = gen_range(rnd, 0, 999);
        let c: u16 = gen_range(rnd, 0, 9999);
        return if hyphen {
            format!("{:>02}-{:>03}-{:>04}", a, b, c)
        } else {
            format!("{:>02}{:>03}{:>04}", a, b, c)
        };
    }
}
