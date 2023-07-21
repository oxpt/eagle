import PageTitle from "@/components/pageTitle";
import FooterNavButton from "@/components/footerNaviButton";

import {
  ArrowLongRightIcon,
  ArrowLongDownIcon,
} from "@heroicons/react/20/solid";

import { setting, games } from "@/data/settings";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function Process() {
  return (
    <>
      <PageTitle
        title="実験順序設定"
        description="実施する実験を追加してください。"
      />
      <div className="mx-auto mb-12 grid grid-cols-12 gap-3">
        <div className="col-span-12 sm:col-span-5">
          <label htmlFor="game" className="mb-1 block text-sm text-gray-900">
            実験一覧
          </label>
          <select
            id="game"
            size={10}
            className="ml-[1px] block w-full rounded-lg border border-gray-300 bg-gray-50 p-2 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500"
          >
            {games.map((game) => (
              <option value={game.id} key={game.id}>
                {game.name}
              </option>
            ))}
          </select>
        </div>
        <div className="col-span-12 items-center text-center sm:hidden">
          <div className="block">
            <button
              type="button"
              className="rounded-full bg-blue-500 p-1 text-center text-white shadow-sm hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-700"
            >
              <ArrowLongDownIcon className="h-5 w-5" aria-hidden="true" />
            </button>
          </div>
        </div>
        <div className="hidden h-full items-center text-center sm:col-span-2 sm:grid">
          <div className="block">
            <button
              type="button"
              className="rounded-full bg-blue-500 p-1 text-center text-white shadow-sm hover:bg-blue-700 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-700"
            >
              <ArrowLongRightIcon className="h-5 w-5" aria-hidden="true" />
            </button>
          </div>
        </div>
        <div className="col-span-12 sm:col-span-5">
          <label htmlFor="game" className="mb-1 block text-sm text-gray-900">
            実施順
          </label>
          <div className="flow-root">
            <ul role="list" className="-mb-8">
              {setting.timeline.map((event, eventIdx) =>
                games
                  .filter((games) => games.id === event.game_id)
                  .map((game) => (
                    <li key={"timeline_" + event.id}>
                      <div className="relative pb-8">
                        {eventIdx !== setting.timeline.length - 1 ? (
                          <span
                            className="absolute left-4 top-4 -ml-px h-full w-0.5 bg-gray-200"
                            aria-hidden="true"
                          />
                        ) : null}
                        <div className="relative flex space-x-3">
                          <div>
                            <span
                              className={classNames(
                                game.iconBackground,
                                "flex h-8 w-8 items-center justify-center rounded-full ring-8 ring-white"
                              )}
                            >
                              <game.icon
                                className="h-5 w-5 text-white"
                                aria-hidden="true"
                              />
                            </span>
                          </div>

                          <div className="block min-w-0 justify-start space-x-4 pt-1.5">
                            <div>
                              <p className="text-sm font-medium text-gray-900">
                                {game.name}{" "}
                              </p>
                            </div>
                            {game.result && (
                              <div className="space-y-5">
                                <div className="relative flex items-start">
                                  <div className="flex h-6 items-center">
                                    <input
                                      id="comments"
                                      aria-describedby="comments-description"
                                      name="comments"
                                      type="checkbox"
                                      defaultChecked={event.result}
                                      className="h-4 w-4 rounded border-gray-300 text-blue-700 focus:ring-blue-700"
                                    />
                                  </div>
                                  <div className="ml-3 text-sm leading-6">
                                    <label
                                      htmlFor="comments"
                                      className="font-medium text-gray-900"
                                    >
                                      結果ページ表示
                                    </label>
                                  </div>
                                </div>
                              </div>
                            )}

                            {game.continue && (
                              <div className="space-y-5">
                                <div className="relative flex items-start">
                                  <div className="flex h-6 items-center">
                                    <input
                                      id="comments"
                                      aria-describedby="comments-description"
                                      name="comments"
                                      type="checkbox"
                                      defaultChecked={event.continue}
                                      className="h-4 w-4 rounded border-gray-300 text-blue-700 focus:ring-blue-700"
                                    />
                                  </div>
                                  <div className="ml-3 text-sm leading-6">
                                    <label
                                      htmlFor="comments"
                                      className="font-medium text-gray-900"
                                    >
                                      個別進行
                                    </label>
                                  </div>
                                </div>
                              </div>
                            )}
                          </div>
                        </div>
                      </div>
                    </li>
                  ))
              )}
            </ul>
          </div>
        </div>
      </div>

      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="設定方法を選択しなおす"
        leftURL="restoration"
        rightTitle="実験順序決定"
        rightDescription="同意文編集へ進む"
        rightURL="description"
      />
    </>
  );
}
