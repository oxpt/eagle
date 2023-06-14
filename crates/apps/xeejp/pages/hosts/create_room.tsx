import TitleHeader from '@/components/titleHeader';

export default function CreateRoom() {
  return (
    <div>
      <TitleHeader title="実験モード選択" subTitle="実験モードを選択してください。" />
      <ul className="mx-auto max-w-7xl grid w-full gap-4 px-4 sm:px-6 lg:px-8 py-4 md:grid-cols-2">
        <li>
          <input
            type="radio"
            id="classroom"
            name="mode"
            value="classroom"
            className="hidden peer"
            required
          />
          <label
            htmlFor="classroom"
            className="inline-flex items-center justify-between w-full p-4  border border-gray-300 rounded-lg cursor-pointer peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">授業モード（ゲストID自動生成）</div>
              <div className="w-full">ルーム名だけで実験できる簡易版です。</div>
            </div>
            <svg
              aria-hidden="true"
              className="w-6 h-6 ml-3"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fillRule="evenodd"
                d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </label>
        </li>
        <li>
          <input
            type="radio"
            id="laboratory"
            name="mode"
            value="laboratory"
            className="hidden peer"
          />
          <label
            htmlFor="laboratory"
            className="inline-flex items-center justify-between w-full p-4  border border-gray-300 rounded-lg cursor-pointer peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">研究モード（ゲストID事前登録）</div>
              <div className="w-full">詳細なゲーム設定ができる厳密版です。</div>
            </div>
            <svg
              aria-hidden="true"
              className="w-6 h-6 ml-4"
              fill="currentColor"
              viewBox="0 0 20 20"
              xmlns="http://www.w3.org/2000/svg"
            >
              <path
                fillRule="evenodd"
                d="M12.293 5.293a1 1 0 011.414 0l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-2.293-2.293a1 1 0 010-1.414z"
                clipRule="evenodd"
              />
            </svg>
          </label>
        </li>
      </ul>
    </div>
  );
}
