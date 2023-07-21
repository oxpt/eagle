import { Disclosure, Menu, Transition } from "@headlessui/react";
import { Bars3Icon, UserIcon, XMarkIcon } from "@heroicons/react/24/outline";
import Image from "next/image";
import Link from "next/link";
import { Fragment } from "react";
import "flag-icons";

function classNames(...classes: string[]) {
  return classes.filter(Boolean).join(" ");
}

const header = {
  name: "XEE.JP",
  url: "http://localhost:3000",
  logo: "/logo_right.png",
  menu: [
    { name: "home", url: "/visitors/home" },
    { name: "about", url: "/visitors/about" },
    { name: "service", url: "/visitors/service" },
    { name: "pricing", url: "/visitors/pricing" },
    { name: "contact", url: "/visitors/contact" },
  ],
  languageDefault: {
    name: "日本語",
    url: "./jp",
    flag: <span className="fi fi-jp mr-2 h-5 w-5" aria-hidden="true" />,
  },
  languages: [
    {
      name: "日本語",
      url: "./jp",
      flag: <span className="fi fi-jp mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: "English (US)",
      url: "./en_us",
      flag: <span className="fi fi-us mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: "Español",
      url: "./es",
      flag: <span className="fi fi-es mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
    {
      name: "中文 (繁體)",
      url: "./cn",
      flag: <span className="fi fi-cn mr-2 h-3.5 w-3.5" aria-hidden="true" />,
    },
  ],
  naviMenus: [
    { name: "概要", url: "/visitors/about" },
    { name: "使い方", url: "/visitors/docs" },
    { name: "ゲーム一覧", url: "/visitors/games" },
    { name: "FAQ", url: "/visitors/faq" },
  ],
  userMenus: [
    {
      title: "ゲストメニュー",
      menus: [{ name: "ルーム一覧", url: "/subject/room_list" }],
    },
    {
      title: "ホストメニュー",
      menus: [
        { name: "ルーム開設", url: "/conductor/room_create" },
        { name: "ルーム管理", url: "/conductor/room_management" },
      ],
    },
    {
      title: "ユーザーメニュー",
      menus: [
        { name: "アカウント管理", url: "/conductor/account" },
        { name: "お問合せ", url: "/visitors/contact" },
      ],
    },
  ],
};

export default function Header() {
  return (
    <header>
      <Disclosure
        as="nav"
        className="mx-auto grid h-16 grid-cols-12 items-center gap-3 bg-white"
      >
        {({ open }) => (
          <>
            <Link
              href="/"
              className="col-span-6 flex  items-center md:col-span-4"
            >
              <Image
                src={header.logo}
                width={90}
                height={90}
                alt="XEE.JP"
                className="h-8 w-auto"
              />
              <span className="prose prose-2xl ml-2 font-extrabold text-blue-700">
                {header.name}
              </span>
            </Link>
            <div className="hidden items-center justify-between md:col-span-4 md:flex">
              {header.naviMenus.map((menu) => (
                <Link
                  key={menu.name}
                  href={menu.url}
                  className="prose lg:prose-sm"
                >
                  {menu.name}
                </Link>
              ))}
            </div>
            <div className="col-span-6 flex items-center justify-end md:col-span-4">
              <Menu as="div" className="relative mr-4 items-center">
                <Menu.Button className="flex w-32  justify-center rounded-md bg-gray-100 px-3 py-2 text-sm text-gray-500 hover:bg-blue-500 hover:text-white focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500">
                  <span className="sr-only">Language select menu</span>
                  {header.languageDefault.flag}
                  {header.languageDefault.name}
                </Menu.Button>
                <Transition
                  as={Fragment}
                  enter="transition ease-out duration-200"
                  enterFrom="transform opacity-0 scale-95"
                  enterTo="transform opacity-100 scale-100"
                  leave="transition ease-in duration-75"
                  leaveFrom="transform opacity-100 scale-100"
                  leaveTo="transform opacity-0 scale-95"
                >
                  <Menu.Items className="absolute right-0 w-44 origin-top-right rounded-md bg-white py-1 ring-1 ring-gray-100 focus:outline-none">
                    {header.languages.map((language) => (
                      <Menu.Item key={language.name}>
                        {({ active }) => (
                          <Link
                            href={language.url}
                            className={classNames(
                              active ? "bg-gray-100" : "",
                              "block px-4 py-2 text-xs text-gray-700"
                            )}
                          >
                            <div className="flex items-center">
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
              <div>
                <div className="mr-2 hidden shrink-0 items-center sm:flex">
                  {/* Desktop menu dropdown */}
                  <Menu as="div" className="relative">
                    <Menu.Button className="flex rounded-full bg-gray-100  p-0.5 text-sm text-gray-500 ring-4 ring-gray-100  hover:bg-blue-500 hover:text-white hover:ring-4 hover:ring-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500">
                      <span className="sr-only">Open menu</span>
                      <UserIcon className="h-6 w-6" aria-hidden="true" />
                    </Menu.Button>
                    <Transition
                      as={Fragment}
                      enter="transition ease-out duration-200"
                      enterFrom="transform opacity-0 scale-95"
                      enterTo="transform opacity-100 scale-100"
                      leave="transition ease-in duration-75"
                      leaveFrom="transform opacity-100 scale-100"
                      leaveTo="transform opacity-0 scale-95"
                    >
                      <Menu.Items className="absolute right-0 mt-2 w-48 origin-top-right rounded-md bg-white py-1 ring-1 ring-black ring-opacity-5 focus:outline-none">
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
                                      active ? "bg-gray-100" : "",
                                      "block px-4 py-2 text-sm text-gray-500"
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
                <div className="flex shrink-0 items-center sm:hidden">
                  {/* Mobile menu button */}
                  <Menu as="div" className="relative z-10">
                    <Menu.Button
                      className="flex rounded-md bg-gray-100 p-1.5 text-sm text-gray-500 hover:bg-gray-100 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-blue-500"
                      aria-hidden="true"
                    >
                      <span className="sr-only">Open menu</span>
                      {open ? (
                        <XMarkIcon className="h-6 w-6" aria-hidden="true" />
                      ) : (
                        <Bars3Icon className="h-6 w-6" aria-hidden="true" />
                      )}
                    </Menu.Button>
                    <Transition
                      as={Fragment}
                      enter="transition ease-out duration-200"
                      enterFrom="transform opacity-0 scale-95"
                      enterTo="transform opacity-100 scale-100"
                      leave="transition ease-in duration-75"
                      leaveFrom="transform opacity-100 scale-100"
                      leaveTo="transform opacity-0 scale-95"
                    >
                      <Menu.Items className="absolute right-0 w-screen bg-white">
                        <div className="space-y-1 pb-3 pt-2">
                          {header.naviMenus.map((menu) => (
                            <Menu.Item key={menu.name}>
                              {({ active }) => (
                                <Link
                                  href={menu.url}
                                  className={classNames(
                                    active ? "bg-gray-100" : "",
                                    "block border-l-4 border-blue-500 bg-blue-50 py-2 pl-3 pr-4 text-base font-medium text-blue-700 sm:pl-5 sm:pr-6"
                                  )}
                                >
                                  {menu.name}
                                </Link>
                              )}
                            </Menu.Item>
                          ))}
                        </div>
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
                                      active ? "bg-gray-100" : "",
                                      "block px-4 py-2 text-sm text-gray-500"
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
              </div>
            </div>
          </>
        )}
      </Disclosure>
    </header>
  );
}
