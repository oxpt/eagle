import Image from "next/image";
import Link from "next/link";

const costom = {
  error_code: "500",
  description: "内部サーバーエラーが発生しました。",
  src: "/No02.png",
  back_to_top: "トップページへ戻る",
};

export default function Custom400() {
  return (
    <div className="text-center">
      <Image
        src={costom.src}
        width={865}
        height={882}
        alt="Error"
        priority={false}
        className="mx-auto mb-[5svh] mt-[10svh] h-[30svh] w-auto items-center"
      />
      <h1 className="mb-4 text-4xl font-bold leading-none tracking-tight text-gray-900 md:text-5xl lg:text-6xl">
        {costom.error_code}
      </h1>
      <p className="mb-8 text-lg font-normal text-gray-500 sm:px-16 lg:px-48 lg:text-xl">
        {costom.description}
      </p>
      <Link
        href="/"
        className="mb-[10svh] inline-flex max-w-sm justify-center rounded-md bg-blue-500 px-3 py-2 text-sm text-white shadow-sm hover:bg-blue-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-blue-500 sm:col-start-2"
      >
        {costom.back_to_top}
      </Link>
    </div>
  );
}
