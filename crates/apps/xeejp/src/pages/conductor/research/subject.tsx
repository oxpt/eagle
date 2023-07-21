import { PlusIcon, MinusIcon } from "@heroicons/react/20/solid";
import PageTitle from "@/components/pageTitle";
import LoadingButton from "@/components/loadingButton";
import FooterNavButton from "@/components/footerNaviButton";

import { setting } from "@/data/settings";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

export default function Subject() {
  return (
    <>
      <PageTitle
        title="被験者登録"
        description="実験参加者を登録・キックアウト・削除してください。"
      />
      <div className="mx-auto grid grid-cols-12 gap-3">
        <div className="col-span-12 sm:col-span-6 lg:col-span-3">
          <LoadingButton text="CSVダウンロード" loading={true} />
        </div>
        <div className="col-span-12 sm:col-span-6 lg:col-span-3">
          <LoadingButton text="CSVアップロード" loading={false} />
        </div>
        <div className="hidden lg:col-span-6 lg:block" />
        <div className="col-span-12 rounded-md border border-gray-300 px-4 py-4 sm:col-span-6 lg:col-span-3">
          <span className="my-1 mr-1 inline-flex items-center rounded-md bg-green-50 px-2 py-1 text-xs font-medium text-green-700 ring-1 ring-inset ring-green-600/20">
            Active
          </span>
          <span className="inline-flex items-center rounded-md  px-2 py-1 text-xs font-medium text-gray-700 ring-1 ring-inset">
            2023/06/22 21:00
          </span>
          <input
            type="text"
            defaultValue="abcdefg"
            id="guest_id"
            className="mb-1 block w-full rounded border-none bg-gray-100 p-2 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500"
            placeholder="被験者ID"
          />
          <input
            type="text"
            defaultValue="林良平"
            id="guest_name"
            className="mb-1 block w-full rounded border-none bg-gray-100 p-2 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500"
            placeholder="お名前"
          />
          <input
            type="text"
            defaultValue="A"
            id="category"
            className="mb-1 block w-full rounded border-none bg-gray-100 p-2 text-sm text-gray-900 focus:border-blue-500 focus:ring-blue-500"
            placeholder="カテゴリー"
          />
          <div className="relative flex items-start  justify-between">
            <div className="flex h-6 items-center">
              <input
                id="comments"
                aria-describedby="comments-description"
                name="comments"
                type="checkbox"
                className="h-4 w-4 rounded border-gray-300 text-blue-500 focus:ring-blue-500"
              />
              <div className="ml-3 text-sm ">
                <label htmlFor="comments" className="font-medium text-gray-900">
                  キックアウトする
                </label>
              </div>
            </div>
            <button
              type="button"
              className="rounded-full bg-blue-300 p-1 text-white hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500"
            >
              <PlusIcon className="h-4 w-4" aria-hidden="true" />
            </button>
          </div>
        </div>

        {setting.guest_list.map((item) => (
          <div
            className="col-span-12 rounded-md border border-gray-300 bg-blue-50 px-4 py-4 sm:col-span-6 lg:col-span-3"
            key={item.no}
          >
            <button
              type="button"
              className="relative -left-3 -top-8 mx-1 inline-flex  items-end justify-end rounded-full bg-red-300 p-0.5 align-middle text-white  hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-500"
            >
              <MinusIcon className="h-4 w-4" aria-hidden="true" />
            </button>
            <span
              className={classNames(
                item.kickout
                  ? " bg-red-50 text-red-700 ring-red-700/20"
                  : " bg-green-50 text-green-700 ring-green-700/20",
                "my-1 -ml-7 mr-1 inline-flex items-center rounded-md px-2 py-1 text-xs font-medium  ring-1 ring-inset"
              )}
            >
              {item.kickout ? "Dead" : "Active"}
            </span>
            <span className="inline-flex items-center rounded-md  px-2 py-1 text-xs font-medium text-gray-700 ring-1 ring-inset">
              2023/06/22 21:00
            </span>
            <input
              type="text"
              defaultValue="abcdefg"
              id="guest_id"
              className="mb-1 block w-full  rounded border-none bg-blue-50 p-2 text-sm text-gray-900 hover:border-gray-300 hover:bg-gray-100 focus:border-blue-500 focus:bg-gray-100 focus:ring-blue-500"
              placeholder="被験者ID"
            />
            <input
              type="text"
              defaultValue="林良平"
              id="guest_name"
              className="mb-1 block w-full  rounded border-none bg-blue-50 p-2 text-sm text-gray-900  hover:border-gray-300 hover:bg-gray-100 focus:border-blue-500 focus:bg-gray-100 focus:ring-blue-500"
              placeholder="お名前"
            />
            <input
              type="text"
              defaultValue="A"
              id="category"
              className="mb-1 block w-full  rounded border-none bg-blue-50 p-2 text-sm text-gray-900  hover:border-gray-300 hover:bg-gray-100 focus:border-blue-500 focus:bg-gray-100 focus:ring-blue-500"
              placeholder="カテゴリー"
            />
            <div className="relative flex items-start  justify-between">
              <div className="flex h-6 items-center">
                <input
                  id="comments"
                  aria-describedby="comments-description"
                  name="comments"
                  type="checkbox"
                  className="roundedborder-gray-300 h-4 w-4  rounded text-blue-500 focus:ring-blue-500"
                />
                <div className="ml-3 text-sm">
                  <label
                    htmlFor="comments"
                    className="font-medium text-gray-900"
                  >
                    キックアウトする
                  </label>
                </div>
              </div>
            </div>
          </div>
        ))}
      </div>

      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="言語を設定しなおす"
        leftURL="localize"
        rightTitle="被験者入力終了"
        rightDescription="報酬設定へすすむ"
        rightURL="payment"
      />
    </>
  );
}
