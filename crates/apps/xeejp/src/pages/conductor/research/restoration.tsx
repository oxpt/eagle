import PageTitle from "@/components/pageTitle";
import {
  PencilIcon,
  DocumentCheckIcon,
  PlayIcon,
  GiftIcon,
  UserGroupIcon,
  CalendarDaysIcon,
} from "@heroicons/react/20/solid";

import LoadingButton from "@/components/loadingButton";
import FooterNavButton from "@/components/footerNaviButton";

const setting = {
  data: [
    {
      id: 1,
      create_date: "2023/06/12 11:12",
      room_name: "a",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
    {
      id: 2,
      create_date: "2023/06/12 11:12",
      room_name: "テスト",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
    {
      id: 3,
      create_date: "2023/06/12 11:12",
      room_name: "マクロ",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
    {
      id: 4,
      create_date: "2023/06/12 11:12",
      room_name: "ミクロ",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
    {
      id: 5,
      create_date: "2023/06/12 11:12",
      room_name: "テスト",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
    {
      id: 6,
      create_date: "2023/06/12 11:12",
      room_name: "ヤフー",
      games: [1, 2, 3, 5, 4],
      guest_amount: 32,
      deleted: false,
    },
  ],
};

const games = [
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

export default function Restoration() {
  return (
    <>
      <PageTitle
        title="設定方法選択"
        description="設定方法を選択してください。"
      />
      <div className="mx-auto grid grid-cols-12 gap-3">
        <div className="col-span-12 mt-4">1. 既存の設定を複製する。</div>
        {setting.data.map((item) => (
          <div
            className="col-span-12 rounded-md border border-gray-300 bg-blue-50 px-4 py-2 sm:col-span-6 lg:col-span-3"
            key={item.id}
          >
            <dl className="flex flex-wrap">
              <div className="flex-auto py-2 text-base font-semibold leading-6 text-blue-900">
                {item.room_name}
              </div>
              <div className="flex w-full flex-none gap-x-4 border-t border-gray-900/5 pt-2">
                <dt className="flex-none">
                  <span className="sr-only">Create date</span>
                  <CalendarDaysIcon
                    className="h-6 w-5 text-gray-400"
                    aria-hidden="true"
                  />
                </dt>
                <dd className="text-sm leading-6 text-gray-500">
                  <time dateTime="2023-01-31">{item.create_date}</time>
                </dd>
              </div>
              <div className="mt-2 flex w-full flex-none gap-x-4">
                <dt className="flex-none">
                  <span className="sr-only">Guest amount</span>
                  <UserGroupIcon
                    className="h-6 w-5 text-gray-400"
                    aria-hidden="true"
                  />
                </dt>
                <dd className="text-sm leading-6 text-gray-500">
                  {item.guest_amount}人
                </dd>
              </div>
              <div className="mt-2 flex w-full flex-none gap-x-4">
                <dt className="flex-none">
                  <span className="sr-only">Games</span>
                  <PlayIcon
                    className="h-6 w-5 text-gray-400"
                    aria-hidden="true"
                  />
                </dt>
                <dd className="text-sm leading-6 text-gray-500">
                  {item.games.map((game) => (
                    <ul key={"ul_" + game} className="list-disc">
                      {games
                        .filter((g) => g.id === game)
                        .map((game_item) => (
                          <li key={"game_list_" + game_item.name}>
                            {game_item.name}
                          </li>
                        ))}
                    </ul>
                  ))}
                </dd>
              </div>
            </dl>
            <button className="mt-2 block w-full max-w-sm rounded-md bg-white px-3 py-2 text-center text-sm   font-semibold leading-6 text-blue-500 ring-1 ring-inset ring-blue-100 hover:ring-blue-300 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500">
              この設定を複製
            </button>
          </div>
        ))}

        <div className="col-span-12 mt-4">2. 既存の設定を読み込む。</div>

        <div className="col-span-12 flex w-full items-center justify-center">
          <label
            htmlFor="dropzone-file"
            className="flex h-64 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed border-gray-300 bg-gray-50 hover:bg-gray-100"
          >
            <div className="flex flex-col items-center justify-center pb-6 pt-5">
              <svg
                aria-hidden="true"
                className="mb-3 h-10 w-10 text-gray-400"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
                xmlns="http://www.w3.org/2000/svg"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth="2"
                  d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
                ></path>
              </svg>
              <p className="mb-2 text-sm text-gray-500">
                <span className="font-semibold">Click to upload</span> or drag
                and drop
              </p>
              <p className="text-xs text-gray-500 ">CSV (MAX. 1000MB)</p>
            </div>
            <input
              id="dropzone-file"
              type="file"
              className="hidden"
              accept=".csv"
            />
          </label>
        </div>

        <div className="col-span-12 mt-4">3. 新規作成する</div>

        <div className="col-span-12 mb-6 sm:col-span-6 lg:col-span-3">
          <LoadingButton text="新規作成する" loading={false} />
        </div>
      </div>
      <FooterNavButton
        leftTitle="前へ戻る"
        leftDescription="実験モードを選択しなおす"
        leftURL="../room_create"
        rightTitle="設定方法決定"
        rightDescription="実験順序設定へすすむ"
        rightURL="process"
      />
    </>
  );
}
