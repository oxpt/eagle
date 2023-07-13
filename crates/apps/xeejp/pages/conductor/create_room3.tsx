import { useRouter } from 'next/router';
import TitleHeader from '@/components/titleHeader';

const setting = {
  headers: [
    { name: '番号', hidden: 'none' },
    { name: 'ゲストID', hidden: 'none' },
    { name: 'ゲスト名', hidden: 'none' },
    { name: '入室日時', hidden: 'md' },
    { name: 'アクティブ', hidden: 'sm' },
    { name: 'キックアウト', hidden: 'none' },
  ],
  select: '選択',
  data: [
    {
      no: 1,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:05',
      kickout: false,
    },
    {
      no: 2,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:06',
      kickout: true,
    },
    {
      no: 3,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:07',
      kickout: false,
    },
    {
      no: 4,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:08',
      kickout: false,
    },
    {
      no: 5,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:09',
      kickout: false,
    },
    {
      no: 6,
      guest_id: 'abc-def',
      guest_name: '林 良平',
      entered_date: '2023/06/22 09:20',
      active: '2023/06/22 10:10',
      kickout: false,
    },
  ],
};

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(' ');
}

export default function HostCreateRoom2() {
  const router = useRouter();
  const handleClick = (e) => {
    e.preventDefault();
    router.push('/hosts/settings');
  };

  return (
    <>
      <div>
        <TitleHeader title="ゲストID登録" subTitle="ゲストIDを登録してください。" />
      </div>
      <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-4">
        <table className="min-w-full divide-y divide-gray-300">
          <thead>
            <tr>
              {setting.headers.map((header) => (
                <th
                  key={header.name}
                  scope="col"
                  className={classNames(
                    header.hidden === 'md' ? 'hidden lg:table-cell' : '',
                    header.hidden === 'sm' ? 'hidden sm:table-cell' : '',
                    header.name === 'キックアウト' ? 'text-right' : '',
                    'py-4 text-left text-sm font-semibold text-gray-900',
                  )}
                >
                  {header.name}
                </th>
              ))}
            </tr>
          </thead>
          <tbody className="divide-y divide-gray-200 bg-white">
            {setting.data.map((item) => (
              <tr key={item.no}>
                <td className="inline-flex py-4 text-sm font-medium text-gray-900 sm:w-auto sm:max-w-none">
                  {item.no}
                  <div className="my-1.5 sm:hidden w-2 h-2 mx-2 bg-red-600 rounded-full opacity-75 animate-[ping_5s_ease-in-out_infinite]" />
                </td>
                <td className="py-4 text-sm font-medium">
                  <input
                    type="text"
                    defaultValue={item.guest_id}
                    id="guest_id"
                    className="block w-full p-2 m-0 text-sm text-gray-900 rounded bg-gray-100 focus:ring-blue-500 focus:border-blue-500"
                    placeholder={item.guest_id}
                  />
                </td>
                <td className="py-4 text-sm font-medium">
                  <input
                    type="text"
                    defaultValue={item.guest_name}
                    id="guest_name"
                    className="block w-full p-2 m-0 text-sm text-gray-900 rounded bg-gray-100 focus:ring-blue-500 focus:border-blue-500"
                    placeholder={item.guest_name}
                  />
                </td>
                <td className="hidden py-4 text-sm text-gray-500 lg:table-cell">
                  {item.entered_date}
                </td>
                <td className="hidden py-4 text-sm text-gray-500 sm:table-cell">
                  <span className="flex w-2 h-2 mx-2 bg-red-600 rounded-full opacity-75 animate-[ping_5s_ease-in-out_infinite]" />
                </td>
                <td className="py-4 text-sm font-medium text-right">
                  <input
                    id={item.no}
                    aria-describedby="comments-description"
                    name="comments"
                    type="checkbox"
                    checked={item.kickout}
                    onChange={handleClick}
                    className="h-4 w-4 rounded border-gray-300 text-indigo-600 focus:ring-indigo-600"
                  />
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <div className="text-right mx-auto max-w-7xl px-4 sm:px-6 lg:px-8 py-4">
        <button
          disabled
          type="button"
          className="text-white bg-blue-500 hover:bg-blue-700 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center inline-flex items-center"
        >
          <svg
            aria-hidden="true"
            role="status"
            className="inline w-4 h-4 mr-3 text-white animate-spin"
            viewBox="0 0 100 101"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
          >
            <path
              d="M100 50.5908C100 78.2051 77.6142 100.591 50 100.591C22.3858 100.591 0 78.2051 0 50.5908C0 22.9766 22.3858 0.59082 50 0.59082C77.6142 0.59082 100 22.9766 100 50.5908ZM9.08144 50.5908C9.08144 73.1895 27.4013 91.5094 50 91.5094C72.5987 91.5094 90.9186 73.1895 90.9186 50.5908C90.9186 27.9921 72.5987 9.67226 50 9.67226C27.4013 9.67226 9.08144 27.9921 9.08144 50.5908Z"
              fill="#E5E7EB"
            />
            <path
              d="M93.9676 39.0409C96.393 38.4038 97.8624 35.9116 97.0079 33.5539C95.2932 28.8227 92.871 24.3692 89.8167 20.348C85.8452 15.1192 80.8826 10.7238 75.2124 7.41289C69.5422 4.10194 63.2754 1.94025 56.7698 1.05124C51.7666 0.367541 46.6976 0.446843 41.7345 1.27873C39.2613 1.69328 37.813 4.19778 38.4501 6.62326C39.0873 9.04874 41.5694 10.4717 44.0505 10.1071C47.8511 9.54855 51.7191 9.52689 55.5402 10.0491C60.8642 10.7766 65.9928 12.5457 70.6331 15.2552C75.2735 17.9648 79.3347 21.5619 82.5849 25.841C84.9175 28.9121 86.7997 32.2913 88.1811 35.8758C89.083 38.2158 91.5421 39.6781 93.9676 39.0409Z"
              fill="currentColor"
            />
          </svg>
          CSVダウンロード
        </button>
      </div>

      <ul className="mx-auto max-w-7xl grid w-full gap-4 px-4 sm:px-6 lg:px-8 py-4 md:grid-cols-2">
        <li>
          <input type="file" id="dropzone-file" name="file" className="hidden peer" accept=".csv" />
          <label
            htmlFor="dropzone-file"
            className="inline-flex items-center justify-between p-4 w-full border border-gray-300 rounded-lg cursor-pointer peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">被験者IDリストを読み込む</div>
              <div className="w-full">ここをクリックするかファイルをドラッグしてください。</div>
            </div>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              viewBox="0 0 20 20"
              fill="currentColor"
              className="w-6 h-6 ml-4"
            >
              <path
                fillRule="evenodd"
                d="M5.5 17a4.5 4.5 0 01-1.44-8.765 4.5 4.5 0 018.302-3.046 3.5 3.5 0 014.504 4.272A4 4 0 0115 17H5.5zm3.75-2.75a.75.75 0 001.5 0V9.66l1.95 2.1a.75.75 0 101.1-1.02l-3.25-3.5a.75.75 0 00-1.1 0l-3.25 3.5a.75.75 0 101.1 1.02l1.95-2.1v4.59z"
                clipRule="evenodd"
              />
            </svg>
          </label>
        </li>
        <li>
          <input
            type="button"
            id="laboratory"
            name="mode"
            defaultValue="laboratory"
            className="hidden peer"
            onClick={handleClick}
          />
          <label
            htmlFor="laboratory"
            className="inline-flex items-center justify-between w-full p-4  border border-gray-300 rounded-lg cursor-pointer peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">被験者ID決定</div>
              <div className="w-full">上記のIDで実施する。</div>
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
    </>
  );
}
