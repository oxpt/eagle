import { Disclosure, Menu, Transition } from '@headlessui/react';
import { Bars3Icon, UserIcon, XMarkIcon } from '@heroicons/react/24/outline';
import { UserCircleIcon } from '@heroicons/react/24/solid';
import Image from 'next/image';
import Link from 'next/link';
import { Fragment } from 'react';
import 'flag-icons';

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(' ');
}

const header = {
  name: 'XEE.JP',
  url: 'http://localhost:3000',
  logo: '/logo_right.png',
  menu: [
    { name: 'home', url: '/visitors/home' },
    { name: 'about', url: '/visitors/about' },
    { name: 'service', url: '/visitors/service' },
    { name: 'pricing', url: '/visitors/pricing' },
    { name: 'contact', url: '/visitors/contact' },
  ],
  languageDefault: {
    name: '日本語',
    url: './jp',
    flag: <span className="fi fi-jp mr-2 h-5 w-5" aria-hidden="true" />,
  },
  languages: [
    {
      name: '日本語',
      url: './jp',
      flag: <span className="fi fi-jp mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: 'English (US)',
      url: './en_us',
      flag: <span className="fi fi-us mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: 'Español',
      url: './es',
      flag: <span className="fi fi-es mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: '中文 (繁體)',
      url: './cn',
      flag: <span className="fi fi-cn mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
  ],
  naviMenus: [
    { name: '概要', url: '/visitors/about' },
    { name: '使い方', url: '/visitors/docs' },
    { name: 'ゲーム一覧', url: '/visitors/games' },
  ],
  userMenus: [
    {
      title: 'ゲストメニュー',
      menus: [{ name: 'ルーム一覧', url: '/guests/room_list' }],
    },
    {
      title: 'ホストメニュー',
      menus: [
        { name: 'ルーム開設', url: '/hosts/create_room' },
        { name: 'ルーム管理', url: '/hosts/room_management' },
      ],
    },
    {
      title: 'ユーザーメニュー',
      menus: [
        { name: 'アカウント管理', url: '/hosts/account' },
        { name: 'お問合せ', url: '/visitors/contact' },
      ],
    },
  ],
};

export default function Header() {
  return (
    <header>
      <Disclosure as="nav" className="bg-white shadow">
        {({ open }) => (
          <>
            <div className="mx-auto max-w-7xl px-4 sm:px-6 lg:px-8">
              <div className="flex h-16 justify-between">
                <div className="flex">
                  <Link href="/" className="-ml-2 mr-2 flex items-center">
                    <Image
                      src={header.logo}
                      width={90}
                      height={90}
                      alt="XEE.JP"
                      className="h-8 w-auto"
                    />
                    <span className="ml-2 font-extrabold text-3xl text-blue-700">
                      {header.name}
                    </span>
                  </Link>
                  <div className="hidden md:ml-8 md:flex md:space-x-8">
                    {header.naviMenus.map((menu) => (
                      <Link
                        key={menu.name}
                        href={menu.url}
                        className="inline-flex items-center border-transparent px-1 pt-1 text-sm font-medium text-gray-900 hover:border-b-2 hover:border-blue-500"
                      >
                        {menu.name}
                      </Link>
                    ))}
                  </div>
                </div>
                <div className="flex items-center">
                  <div className="shrink-0">
                    <Menu as="div" className="relative ml-4">
                      <div>
                        <Menu.Button className="relative inline-flex w-full items-center rounded-md bg-white px-3 py-2 text-sm text-gray-500 shadow-sm hover:bg-blue-500 hover:text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500">
                          <span className="sr-only">Language select menu</span>
                          {header.languageDefault.flag}
                          {header.languageDefault.name}
                        </Menu.Button>
                      </div>
                      <Transition
                        as={Fragment}
                        enter="transition ease-out duration-200"
                        enterFrom="transform opacity-0 scale-95"
                        enterTo="transform opacity-100 scale-100"
                        leave="transition ease-in duration-75"
                        leaveFrom="transform opacity-100 scale-100"
                        leaveTo="transform opacity-0 scale-95"
                      >
                        <Menu.Items className="absolute right-0 z-10 mt-2 w-44 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                          {header.languages.map((language) => (
                            <Menu.Item key={language.name}>
                              {({ active }) => (
                                <Link
                                  href={language.url}
                                  className={classNames(
                                    active ? 'bg-gray-100' : '',
                                    'block px-4 py-2 text-xs text-gray-700',
                                  )}
                                >
                                  <div className="inline-flex items-center">
                                    {language.flag}
                                    {language.name}
                                  </div>
                                </Link>
                              )}
                            </Menu.Item>
                          ))}
                        </Menu.Items>
                      </Transition>
                    </Menu>
                  </div>
                  <div className="hidden md:ml-4 md:flex md:shrink-0 md:items-center">
                    {/* User menu dropdown */}
                    <Menu as="div" className="relative">
                      <div>
                        <Menu.Button className="flex rounded-full bg-white text-sm text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2">
                          <span className="sr-only">Open menu</span>
                          <UserIcon className="h-6 w-6" aria-hidden="true" />
                        </Menu.Button>
                      </div>
                      <Transition
                        as={Fragment}
                        enter="transition ease-out duration-200"
                        enterFrom="transform opacity-0 scale-95"
                        enterTo="transform opacity-100 scale-100"
                        leave="transition ease-in duration-75"
                        leaveFrom="transform opacity-100 scale-100"
                        leaveTo="transform opacity-0 scale-95"
                      >
                        <Menu.Items className="absolute right-0 z-10 mt-2 w-48 origin-top-right rounded-md bg-white py-1 shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
                          {header.userMenus.map((userMenu) => (
                            <Fragment key={userMenu.title}>
                              <Menu.Item>
                                <div className="relative my-2">
                                  <div
                                    className="absolute inset-0 flex items-center"
                                    aria-hidden="true"
                                  >
                                    <div className="w-full border-t border-gray-300" />
                                  </div>
                                  <div className="relative flex pl-2">
                                    <span className="rounded-full bg-gray-100 px-2 text-sm  text-gray-500 ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
                                      {userMenu.title}
                                    </span>
                                  </div>
                                </div>
                              </Menu.Item>
                              {userMenu.menus.map((menu) => (
                                <Menu.Item key={menu.name}>
                                  {({ active }) => (
                                    <Link
                                      href={menu.url}
                                      className={classNames(
                                        active ? 'bg-gray-100' : '',
                                        'block px-4 py-2 text-sm text-gray-500',
                                      )}
                                    >
                                      {menu.name}
                                    </Link>
                                  )}
                                </Menu.Item>
                              ))}
                            </Fragment>
                          ))}
                        </Menu.Items>
                      </Transition>
                    </Menu>
                  </div>
                  <div className="-mr-2 ml-2 flex items-center md:hidden">
                    {/* Mobile menu button */}
                    <Disclosure.Button className="inline-flex items-center justify-center rounded-md p-2 text-gray-500 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-inset focus:ring-blue-500">
                      <span className="sr-only">Open main menu</span>
                      {open ? (
                        <XMarkIcon className="block h-6 w-6" aria-hidden="true" />
                      ) : (
                        <Bars3Icon className="block h-6 w-6" aria-hidden="true" />
                      )}
                    </Disclosure.Button>
                  </div>
                </div>
              </div>
            </div>

            <Disclosure.Panel className="md:hidden">
              <div className="space-y-1 pb-3 pt-2">
                {header.naviMenus.map((menu) => (
                  <Disclosure.Button
                    key={menu.name}
                    as="a"
                    href={menu.url}
                    className="block border-l-4 border-blue-500 bg-blue-50 py-2 pl-3 pr-4 text-base font-medium text-blue-700 sm:pl-5 sm:pr-6"
                  >
                    {menu.name}
                  </Disclosure.Button>
                ))}
              </div>
              <div className="border-t border-gray-200 pb-3 pt-4">
                <div className="flex items-center px-4 sm:px-6">
                  <div className="ml-3">
                    <div className="text-base font-medium text-gray-900">Tom Cook</div>
                    <div className="text-sm font-medium text-gray-500">tom@example.com</div>
                  </div>
                  <button
                    type="button"
                    className="ml-auto shrink-0 rounded-full bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                  >
                    <span className="sr-only">View notifications</span>
                    <UserCircleIcon className="h-10 w-10" aria-hidden="true" />
                  </button>
                </div>
                <div className="mt-3 space-y-1">
                  {header.userMenus.map((userMenu) => (
                    <Fragment key={userMenu.title}>
                      <div className="relative my-2">
                        <div className="absolute inset-0 flex items-center" aria-hidden="true">
                          <div className="w-full border-t border-gray-300" />
                        </div>
                        <div className="relative flex pl-2">
                          <span className="rounded-full bg-gray-100 px-2 text-sm  text-gray-500 ring-1 ring-inset ring-gray-300 hover:bg-gray-50">
                            {userMenu.title}
                          </span>
                        </div>
                      </div>
                      {userMenu.menus.map((menu) => (
                        <Disclosure.Button
                          key={menu.name}
                          as="a"
                          href={menu.url}
                          className="block px-4 py-2 text-base font-medium text-gray-500 hover:bg-gray-100 hover:text-gray-900 sm:px-6"
                        >
                          {menu.name}
                        </Disclosure.Button>
                      ))}
                    </Fragment>
                  ))}
                </div>
              </div>
            </Disclosure.Panel>
          </>
        )}
      </Disclosure>
    </header>
  );
}
