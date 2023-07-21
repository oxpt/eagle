import PageTitle from "@/components/pageTitle";
import FooterNavButton from "@/components/footerNaviButton";

import { setting, games, locales } from "@/data/settings";

export default function Payment() {
  return (
    <>
      <PageTitle title="報酬設定" description="報酬を設定してください。" />
      {setting.timeline.map((value) => {
        return (
          <div
            key={value.id}
            className="grid grid-cols-1 gap-x-8 gap-y-2 border-b border-gray-900/10 pb-12 md:grid-cols-12"
          >
            <div className="col-span-1 md:col-span-4">
              {games
                .filter((game) => game.id === value.game_id)
                .map((game) => (
                  <h2
                    key={"h2_" + value.id}
                    className="text-base font-semibold leading-7 text-gray-900"
                  >
                    {game.name}
                  </h2>
                ))}
              <p className="mt-1 text-sm leading-6 text-gray-600">
                {value.peymentType === "join" &&
                  "参加報酬 (" + value.min + "-" + value.max + ")"}
                {value.peymentType === "point" &&
                  "実験報酬 (" + value.min + "-" + value.max + ")"}
                {value.peymentType === "none" && "報酬設定なし"}
              </p>
            </div>
            {value.peymentType !== "none" && (
              <div className="col-span-1 gap-x-6 gap-y-8 md:col-span-4">
                {value.peyment.map((element) => {
                  return (
                    <div
                      key={value.id + "_" + element.category}
                      className="mt-4 md:mt-8"
                    >
                      <label
                        htmlFor="round_num"
                        className="text-sm font-semibold leading-6 text-gray-900"
                      >
                        被験者カテゴリ：{element.category}
                      </label>
                      <div className="mt-2">
                        <p className="text-sm leading-6 text-gray-900">英語</p>
                        <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                          <input
                            type="number"
                            name="round_num"
                            id="round_num"
                            className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                            placeholder={element.rate.english.toString()}
                            defaultValue={element.rate.english}
                          />
                          <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                            {locales.english}
                          </span>
                        </div>
                      </div>

                      <div className="mt-2">
                        <p className="text-sm leading-6 text-gray-900">
                          日本語
                        </p>
                        <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                          <input
                            type="number"
                            name="round_num"
                            id="round_num"
                            className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                            placeholder={element.rate.japanese.toString()}
                            defaultValue={element.rate.japanese}
                          />
                          <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                            {locales.japanese}
                          </span>
                        </div>
                      </div>

                      <div className="mt-2">
                        <p className="text-sm leading-6 text-gray-900">
                          スペイン語
                        </p>
                        <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                          <input
                            type="number"
                            name="round_num"
                            id="round_num"
                            className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                            placeholder={element.rate.spanish.toString()}
                            defaultValue={element.rate.spanish}
                          />
                          <span className="flex select-none items-center pr-2 text-gray-500 sm:text-sm">
                            {locales.spanish}
                          </span>
                        </div>
                      </div>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        );
      })}

      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="被験者を登録しなおす"
        leftURL="subject"
        rightTitle="報酬設定終了"
        rightDescription="設定内容を確認する"
        rightURL="confirmation"
      />
    </>
  );
}
