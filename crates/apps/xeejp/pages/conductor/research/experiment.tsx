import PageTitle from "@/components/pageTitle";
import FooterNavButton from "@/components/footerNaviButton";
import { useState } from "react";
import { Switch } from "@headlessui/react";

// import { setting } from '@/data/settings';

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function ExperimentSettings() {
  const [enabled, setEnabled] = useState(false);

  return (
    <>
      <PageTitle
        title="個別実験設定"
        description="各実験の実施方法を選択してください。"
      />
      <div className="grid grid-cols-1 gap-x-8 gap-y-2 border-b border-gray-900/10 pb-12 md:grid-cols-12">
        <div className="col-span-1 mt-4 md:col-span-4">
          <h2 className="text-base font-semibold leading-7 text-gray-900">
            最後通牒ゲーム
          </h2>
          <p className="mt-1 text-sm leading-6 text-gray-600">実験順 1</p>
        </div>

        <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
          <label
            htmlFor="round_num"
            className="text-sm font-semibold leading-6 text-gray-900"
          >
            ラウンド数
          </label>
          <div className="mt-2">
            <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
              <input
                type="number"
                name="round_num"
                id="round_num"
                className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                placeholder="1"
                defaultValue={1}
              />
              <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                回
              </span>
            </div>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <label
              htmlFor="round_num"
              className="text-sm font-semibold leading-6 text-gray-900"
            >
              配分者と応答者の役割交代
            </label>
            <div className="mt-2">
              <Switch.Group as="div" className="flex items-center">
                <Switch.Label as="span" className="mr-3 text-sm">
                  <span className="font-medium text-gray-900">
                    役割交代しない
                  </span>
                </Switch.Label>
                <Switch
                  checked={enabled}
                  onChange={setEnabled}
                  className={classNames(
                    enabled ? "bg-indigo-600" : "bg-gray-200",
                    "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                  )}
                >
                  <span
                    aria-hidden="true"
                    className={classNames(
                      enabled ? "translate-x-5" : "translate-x-0",
                      "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
                    )}
                  />
                </Switch>
                <Switch.Label as="span" className="ml-3 text-sm">
                  <span className="font-medium text-gray-900">
                    役割交代する
                  </span>
                </Switch.Label>
              </Switch.Group>
            </div>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <label
              htmlFor="round_num"
              className="text-sm font-semibold leading-6 text-gray-900"
            >
              応答者に拒否された時の再提案回数
            </label>
            <div className="mt-2">
              <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                <input
                  type="number"
                  name="round_num"
                  id="round_num"
                  className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                  placeholder="0"
                  defaultValue={0}
                />
                <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                  回
                </span>
              </div>
            </div>
            <div className="relative flex gap-x-3">
              <div className="flex h-6 items-center">
                <input
                  id="comments"
                  name="comments"
                  type="checkbox"
                  className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                />
              </div>
              <div className="text-sm leading-6">
                <label htmlFor="comments" className="font-medium text-gray-900">
                  無限回
                </label>
                <p className="text-gray-500">
                  承諾されるか実験終了まで回数制限なしで提案できます。
                </p>
              </div>
            </div>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <label
              htmlFor="round_num"
              className="text-sm font-semibold leading-6 text-gray-900"
            >
              初期保有額
            </label>
            <div className="mt-2">
              <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                <input
                  type="number"
                  name="round_num"
                  id="round_num"
                  className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                  placeholder="1000"
                  defaultValue={1000}
                />
                <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                  ポイント
                </span>
              </div>
            </div>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <fieldset>
              <legend className="text-sm font-semibold leading-6 text-gray-900">
                ゲストに見せる画面
              </legend>
              <div className="mt-2 space-y-6">
                <div className="relative flex gap-x-3">
                  <div className="flex h-6 items-center">
                    <input
                      id="comments"
                      name="comments"
                      type="checkbox"
                      className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                    />
                  </div>
                  <div className="text-sm leading-6">
                    <label
                      htmlFor="comments"
                      className="font-medium text-gray-900"
                    >
                      説明画面
                    </label>
                    <p className="text-gray-500">
                      実験説明文がスライド形式で表示されます。独自説明資料などがあり、説明画面を表示したくない場合はオフにしてください。
                    </p>
                  </div>
                </div>
                <div className="relative flex gap-x-3">
                  <div className="flex h-6 items-center">
                    <input
                      id="candidates"
                      name="candidates"
                      type="checkbox"
                      className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                    />
                  </div>
                  <div className="text-sm leading-6">
                    <label
                      htmlFor="candidates"
                      className="font-medium text-gray-900"
                    >
                      実験画面
                    </label>
                    <p className="text-gray-500">
                      説明のみで終了して実験は実施しない場合などはオフにしてください。
                    </p>
                  </div>
                </div>
                <div className="relative flex gap-x-3">
                  <div className="flex h-6 items-center">
                    <input
                      id="offers"
                      name="offers"
                      type="checkbox"
                      className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                    />
                  </div>
                  <div className="text-sm leading-6">
                    <label
                      htmlFor="offers"
                      className="font-medium text-gray-900"
                    >
                      結果画面
                    </label>
                    <p className="text-gray-500">
                      実験結果グラフ、ランキング表を表示します。ゲストに相対的な順位を知られたくない場合などはオフにして下さい。
                    </p>
                  </div>
                </div>
              </div>
            </fieldset>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <label
              htmlFor="round_num"
              className="text-sm font-semibold leading-6 text-gray-900"
            >
              各ラウンドの提案終了タイミング
            </label>
            <div className="mt-2">
              <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                <input
                  type="number"
                  name="round_num"
                  id="round_num"
                  className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                  placeholder="120"
                />
                <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                  秒
                </span>
              </div>
            </div>
            <div className="relative flex gap-x-3">
              <div className="flex h-6 items-center">
                <input
                  id="comments"
                  name="comments"
                  type="checkbox"
                  className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                />
              </div>
              <div className="text-sm leading-6">
                <label htmlFor="comments" className="font-medium text-gray-900">
                  提案者が送信した時点
                </label>
                <p className="text-gray-500">
                  グループ毎にゲームが進行していきます。
                </p>
              </div>
            </div>
          </div>

          <div className="col-span-1 mt-4 gap-x-6 gap-y-8 md:col-span-4 md:mt-8">
            <label
              htmlFor="round_num"
              className="text-sm font-semibold leading-6 text-gray-900"
            >
              配分額決定過程表示
            </label>
            <div className="mt-2">
              <Switch.Group as="div" className="flex items-center">
                <Switch.Label as="span" className="mr-3 text-sm">
                  <span className="font-medium text-gray-900">表示しない</span>{" "}
                  <p className="text-sm text-gray-500">
                    応答者には待機指示画面が表示されます。
                  </p>
                </Switch.Label>
                <Switch
                  checked={enabled}
                  onChange={setEnabled}
                  className={classNames(
                    enabled ? "bg-indigo-600" : "bg-gray-200",
                    "relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-indigo-600 focus:ring-offset-2"
                  )}
                >
                  <span
                    aria-hidden="true"
                    className={classNames(
                      enabled ? "translate-x-5" : "translate-x-0",
                      "pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out"
                    )}
                  />
                </Switch>
                <Switch.Label as="span" className="ml-3 text-sm">
                  <span className="font-medium text-gray-900">表示する</span>{" "}
                  <p className="text-sm text-gray-500">
                    配分者の操作に伴ってスライダーバーが動きます。
                  </p>
                </Switch.Label>
              </Switch.Group>
            </div>
          </div>
        </div>
      </div>

      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="同意文を編集しなおす"
        leftURL="description"
        rightTitle="個別実験設定決定"
        rightDescription="言語設定へすすむ"
        rightURL="localize"
      />
    </>
  );
}
