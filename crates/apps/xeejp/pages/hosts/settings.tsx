import Link from 'next/link';
import { useRouter } from 'next/router';
import TitleHeader from '@/components/titleHeader';

const setting = {
  headers: [
    { name: '作成日', hidden: 'none' },
    { name: 'ルーム名', hidden: 'none' },
    { name: '実施ゲーム', hidden: 'sm' },
    { name: '被験者数', hidden: 'md' },
    { name: '選択', hidden: 'none' },
  ],
  select: '選択',
  data: [
    {
      create_date: '2021/12/24',
      room_name: 'あ',
      games: '最後通牒ゲーム、独裁者ゲーム',
      participants: 30,
      id: 1,
    },
    {
      create_date: '2021/12/24',
      room_name: 'あ',
      games: '最後通牒ゲーム、独裁者ゲーム',
      participants: 30,
      id: 2,
    },
    {
      create_date: '2021/12/24',
      room_name: 'あ',
      games: '最後通牒ゲーム、独裁者ゲーム',
      participants: 30,
      id: 3,
    },
    {
      create_date: '2021/12/24',
      room_name: 'あ',
      games: '最後通牒ゲーム、独裁者ゲーム',
      participants: 30,
      id: 4,
    },
  ],
};

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(' ');
}

export default function Settings() {
  const router = useRouter();
  const handleClick = (e) => {
    e.preventDefault();
    router.push('/hosts/create_room3');
  };

  return (
    <div>
      <TitleHeader title="設定方法選択" subTitle="設定方法を選択してください。" />
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
                    header.name === '選択' ? 'text-right' : '',
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
              <tr key={item.id}>
                <td className="max-w-0 py-4 text-sm font-medium text-gray-900 sm:w-auto sm:max-w-none">
                  {item.create_date}
                  <dl className="font-normal lg:hidden">
                    <dt className="sr-only sm:hidden">Games</dt>
                    <dd className="mt-1 truncate text-gray-500 sm:hidden">{item.games}</dd>
                    <dt className="sr-only">Participants</dt>
                    <dd className="mt-1 truncate text-gray-700">{item.participants}</dd>
                  </dl>
                </td>
                <td className="py-4 text-sm text-gray-500">{item.room_name}</td>
                <td className="hidden py-4 text-sm text-gray-500 lg:table-cell">
                  {item.room_name}
                </td>
                <td className="hidden py-4 text-sm text-gray-500 sm:table-cell">{item.games}</td>
                <td className="py-4 text-sm font-medium text-right">
                  <Link href="/" className="text-blue-700 hover:text-blue-900">
                    {setting.select}
                    <span className="sr-only">{item.id}</span>
                  </Link>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
      <ul className="mx-auto max-w-7xl grid w-full gap-4 px-4 sm:px-6 lg:px-8 py-4 md:grid-cols-2">
        <li>
          <input type="file" id="dropzone-file" name="file" className="hidden peer" accept=".csv" />
          <label
            htmlFor="dropzone-file"
            className="inline-flex items-center justify-between  p-4 w-full border border-gray-300 rounded-lg cursor-pointer   peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">設定ファイルを読み込む</div>
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
            value="laboratory"
            className="hidden peer"
            onClick={handleClick}
          />
          <label
            htmlFor="laboratory"
            className="inline-flex items-center justify-between w-full p-4  border border-gray-300 rounded-lg cursor-pointer peer-checked:border-blue-700 peer-checked:text-blue-700 hover:text-gray-700 hover:bg-gray-100"
          >
            <div className="block">
              <div className="w-full text-lg font-semibold">新規作成する</div>
              <div className="w-full">新しい実験を設定する場合はこちら。</div>
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
