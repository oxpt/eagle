import { Fragment } from "react";
import { CheckIcon, MinusIcon } from "@heroicons/react/20/solid";
import PageTitle from "@//components/pageTitle";

const tiers = [
  {
    name: "授業モード",
    id: "class",
    href: "#",
    shortName: "授業",
    description: "典型的な実験を即席で実施できます。",
    mostPopular: true,
  },
  {
    name: "研究モード",
    id: "research",
    href: "/conductor/research/restoration",
    shortName: "研究",
    description: "細かな実験設定が可能です。",
    mostPopular: false,
  },
  {
    name: "カスタムモード",
    id: "custom",
    href: "#",
    shortName: "カスタム",
    description: "独自実験を使用します。",
    mostPopular: false,
  },
];
const sections = [
  {
    name: "基盤機能",
    features: [
      {
        name: "実験条件設定",
        tiers: {
          授業モード: "簡易",
          研究モード: "詳細",
          カスタムモード: "カスタム",
        },
      },
      {
        name: "アカウント登録",
        tiers: {
          授業モード: "不要",
          研究モード: "必要",
          カスタムモード: "必要",
        },
      },
      {
        name: "事前連絡",
        tiers: {
          授業モード: "50人以下は不要*",
          研究モード: "50人以下は不要*",
          カスタムモード: "必要",
        },
      },
      {
        name: "データ保存期間",
        tiers: {
          授業モード: "当日中",
          研究モード: "1ヶ月",
          カスタムモード: "1ヶ月",
        },
      },
      {
        name: "過去実験の復元",
        tiers: { 授業モード: false, 研究モード: true, カスタムモード: true },
      },
      {
        name: "ログ取得",
        tiers: { 授業モード: false, 研究モード: true, カスタムモード: true },
      },
    ],
  },
  {
    name: "授業機能",
    features: [
      {
        name: "結果グラフ表示",
        tiers: { 授業モード: true, 研究モード: true, カスタムモード: true },
      },
    ],
  },
  {
    name: "研究機能",
    features: [
      {
        name: "被験者ID設定",
        tiers: { 授業モード: false, 研究モード: true, カスタムモード: true },
      },
      {
        name: "参加同意画面",
        tiers: { 授業モード: false, 研究モード: true, カスタムモード: true },
      },
      {
        name: "謝金計算",
        tiers: { 授業モード: false, 研究モード: true, カスタムモード: true },
      },
    ],
  },
];

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function RoomCreate() {
  return (
    <>
      <PageTitle
        title="ルーム作成"
        description="実験目的に適ったモードを選んでください。"
      />
      <div className="bg-white">
        <div className="mx-auto max-w-7xl">
          {/* xs to lg */}
          <div className="mx-auto max-w-md space-y-8 lg:hidden">
            {tiers.map((tier) => (
              <section
                key={tier.id}
                className={classNames(
                  tier.mostPopular
                    ? "rounded-xl bg-blue-400/5 ring-1 ring-inset ring-blue-200"
                    : "rounded-xl bg-gray-400/5 ring-1 ring-inset ring-gray-200",
                  "p-8"
                )}
              >
                <p id={tier.id} className="text-4xl font-bold text-gray-900">
                  {tier.name}
                </p>
                <a
                  href={tier.href}
                  aria-describedby={tier.id}
                  className={classNames(
                    tier.mostPopular
                      ? "bg-blue-500 text-white hover:bg-blue-700"
                      : "bg-white text-blue-500 ring-1 ring-inset ring-blue-100 hover:ring-blue-300",
                    "my-4 block max-w-sm   rounded-md px-3 py-2 text-center text-sm font-semibold leading-6 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500"
                  )}
                >
                  {tier.name === "カスタムモード"
                    ? "お問い合わせ"
                    : "ルーム作成"}
                </a>
                <ul
                  role="list"
                  className="mt-6 space-y-4 text-sm leading-6 text-gray-900"
                >
                  {sections.map((section) => (
                    <li key={section.name}>
                      <ul role="list" className="space-y-4">
                        {section.features.map((feature:any) =>
                          feature.tiers[tier.name] ? (
                            <li key={feature.name} className="flex gap-x-3">
                              <CheckIcon
                                className="h-6 w-5 flex-none text-blue-500"
                                aria-hidden="true"
                              />
                              <span>
                                {feature.name}{" "}
                                {typeof feature.tiers[tier.name] ===
                                "string" ? (
                                  <span className="text-sm leading-6 text-gray-500">
                                    ({feature.tiers[tier.name]})
                                  </span>
                                ) : null}
                              </span>
                            </li>
                          ) : null
                        )}
                      </ul>
                    </li>
                  ))}
                </ul>
              </section>
            ))}
          </div>

          {/* lg+ */}
          <div className="isolate hidden lg:block">
            <div className="relative -mx-8">
              {tiers.some((tier) => tier.mostPopular) ? (
                <div className="absolute inset-x-4 inset-y-0 -z-10 flex">
                  <div
                    className="flex w-1/4 px-4"
                    aria-hidden="true"
                    style={{
                      marginLeft: `${
                        (tiers.findIndex((tier) => tier.mostPopular) + 1) * 25
                      }%`,
                    }}
                  >
                    <div className="w-full rounded-t-xl border-x border-t border-gray-900/10 bg-gray-400/5" />
                  </div>
                </div>
              ) : null}
              <table className="w-full table-fixed border-separate border-spacing-x-8 text-left">
                <caption className="sr-only">Mode comparison</caption>
                <colgroup>
                  <col className="w-1/4" />
                  <col className="w-1/4" />
                  <col className="w-1/4" />
                  <col className="w-1/4" />
                </colgroup>
                <thead>
                  <tr>
                    <td />
                    {tiers.map((tier) => (
                      <th
                        key={tier.id}
                        scope="col"
                        className="px-6 pt-6 xl:px-8 xl:pt-8"
                      >
                        <span className="text-3xl font-bold">
                          {tier.shortName}
                        </span>
                        <span className="ml-1 text-sm font-semibold text-gray-600">
                          モード
                        </span>
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  <tr>
                    <th scope="row">
                      <span className="sr-only">Mode</span>
                    </th>
                    {tiers.map((tier) => (
                      <td key={tier.id} className="px-6 pt-2 xl:px-8">
                        <a
                          href={tier.href}
                          className={classNames(
                            tier.mostPopular
                              ? "bg-blue-500 text-white hover:bg-blue-700"
                              : "text-blue-500 ring-1 ring-inset ring-blue-100 hover:ring-blue-300",
                            "mt-8 block rounded-md px-3 py-2 text-center text-sm font-semibold leading-6 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500"
                          )}
                        >
                          {tier.name === "カスタムモード"
                            ? "お問い合わせ"
                            : "ルーム作成"}
                        </a>
                      </td>
                    ))}
                  </tr>
                  {sections.map((section, sectionIdx) => (
                    <Fragment key={section.name}>
                      <tr>
                        <th
                          scope="colgroup"
                          colSpan={4}
                          className={classNames(
                            sectionIdx === 0 ? "pt-8" : "pt-16",
                            "pb-4 text-sm font-semibold leading-6 text-gray-900"
                          )}
                        >
                          {section.name}
                          <div className="absolute inset-x-8 mt-4 h-px bg-gray-900/10" />
                        </th>
                      </tr>
                      {section.features.map((feature:any) => (
                        <tr key={feature.name}>
                          <th
                            scope="row"
                            className="py-4 text-sm font-normal leading-6 text-gray-900"
                          >
                            {feature.name}
                            <div className="absolute inset-x-8 mt-4 h-px bg-gray-900/5" />
                          </th>
                          {tiers.map((tier) => (
                            <td key={tier.id} className="px-6 py-4 xl:px-8">
                              {typeof feature.tiers[tier.name] === "string" ? (
                                <div className="text-center text-sm leading-6 text-gray-500">
                                  {feature.tiers[tier.name]}
                                </div>
                              ) : (
                                <>
                                  {feature.tiers[tier.name] === true ? (
                                    <CheckIcon
                                      className="mx-auto h-5 w-5 text-blue-500"
                                      aria-hidden="true"
                                    />
                                  ) : (
                                    <MinusIcon
                                      className="mx-auto h-5 w-5 text-gray-400"
                                      aria-hidden="true"
                                    />
                                  )}

                                  <span className="sr-only">
                                    {feature.tiers[tier.name] === true
                                      ? "Included"
                                      : "Not included"}{" "}
                                    in {tier.name}
                                  </span>
                                </>
                              )}
                            </td>
                          ))}
                        </tr>
                      ))}
                    </Fragment>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
          <div className="my-4 text-sm text-gray-500">
            *
            51人以上で実施する場合はサーバーを増強しますので、前日までにお問い合わせからご連絡ください。
          </div>
        </div>
      </div>
    </>
  );
}
