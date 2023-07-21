import PageTitle from "@/components/pageTitle";
import FooterNavButton from "@/components/footerNaviButton";

import { setting } from "@/data/settings";

export default function Localize() {
  return (
    <>
      <PageTitle
        title="言語設定"
        description="言語に合わせて用語を指定してください。"
      />
      {setting.locales.map((value) => {
        return (
          <div
            key={value.language}
            className="grid grid-cols-1 gap-x-8 gap-y-2 border-b border-gray-900/10 pb-12 md:grid-cols-12"
          >
            <div className="col-span-1 md:col-span-4">
              <h2 className="text-base font-semibold leading-7 text-gray-900">
                最後通牒ゲーム
              </h2>
              <p className="mt-1 text-sm leading-6 text-gray-600">
                {value.language}
              </p>
            </div>
            <div className="col-span-1 gap-x-6 gap-y-8 md:col-span-4">
              {value.variables.map((element) => {
                return (
                  <div
                    key={value.language + "_" + element.key}
                    className="mt-4 md:mt-8"
                  >
                    <label
                      htmlFor="round_num"
                      className="text-sm font-semibold leading-6 text-gray-900"
                    >
                      {element.key}
                    </label>
                    <div className="mt-2">
                      <div className="flex rounded-md shadow-sm ring-1 ring-inset ring-gray-300 focus-within:ring-2 focus-within:ring-inset focus-within:ring-indigo-600">
                        <input
                          type="text"
                          name="round_num"
                          id="round_num"
                          className="block flex-1 border-0 bg-transparent py-1.5 pr-2 text-gray-900 placeholder:text-gray-400 focus:ring-0 sm:text-sm sm:leading-6"
                          placeholder={element.locale}
                          defaultValue={element.locale}
                        />
                      </div>
                    </div>
                  </div>
                );
              })}
            </div>
          </div>
        );
      })}
      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="個別実験を設定しなおす"
        leftURL="experiment"
        rightTitle="言語設定決定"
        rightDescription="被験者登録へ進む"
        rightURL="subject"
      />
    </>
  );
}
