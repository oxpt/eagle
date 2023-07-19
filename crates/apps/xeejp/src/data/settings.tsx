import {
  PencilIcon,
  DocumentCheckIcon,
  PlayIcon,
  GiftIcon,
} from "@heroicons/react/20/solid";

export const setting = {
  timeline: [
    {
      id: 1,
      game_id: 1,
      result: false,
      continue: false,
      max: 1,
      min: 0,
      peymentType: "join",
      peyment: [
        {
          category: "A",
          rate: {
            english: 10,
            japanese: 1000,
            spanish: 10,
          },
        },
      ],
    },
    {
      id: 2,
      game_id: 2,
      result: false,
      continue: false,
      max: 0,
      min: 0,
      peymentType: "none",
      peyment: [
        {
          category: "A",
          rate: {
            english: 0,
            japanese: 0,
            spanish: 0,
          },
        },
      ],
    },
    {
      id: 3,
      game_id: 3,
      result: true,
      continue: true,
      max: 1000,
      min: 0,
      peymentType: "point",
      peyment: [
        {
          category: "A",
          rate: {
            english: 0.01,
            japanese: 1,
            spanish: 0.01,
          },
        },
      ],
    },
    {
      id: 4,
      game_id: 3,
      result: false,
      continue: false,
      max: 1000,
      min: 0,
      peymentType: "point",
      peyment: [
        {
          category: "A",
          rate: {
            english: 0.01,
            japanese: 1,
            spanish: 0.01,
          },
        },
        {
          category: "B",
          rate: {
            english: 0.01,
            japanese: 1,
            spanish: 0.01,
          },
        },
      ],
    },
    {
      id: 5,
      game_id: 4,
      result: false,
      continue: false,
      max: 0,
      min: 0,
      peymentType: "none",
      peyment: [
        {
          category: "A",
          rate: {
            english: 0,
            japanese: 0,
            spanish: 0,
          },
        },
      ],
    },
  ],
  guest_list: [
    {
      no: 1,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:05",
      kickout: false,
    },
    {
      no: 2,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:06",
      kickout: true,
    },
    {
      no: 3,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:07",
      kickout: false,
    },
    {
      no: 4,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:08",
      kickout: false,
    },
    {
      no: 5,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:09",
      kickout: false,
    },
    {
      no: 6,
      guest_id: "abc-def",
      guest_name: "林 良平",
      entered_date: "2023/06/22 09:20",
      active: "2023/06/22 10:10",
      kickout: false,
    },
  ],
  agreement: {
    pageTitle: {
      title: "同意説明",
      discription: "以下の説明文をよく読んでください。",
    },
    researchTitle: {
      titleHead: "研究課題名",
      title: "文化混在状況下における協力行動の動学的実証研究（その１）",
      discription:
        "この説明文は、あなたにこの研究の内容を正しく理解していただき、あなたの自由な意思に基づいて、この研究に参加するかどうかを判断していただくためのものです。<br />この研究への協力の同意は、あなたの自由意志で決めてください。同意しなくてもあなたの不利益になるようなことはありません。<br />また、一旦同意した場合でも、あなたが不利益を受けることなく、いつでも同意を撤回することができます。<br />以下の項目をよく読んで、同意できる場合は一番下にあるボタンを押して、同意書を表示してください。",
    },
    listItems: [
      {
        title: "研究について",
        discription:
          '研究の実施にあたっては、高知工科大学倫理審査委員会の審査を経て、研究機関の長より許可を受けています。この研究が許可されているのは<span className="underline">2028年3月31日</span>までです。<br />また、研究計画の変更、実施方法の変更が生じる場合には適宜審査を受け、安全性と人権に最大の配慮をします。',
      },
      {
        title: "研究の意義・目的",
        discription:
          "この研究では、人間の社会行動には一般的にどのような傾向があるかを明らかにする目的で実施されます。年齢、性別、住んでいる地域などの属性の違いが、信頼、協力、処罰などの行動に与える影響を検証します。",
      },
      { title: "研究の対象", discription: "18歳以上の日本在住者。" },
      {
        title: "研究の方法",
        discription:
          "この研究では、みなさんにコンピュータを操作してもらい、いくつかの質問に答えてもらいます。<br />最初に、みなさんの属性（年齢や性別など）に関する質問をします。<br />次に、みなさんをコンピュータ上でいくつかのグループにわけ、同じグループになった他の参加者とともに、ポイントを分け合う経済実験を行ってもらいます。この経済実験のポイントは、実験終了時にお渡しする実験報酬額と連動しています。<br />最後に、本日ご参加いただいた参稼報酬と、実験によって得られた実験報酬の合計額を、実験参加謝金としてお支払いします。",
      },
      {
        title: "予測される（物理的及び精神的）危険及びその対応",
        discription:
          "実験中に簡単な作業をしてもらう場合があります。その作業は誰にでもできる日常的な作業です。作業内容は実験前に詳細に説明します。あなたが支障がないと合意した場合にのみ実施していただきます。また、作業中にいつでも合意を取り消して中止することもできます。<br />不明な点がありましたら、実験前、実験後にかかわらず、実験実施者にお尋ねください。なお実験中は質問できない期間があります。",
      },
      {
        title: "研究対象者にもたらされる利益及び不利益",
        discription:
          "あなたは、実験参加によって報酬（実験参加謝金）を受け取ることができます。<br />あなたは、実験参加によって不利益を被る可能性は極めて少ないです。",
      },
      {
        title: "経済的負担や報酬について",
        discription:
          "あなたは、本日ご参加いただいた参加報酬（900円）と、実験によって得られた実験報酬（0円〜900円）の合計額（900円〜1,800円）を、実験参加謝金として実験終了時に受け取れます。<br />あなたは、実験前、実験中のいつでも実験を途中で中止して、実験室から出ることができます。実験を途中で中止した場合は、参加報酬（900円）だけを受け取れ、実験報酬（0円〜900円）は受け取れません。",
      },
      {
        title: "個人情報の保護について",
        discription:
          "この研究をすすめる上で必要な属性情報、心理的傾向、実験結果の情報以外は収集しません。データは匿名のID番号を用いて統計処理され、個人が特定できる情報と切り離して分析されます。また、対外的に報告される場合も、個人が特定できない集計した結果を報告しますので、個人が特定されることはありません。",
      },
      {
        title: "データの保管について",
        discription:
          "この研究のデータは電子記憶媒体に保存し、高知工科大学永国寺キャンパスA609の鍵付き保管庫にて保管されます。データを分析する際は、セキュリティ対策が施されたコンピュータのみで処理されます。データの分析・保管は研究代表者が行います。<br />共同研究者が分析する場合は、研究代表者から個人が特定できない情報のみ貸出し、分析終了後に全データを回収して、研究代表者が保管します。",
      },
      {
        title: "研究の費用について",
        discription:
          "この研究は高知工科大学研究費及び科学研究費補助金 国際共同研究加速基金（国際共同研究強化（A))を原資に実施されます。",
      },
      {
        title: "利益相反について",
        discription:
          "この研究に関して、企業等との関わりや、研究成果や参加者の保護に影響を及ぼす可能性のあるすべての経済的利益関係等の状況はありません。",
      },
      {
        title: "研究に関する情報公開の方法について",
        discription:
          "この研究に参加した方々の個人情報の保護や、この研究の独創性の確保に支障がない範囲で、この研究の計画書や研究の方法に関する資料を閲覧することができます。資料の閲覧を希望される方はどうぞお申し出ください。<br />この研究の成果は、学会等にて学術雑誌等で公表されますが、あなた個人が特定される情報を削除した上で発表されます。",
      },
      {
        title: "研究の実施体制について",
        discription:
          "この研究は以下の体制で実施します。<br />研究実施場所：高知工科大学永国寺キャンパス<br />研究責任者：林 良平<br />研究分担者：小谷 浩示、Moinul Islam<br />共同研究機関：マッコーリー大学 経済実験研究室（オーストラリア）<br />研究分担者： Maroš Servátka",
      },
      {
        title: "お問合せ",
        discription:
          "この研究に関して質問等がありましたら、以下の研究責任者にお問合せください。<br />所属：高知工科大学 経済・マネジメント学群<br />研究責任者：林 良平<br />住所：高知県高知市永国寺町２番２号<br />E-mail: hayashi.ryohei@kochi-tech.ac.jp",
      },
      { title: "説明日", discription: "2023年6月5日" },
      { title: "説明者", discription: "林 良平" },
    ],
    confirm: {
      discription:
        "上記の項目をよく読んで、同意できる場合は名前を入力してください。",
      buttonText: "同意",
    },
  },
  experiment: {},
  locales: [
    {
      language: "English",
      variables: [
        { key: "distributor", locale: "distributor" },
        { key: "respondent", locale: "respondent" },
      ],
    },
    {
      language: "日本語",
      variables: [
        { key: "distributor", locale: "distributor" },
        { key: "respondent", locale: "respondent" },
      ],
    },
    {
      language: "スペイン語",
      variables: [
        { key: "distributor", locale: "distributor" },
        { key: "respondent", locale: "respondent" },
      ],
    },
  ],
};

export const games = [
  {
    id: 1,
    name: "参加同意文",
    result: false,
    continue: true,
    icon: DocumentCheckIcon,
    iconBackground: "bg-gray-500",
  },
  {
    id: 2,
    name: "属性調査",
    result: false,
    continue: true,
    icon: PencilIcon,
    iconBackground: "bg-gray-500",
  },
  {
    id: 3,
    name: "最後通牒ゲーム",
    result: true,
    continue: true,
    icon: PlayIcon,
    iconBackground: "bg-green-500",
  },
  {
    id: 4,
    name: "報酬支払い",
    result: true,
    continue: true,
    icon: GiftIcon,
    iconBackground: "bg-gray-500",
  },
];

export const locales = {
  english: "€",
  japanese: "¥",
  spanish: "€",
};
